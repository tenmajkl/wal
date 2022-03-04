// lisp inspired language
// very good very nice

use std::{env, fs, path::Path, process::exit, collections::HashMap};

#[derive(Debug, Clone)]
#[derive(PartialEq)]
enum TokenKind {
    FUNCTION,
    STRING,
    INT,
    VOID,
    BOOL,
    VARIABLE
}

#[derive(Debug, Clone)]
struct Token {
    kind: TokenKind,
    context: String,
    body: Vec<Token>
}

#[derive(Clone)]
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
    let mut parsing_comment = false;
    for character in program.chars() {
        pos += 1;
        if parsing_comment {
            if character == '\n' {
                parsing_comment = false;
            }
            continue;
        }
        match character {
            '[' => {
                if parsing_string {
                    last.push('[');
                } else {
                    result.push(Word { content: String::from("["), pos, line });
                }
            },
            ']' => {
                if parsing_string {
                    last.push(']');
                } else {
                    if !last.is_empty() {
                        result.push(Word { content: last.clone(), pos, line });
                    }
                    result.push(Word { content: String::from("]"), pos, line });
                    last.clear();
                }
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
            ';' => parsing_comment = true,
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
    for index in 0..program.len() {
        let word = program[index].clone();
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
                    curent.body.push(Token { kind: TokenKind::STRING, context: word.content[1..(word.content.len()-1)].to_string(), body: Vec::new() }) // TODO String trim
                } else if is_numeric(&word.content) {
                    curent.body.push(Token { kind: TokenKind::INT, context: word.content, body: Vec::new() })
                } else if curent.context.is_empty() {
                    curent.context = word.content;
                } else if word.content == "]" {
                    tokens.push(curent.clone());
                    parsing_fn = false;
                } else {
                    if program[index - 1].content == "$" {
                        curent.body.push(Token { kind: TokenKind::VARIABLE, context: word.content, body: Vec::new() });
                    } else {
                        error("Unknown word", word.line, word.pos);
                    }
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

struct Interpreter {
    variables: HashMap<String, Token>,
}

impl Interpreter {

    pub fn new() -> Interpreter {
        Interpreter {
            variables: HashMap::new()
        }
    }

    fn parse_token(&mut self, token: Token) -> Token {
        return match token.kind {
            TokenKind::FUNCTION => 
                match token.context.as_str() {
                    "->" => {
                        for child in token.body.clone() {
                            if child.kind != TokenKind::VOID {
                                println!("{}", self.parse_token(child).context);
                            }
                        }
                        return Token { kind: TokenKind::VOID, body: Vec::new(), context: String::new() };
                    },
                    "<-" => {
                        if !token.body.is_empty() {
                            print!("{}", &self.parse_token(token.body[0].clone()).context);
                        }
                        let mut input: String = String::new();
                        std::io::stdin().read_line(&mut input).unwrap();
                        return Token { kind: TokenKind::STRING, context: input.trim().to_string(), body: Vec::new()};
                    },
                    "+" => {
                        let mut result: isize = 0;
                        for child in token.body.clone() {
                            let token = self.parse_token(child);
                            if token.kind == TokenKind::INT {
                                result += token.context.parse::<isize>().unwrap();
                            } else {
                                error("Function + takes only integer as argument", 0, 0);
                            }
                        }
                        return Token { kind: TokenKind::INT, context: format!("{}", result), body: Vec::new() };
                    },
                    "-" => {
                        let mut result: isize = 0;
                        for child in token.body.clone() {
                            let parsed = self.parse_token(child);
                            if parsed.kind == TokenKind::INT {
                                result -= parsed.context.parse::<isize>().unwrap();
                            } else {
                                error("Function - takes only integer as argument", 0, 0);
                            }
                        }
                        return Token { kind: TokenKind::INT, context: format!("{}", result), body: Vec::new() };
                    },
                    "==" => {
                        if token.body.len() < 2 {
                            error("Function == takes at least 2 arguments", 0, 0);
                        }
                        let mut result: bool = true;
                        let mut last: Token = self.parse_token(token.body[0].clone());
                        for index in 1..token.body.len() {
                            let parsed: Token = self.parse_token(token.body[index].clone());
                            if last.kind != parsed.kind {
                                result = false;
                                break;
                            }
                    
                            if last.context != parsed.context {
                                result = false;
                                break;
                            }

                            last = parsed;
                        }
                        return Token { kind: TokenKind::BOOL, context: format!("{}", result), body: Vec::new() }
                    },
                    "=<" => {
                        if token.body.len() < 3 {
                            error("Function if takes at least 3 arguments", 0, 0);
                        }
                        let condition: Token = self.parse_token(token.body[0].clone());
                        if condition.context == "false" {
                            return self.parse_token(token.body[2].clone());
                        } else {
                            return self.parse_token(token.body[1].clone());
                        }
                    },
                    "$" => {
                        if token.body.len() == 1 {
                            if self.variables.contains_key(&token.body[0].context) {
                                return self.variables.get(&token.body[0].context).unwrap().clone();
                            } else {
                                error("Undefined variable", 0, 0);
                            }
                        } else if token.body.len() == 2 {
                            let value = self.parse_token(token.body[1].clone());
                            self.variables.insert(token.body[0].context.clone(), value.clone());
                            return value;
                        } else {
                            error("Function $ takes at least 1 argument", 0, 0);
                        }
                    }
                    _ => error("Undefined function", 0, 0) // TODO position
                },
            TokenKind::INT|TokenKind::STRING|TokenKind::VOID|TokenKind::BOOL|TokenKind::VARIABLE => token,
        }
    }

    fn parse(&mut self, program: Vec<Token>) {
        for token in program {
            self.parse_token(token);
        }
    }
}

fn main() {

    let mut args = env::args();
    let filename = args.nth(1).unwrap();
    if !Path::new(&filename).exists() {
        error(&format!("File {} not found", filename), 0, 0);
    }

    let code = fs::read_to_string(filename)
        .expect("File is not readable");

    let mut interpreter: Interpreter = Interpreter::new();
    interpreter.parse(tokenize(lex(&code)));
}
