// lisp inspired language
// very good very nice

use std::process::exit;

#[derive(Debug)]
enum TokenKind {
    FUNCTION,
    STRING,
    INT,
    VOID
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    context: String,
    body: Vec<Token>
}

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
            '"' => {
                last.push('"');
                parsing_string = !parsing_string;
            },
            _ => last.push(character)
        }
    }
    return result;
}

fn tokenize_fn(body: &Vec<Word>, start: usize) -> (Vec<Token>, usize) {
    let mut tokens: Vec<Token> = Vec::new();
    for index in start..body.len() {
        let word = &body[index];
        let mut kind = TokenKind::VOID;
        if is_numeric(&word.content) {
            kind = TokenKind::INT;
        } else if word.content.starts_with('"') {
            kind = TokenKind::STRING;
        } else if word.content == "[" {
            tokens. tokenize_fn(body.clone(), index);
        } else if word.content == "]" {
            return (tokens, index);
        } else {
            if index != start {
                error("Undefined token", body[index].line, body[index].pos);
            }

            kind = TokenKind::FUNCTION;
        }
        
        tokens.push(Token {
            kind,
            body: Vec::new(),
            context: String::from(&word.content),
        });
    }
    error("Unclosed function call", 0, 0);
}

fn tokenize(program: Vec<Word>) {
    let mut tokens: Vec<Token> = Vec::new();
    let mut skip = 0;
    for index in 0..program.len() {
        if index >= skip {
            match program[index].content.as_str() {
                "[" => {
                    let (body, end) = tokenize_fn(&program, index+1);
                    skip = end+1;
                    println!("{}", skip);
                    tokens.push(Token { kind: TokenKind::FUNCTION, body, context: String::from(&program[index].content) });
                },
                _ => error("Unexpected token", program[index].line, program[index].pos)
            }
        }
    }
}

fn main() {
    println!("{:?}", tokenize(lex(&String::from("
[parek \"parek\"] [+ 1 2]]"))));
}
