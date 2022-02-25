// lisp inspired language
// very good very nice

use std::process::exit;

enum TokenKind {
    FUNCTION,

}

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

fn error(message: String, pos: usize) {
    println!("ERROR PICO VOLE????: {} NA POZICI VOLE {}.", message, pos);
    exit(-1);
}

fn lex(program: &String) {
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
                result.push(Word { content: last.clone(), pos, line });
                result.push(Word { content: String::from("]"), pos, line });
            },
            ' ' =>  {
                if parsing_string {
                    last.push(' ');
                } else if !last.is_empty() {
                    result.push(Word { content: last, pos, line });
                }
            },
            '\n' => {
                pos = 0;
                line += 1;
            },
            '"' => parsing_string = !parsing_string,
            _ => last.push(character)
        }
    }
}

fn main() {
    println!("Hello, world!");
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
