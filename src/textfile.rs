#[derive(Debug, PartialEq, Clone)]
pub enum BlockDelimiter {
    Brace,
    Parentheses,
}


#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Word(String),
    Block(BlockDelimiter, Vec<Token>),
}


fn lex_block(delimiter: Option<BlockDelimiter>, byte_iter: &mut ::std::slice::Iter<u8>) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut word = String::new();
    let mut in_comment = false;

    while let Some(byte) = byte_iter.next() {
        if in_comment {
            if *byte == b'\n' {
                in_comment = false;
            }

            continue;
        }

        match *byte {
            b';' => {
                if !word.is_empty() {
                    tokens.push(Token::Word(word.clone()));
                    word.clear()
                }

                in_comment = true;
            }

            b'\n' | b'\r' | b' ' | b'\t' => {
                if !word.is_empty() {
                    tokens.push(Token::Word(word.clone()));
                    word.clear()
                }
            }

            b'{' => {
                if !word.is_empty() {
                    tokens.push(Token::Word(word.clone()));
                    word.clear()
                };

                tokens.push(Token::Block(BlockDelimiter::Brace, try!(lex_block(Some(BlockDelimiter::Brace), byte_iter))));
            }

            b'}' => {
                if !word.is_empty() {
                    tokens.push(Token::Word(word.clone()));
                    word.clear()
                };

                if delimiter == Some(BlockDelimiter::Brace) {
                    return Ok(tokens);
                } else {
                    return Err("Unexpected  char: '}'".to_string());
                }
            }

            b'(' => {
                if !word.is_empty() {
                    tokens.push(Token::Word(word.clone()));
                    word.clear()
                };

                tokens.push(Token::Block(BlockDelimiter::Parentheses, try!(lex_block(Some(BlockDelimiter::Parentheses), byte_iter))));
            }

            b')' => {
                if !word.is_empty() {
                    tokens.push(Token::Word(word.clone()));
                    word.clear()
                };

                if delimiter == Some(BlockDelimiter::Parentheses) {
                    return Ok(tokens);
                } else {
                    return Err("Unexpected  char: ')'".to_string());
                }
            }

            _ => {
                let c = *byte as char;
                word.push(c);
            }
        }
    }

    if !word.is_empty() {
        tokens.push(Token::Word(word.clone()));
        word.clear()
    };

    Ok(tokens)
}


pub fn lex(data: &[u8]) -> Result<Vec<Token>, String> {
    let mut byte_iter = data.iter();

    lex_block(None, &mut byte_iter)
}
