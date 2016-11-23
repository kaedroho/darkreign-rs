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
