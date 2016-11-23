#[derive(Debug, PartialEq)]
pub enum Token {
    Whitespace,
    Comment,
    Word(String),
    OpenCurlyBracket,
    CloseCurlyBracket,
    OpenRoundBracket,
    CloseRoundBracket,
}


pub fn lex(data: &[u8]) -> Result<Vec<Token>, String> {
    let mut byte_iter = data.iter();
    let mut last_token = Token::Whitespace;
    let mut tokens = Vec::new();

    while let Some(byte) = byte_iter.next() {
        if last_token == Token::Comment {
            if *byte == b'\n' {
                tokens.push(last_token);
                last_token = Token::Whitespace;
            }

            continue;
        }

        match *byte {
            b';' => {
                tokens.push(last_token);
                last_token = Token::Comment;
            }

            b'\n' | b'\r' => {
                if last_token == Token::Comment {
                    tokens.push(last_token);
                    last_token = Token::Whitespace;
                }
            }

            b' ' | b'\t' => {
                if last_token != Token::Whitespace {
                    tokens.push(last_token);
                    last_token = Token::Whitespace;
                }
            }

            b'{' => {
                tokens.push(last_token);
                last_token = Token::OpenCurlyBracket;
            }

            b'}' => {
                tokens.push(last_token);
                last_token = Token::CloseCurlyBracket;
                }

            b'(' => {
                tokens.push(last_token);
                last_token = Token::OpenRoundBracket;
             }

            b')' => {
                tokens.push(last_token);
                last_token = Token::CloseRoundBracket;
            }

            _ => {
                let c = *byte as char;

                if let Token::Word(mut string) = last_token {
                    string.push(c);
                    last_token = Token::Word(string);
                } else {
                    tokens.push(last_token);
                    last_token = Token::Word(c.to_string());
                }
            }
        }
    }

    tokens.push(last_token);

    Ok(tokens)
}

/*

        #[derive(Debug)]
        struct Definition {
            name: String,
            params: Vec<String>,
            statements: Vec<Statement>,
        }

        #[derive(Debug)]
        struct Statement {
            name: String,
            params: Vec<String>,
            substatements: Vec<Statement>,
        }

        let mut definitions = Vec::new();
        let mut tokens_iter = tokens.into_iter();

        fn next_ignoring_whitespace<I: Iterator<Item=Token>>(tokens_iter: &mut I) -> Option<Token> {
            loop {
                let token = tokens_iter.next();

                match token {
                    Some(Token::Whitespace) | Some(Token::Comment) => {},
                    Some(t) => return Some(t),
                    None => return None,
                }
            }
        }

        fn walk_statement<I: Iterator<Item=Token>>(tokens_iter: &mut I) -> Result<Option<Statement>, String> {
            // Name or close bracket
            let name = match next_ignoring_whitespace(tokens_iter) {
                Some(Token::Word(string)) => string,
                Some(Token::CloseCurlyBracket) => return Ok(None),
                Some(t) => return Err(format!("Unexpected token: {:?} (expected Token::Word or Token::CloseCurlyBracket)", t)),
                None => return Err("Unexpected end of input".to_string()),
            };

            // Parameter list / substatements
            let mut params = Vec::new();
            let mut substatements = Vec::new();

            match next_ignoring_whitespace(tokens_iter) {
                Some(Token::OpenRoundBracket) => {
                    // Parameter list
                    loop {
                        match next_ignoring_whitespace(tokens_iter) {
                            Some(Token::CloseRoundBracket) => break,
                            Some(Token::Word(string)) => params.push(string),
                            Some(t) => return Err(format!("Unexpected token: {:?} (expected Token::CloseRoundBracket)", t)),
                            None => return Err("Unexpected end of input".to_string()),
                        }
                    }
                }
                Some(Token::OpenCurlyBracket) => {
                    // Substatements
                    while let Some(statement) = try!(walk_statement(tokens_iter)) {
                        substatements.push(statement);
                    }
                }
                Some(t) => return Err(format!("Unexpected token: {:?} (expected Token::OpenRoundBracket)", t)),
                None => return Err("Unexpected end of input".to_string()),
            }

            Ok(Some(Statement {
                name: name,
                params: params,
                substatements: substatements,
            }))
        }

        fn walk_definition<I: Iterator<Item=Token>>(tokens_iter: &mut I) -> Result<Option<Definition>, String> {
            // Name
            let name = match next_ignoring_whitespace(tokens_iter) {
                Some(Token::Word(string)) => string,
                Some(t) => return Err(format!("Unexpected token: {:?} (expected Token::Word)", t)),
                None => return Ok(None),
            };

            // Parameter list
            match next_ignoring_whitespace(tokens_iter) {
                Some(Token::OpenRoundBracket) => {},
                Some(t) => return Err(format!("Unexpected token: {:?} (expected Token::OpenRoundBracket)", t)),
                None => return Err("Unexpected end of input".to_string()),
            }

            let mut params = Vec::new();
            loop {
                match next_ignoring_whitespace(tokens_iter) {
                    Some(Token::CloseRoundBracket) => break,
                    Some(Token::Word(string)) => params.push(string),
                    Some(t) => return Err(format!("Unexpected token: {:?} (expected Token::CloseRoundBracket)", t)),
                    None => return Err("Unexpected end of input".to_string()),
                }
            }

            // Body
            match next_ignoring_whitespace(tokens_iter) {
                Some(Token::OpenCurlyBracket) => {},
                Some(t) => return Err(format!("Unexpected token: {:?} (expected Token::OpenCurlyBracket)", t)),
                None => return Err("Unexpected end of input".to_string()),
            }

            let mut statements = Vec::new();
            while let Some(statement) = try!(walk_statement(tokens_iter)) {
                println!("{:?}", statement);
                statements.push(statement);
            }

            println!("{:?}", next_ignoring_whitespace(tokens_iter));

            Ok(None)
        }

        while let Some(definition) = try!(walk_definition(&mut tokens_iter)) {
            definitions.push(definition);
        }

        println!("{:?}", definitions);

        // Parse

        print!("\n");
        Err("foo".to_string())
    }

    pub fn open(data: Box<[u8]>) -> Result<TextFile, String> {
        TextFile::parse(&data)
    }
}
*/