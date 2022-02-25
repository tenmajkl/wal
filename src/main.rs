// lisp inspired language
// very good very nice

use std::process::exit;

#[derive(Debug)]
enum TokenKind {
    FUNCTION,
    STRING,
    INT
}

struct Token {
    kind: TokenKind,
    context: String,
}



fn error(message: &str, line: usize, pos: usize) {
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

fn tokenize(word: &str) -> Token {
    let mut kind;
    if is_numeric(&word) {
        kind = TokenKind::INT;
    } else if word.starts_with('"') {
        kind = TokenKind::STRING;
    } else {
        kind = TokenKind::FUNCTION;
    }

    Token {
        kind,
        context: String::from(word)
    }
}

fn lex(program: &String) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();
    let mut last = String::new();
    let mut line = 1;
    let mut pos = 0;
    let mut parsing_string = false; 
    let mut parsing_fn = false;
    for character in program.chars() {
        pos += 1;
        match character {
            '[' => parsing_fn = true,
            ']' => {
                if !last.is_empty() {
                    result.push(tokenize(&last));
                }
                last.clear();
                parsing_fn = false;
            },
            ' ' =>  {
                if parsing_string {
                    last.push(' ');
                } else if !last.is_empty() {
                    result.push(tokenize(&last));
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
            _ => {
                if !parsing_fn {
                    error("Unexpected word", line, pos)
                }
                last.push(character)
            }
        }
    }
    return result;
}

fn main() {
    for i in lex(&String::from("  
[     print \"parek\"]       [+ 1 [- 1 2]] pare")) {
        println!("{:?} {}", i.kind, i.context);
    }
}

// [
//      Function,
//      'parek'
//      [
//          String,
//          'parek',
//          null
//      ]
// ]
