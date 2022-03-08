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
    WORD,
    ARRAY,
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
            '#' => parsing_comment = true,
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
                    curent.body.push(Token { kind: TokenKind::WORD, context: word.content, body: Vec::new() });
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
//    functions: HashMap<String, Token>,
}

impl Interpreter {

    pub fn new() -> Interpreter {
        Interpreter {
            variables: HashMap::new(),
//            functions: HashMap::new(),
        }
    }

    fn to_number(&mut self, body: &Vec<Token>, function: &str) -> Vec<isize> {
        let mut numbers: Vec<isize> = Vec::new();
        for child in body {
            let parsed = self.parse_token(child.clone());
            if parsed.kind == TokenKind::INT {
                numbers.push(parsed.context.parse::<isize>().unwrap());
            } else {
                error(&format!("Function {} takes only integer as argument", function), 0, 0);
            }
        }

        return numbers;
    }

    fn parse_token(&mut self, token: Token) -> Token {
        return match token.kind {
            TokenKind::FUNCTION => 
                match token.context.as_str() {
                    "->" => {
                        for child in token.body.clone() {
                            let child = self.parse_token(child);
                            if child.kind == TokenKind::ARRAY {
                                print!("Array: ");
                                for item in child.body {
                                    print!("{} ", item.context);
                                }
                                println!("");
                            } else if child.kind == TokenKind::WORD {
                                error("Unexpected word", 0, 0);
                            } else if child.kind != TokenKind::VOID {
                                println!("{}", child.context);
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
                        let numbers = self.to_number(&token.body, "+");
                        let mut result: isize = numbers[0];
                        for number in numbers[1..].into_iter() {
                            result += number;
                        }
                        return Token { kind: TokenKind::INT, context: format!("{}", result), body: Vec::new() };
                    },
                    "-" => {
                        let numbers = self.to_number(&token.body, "-");
                        let mut result: isize = numbers[0];
                        for number in numbers[1..].into_iter() {
                            result -= number;
                        }
                        return Token { kind: TokenKind::INT, context: format!("{}", result), body: Vec::new() };
                    },
                    "*" => {
                        let numbers = self.to_number(&token.body, "*");
                        let mut result: isize = numbers[0];
                        for number in numbers[1..].into_iter() {
                            result *= number;
                        }
                        return Token { kind: TokenKind::INT, context: format!("{}", result), body: Vec::new() };                    },
                    "/" => {
                        let numbers = self.to_number(&token.body, "/");
                        let mut result: isize = numbers[0];
                        for number in numbers[1..].into_iter() {
                            result /= number;
                        }
                        return Token { kind: TokenKind::INT, context: format!("{}", result), body: Vec::new() };
                    },
                    "=" => {
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
                    "<" => {
                        if token.body.len() != 2 {
                            error("Function < takes exactly 2 arguments", 0, 0);
                        }

                        let numbers = self.to_number(&token.body, "<");
                        let result = numbers[0] < numbers[1];

                        return Token { kind: TokenKind::BOOL, context: format!("{}", result), body: Vec::new() }
                    },
                    ">" => {
                        if token.body.len() != 2 {
                            error("Function > takes exactly 2 arguments", 0, 0);
                        }

                        let numbers = self.to_number(&token.body, ">");
                        let result = numbers[0] > numbers[1];

                        return Token { kind: TokenKind::BOOL, context: format!("{}", result), body: Vec::new() }                    
                    },
                    ">=" => {
                        if token.body.len() != 2 {
                            error("Function >= takes exactly 2 arguments", 0, 0);
                        }

                        let numbers = self.to_number(&token.body, ">=");
                        let result = numbers[0] >= numbers[1];

                        return Token { kind: TokenKind::BOOL, context: format!("{}", result), body: Vec::new() }                    
                    },
                    "<=" => {
                        if token.body.len() != 2 {
                            error("Function <= takes exactly 2 arguments", 0, 0);
                        }

                        let numbers = self.to_number(&token.body, "<=");
                        let result = numbers[0] <= numbers[1];

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
                    },
                    "@" => {
                        return Token { kind: TokenKind::ARRAY, context: "Array".to_string(), body: token.body};
                    },
                    "@>" => {
                        let mut result: Token = self.parse_token(token.body[0].clone());
                        // TODO errors
                        for item in token.body[1..].into_iter() {
                            result.body.push(item.clone());
                        }
                        return result;
                    },
                    "@$" => {
                        if token.body.len() < 2 {
                            error("Function @$ takes at least 2 arguments", 0, 0)
                        }
                        
                        let index = self.parse_token(token.body[1].clone());
                        if index.kind != TokenKind::INT {
                            error("Array can be indexed only with integer", 0, 0);
                        }
                        let mut array: Vec<Token> = self.parse_token(token.body[0].clone()).body;
                        let index: usize = index.context.parse::<usize>().unwrap();
                        
                        if token.body.len() == 2 {
                            if index > array.len() - 1 {
                                error(&format!("Cannot index to position {}, because size of array is {}", index, array.len()), 0, 0)
                            }
                            return self.parse_token(array[index].clone());
                        } else {
                            array[index] = self.parse_token(token.body[2].clone());
                            return Token { kind: TokenKind::ARRAY, context: "Array".to_string(), body: array }; 
                        }
                    },
                    "$_" => {
                        if token.body.len() != 1 {
                            error("Function $_ takes exactly 1 argument!", 0, 0);
                        }
                        let variable = self.parse_token(token.body[0].clone()).context;
                        let value = env::var(variable).unwrap();
                        return Token { kind: TokenKind::STRING, context: value, body: Vec::new() };
                    },
                    "<>" => { 
                        if token.body.len() < 3 {
                            error("Function <> takes at least 3 arguments!", 0, 0);
                        }
                        
                        let iterator = self.parse_token(token.body[1].clone());
                     
                        if iterator.kind != TokenKind::ARRAY {
                            error("Argument 2 in function <> must be array!", 0, 0);
                        }

                        let variable = token.body[0].context.clone();

                        for item in iterator.body.clone() {
                            let item = self.parse_token(item);
                            self.variables.insert(variable.clone(), item);
                            for statement in token.body[2..].into_iter() {
                                self.parse_token(statement.clone());
                            }
                        }
                        self.variables.remove(&variable);
                        return iterator; 
                    },
                    "!" => {
                        if token.body.len() != 1 {
                            error("Function ! takes exactly 1 argument", 0, 0);
                        }

                        let mut value = self.parse_token(token.body[0].clone());
                        if value.kind != TokenKind::BOOL {
                            error("Function ! takes only boolean arguments", 0, 0);
                        }

                        value.context = (if value.context == "true" { "false" } else { "true" }).to_string();
                        return value;
                    },
                    ".." => {
                        // [.. 1 3]
                        if token.body.len() != 2 {
                            error("Function .. takes exactly 2 arguments", 0, 0);
                        }

                        let from = self.parse_token(token.body[0].clone());
                        let to = self.parse_token(token.body[1].clone());

                        if from.kind != TokenKind::INT || to.kind != TokenKind::INT {
                            error("Arguments for function .. must be integers", 0, 0);
                        }

                        let from = from.context.parse::<isize>().unwrap();
                        let to = to.context.parse::<isize>().unwrap();

                        let mut result: Vec<Token> = Vec::new();
                        for item in from..to+1 {
                            result.push(Token { kind: TokenKind::INT, context: format!("{}", item), body: Vec::new()});
                        }

                        return Token { kind: TokenKind::ARRAY, context: "Array".to_string(), body: result}
                    }
                    _ => error("Undefined function", 0, 0) // TODO position
                },
            TokenKind::INT|TokenKind::STRING|TokenKind::VOID|TokenKind::BOOL|TokenKind::WORD|TokenKind::ARRAY => token,
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
