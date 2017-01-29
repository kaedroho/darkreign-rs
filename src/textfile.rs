#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Word(String),
    CurlyBlock(Vec<Token>),
    RoundBlock(Vec<Token>),
}


#[derive(Debug, PartialEq)]
enum BlockType {
    Module,
    CurlyBlock,
    RoundBlock,
}


fn lex_block(block_type: BlockType, byte_iter: &mut ::std::slice::Iter<u8>) -> Result<Vec<Token>, String> {
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

                tokens.push(Token::CurlyBlock(try!(lex_block(BlockType::CurlyBlock, byte_iter))));
            }

            b'}' => {
                if !word.is_empty() {
                    tokens.push(Token::Word(word.clone()));
                    word.clear()
                };

                if block_type == BlockType::CurlyBlock {
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

                tokens.push(Token::RoundBlock(try!(lex_block(BlockType::RoundBlock, byte_iter))));
            }

            b')' => {
                if !word.is_empty() {
                    tokens.push(Token::Word(word.clone()));
                    word.clear()
                };

                if block_type == BlockType::RoundBlock {
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

    lex_block(BlockType::Module, &mut byte_iter)
}
