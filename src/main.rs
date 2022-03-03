// lisp inspired language
// very good very nice

use std::process::exit;

#[derive(Debug, Clone)]
#[derive(PartialEq)]
enum TokenKind {
    FUNCTION,
    STRING,
    INT,
    VOID
}

#[derive(Debug, Clone)]
struct Token {
    kind: TokenKind,
    context: String,
    body: Vec<Token>
}

#[derive(Debug, Clone)]
struct Word {
    content: String,
    pos: usize,
    line: usize
}

fn error(message: &str, line: usize, pos: usize) -> ! {
    println!("[ERROR] {} at line {}, col {}.", message, line, pos);
    exit(-1);
}

fn is_numeric(target: &str) -> bool {
    for character in target.chars() {
        if character < '0' || character > '9' {
            return false;
        }
    }
    return true;
}

fn lex(program: &String) -> Vec<Word> {
    let mut result: Vec<Word> = Vec::new();
    let mut last = String::new();
    let mut line = 1;
    let mut pos = 0;
    let mut parsing_string = false; 
    for character in program.chars() {
        pos += 1;
        match character {
            '[' => result.push(Word { content: String::from("["), pos, line }),
            ']' => {
                if !last.is_empty() {
                    result.push(Word { content: last.clone(), pos, line });
                }
                result.push(Word { content: String::from("]"), pos, line });
                last.clear();
            },
            ' ' =>  {
                if parsing_string {
                    last.push(' ');
                } else if !last.is_empty() {
                    result.push(Word { content: last.clone(), pos, line });
                    last.clear();
                }
            },
            '\n' => {
                pos = 0;
                line += 1;
            },
            '\'' => {
                last.push('\'');
                parsing_string = !parsing_string;
            },
            _ => last.push(character)
        }
    }
    return result;
}


fn tokenize(program: Vec<Word>) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut parsing_fn: bool = false;
    let mut rec_function: Vec<Word> = Vec::new();
    let mut curent: Token = Token { kind: TokenKind::VOID, context: String::new(), body: Vec::new() };
    let mut nested: isize = 0;
    for word in program {
        if word.content == "[" {
            if parsing_fn {
                nested += 1;
                rec_function.push(word.clone());
            } else {
                parsing_fn = true;
                curent = Token { kind: TokenKind::FUNCTION, context: String::new(), body: Vec::new() }
            }
        } else if parsing_fn {
            if rec_function.is_empty() {
                if word.content.starts_with('\'') {
                    curent.body.push(Token { kind: TokenKind::STRING, context: word.content, body: Vec::new() }) // TODO String trim
                } else if is_numeric(&word.content) {
                    curent.body.push(Token { kind: TokenKind::INT, context: word.content, body: Vec::new() })
                } else if curent.context.is_empty() {
                    curent.context = word.content;
                } else if word.content == "]" {
                    tokens.push(curent.clone());
                    parsing_fn = false;
                } else {
                    error("Unknown word", word.line, word.pos);
                }
            } else {
                rec_function.push(word.clone());
                if word.content == "]"{
                    nested -= 1;
                    if nested == 0 {
                        curent.body.push(tokenize(rec_function)[0].clone());
                        rec_function = Vec::new();
                    }
                }
            }
        } else {
            error("Unexpected token", word.line, word.pos);
        }
    }

    return tokens;
}

fn parse_token(token: Token) -> Token {
    return match token.kind {
        TokenKind::FUNCTION => 
            match token.context.as_str() {
                "echo" => {
                    for child in token.body.clone() {
                        if child.kind != TokenKind::VOID {
                            println!("{}", parse_token(child).context);
                        }
                    }
                    return Token { kind: TokenKind::VOID, body: Vec::new(), context: String::new() };
                },
                "+" => {
                    let mut result: isize = 0;
                    for child in token.body.clone() {
                        result += match child.kind {
                            TokenKind::INT => child.context.parse::<isize>().unwrap(),
                            TokenKind::FUNCTION => parse_token(child).context.parse::<isize>().unwrap(), 
                            _ => error("Function + takes only integer arguments", 0, 0)
                        }
                    }
                    return Token { kind: TokenKind::INT, context: format!("{}", result), body: Vec::new() };
                },
                "-" => {
                    let mut result: isize = 0;
                    for child in token.body.clone() {
                        result -= match child.kind {
                            TokenKind::INT => child.context.parse::<isize>().unwrap(),
                            TokenKind::FUNCTION => parse_token(child).context.parse::<isize>().unwrap(), 
                            _ => error("Function + takes only integer arguments", 0, 0)
                        }
                    }
                    return Token { kind: TokenKind::INT, context: format!("{}", result), body: Vec::new() };
                },
                _ => error("Undefined function", 0, 0) // TODO position
            },
        TokenKind::INT|TokenKind::STRING|TokenKind::VOID => token,
    }
}

fn parse(program: Vec<Token>) {
    for token in program {
        parse_token(token);
    }
}

fn main() {
    parse(tokenize(lex(&String::from("[echo [- 1]]"))));
}
