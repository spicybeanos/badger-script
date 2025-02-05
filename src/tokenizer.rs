use std::fmt;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum TokenType {
    Eof,
    EoStmt,

    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),

    Identifier(String),

    OpenParent,
    CloseParent,
    OpenSquare,
    CloseSquare,
    OpenBrace,
    CloseBrace,

    Colon,
    Period,
    Comma,
    Query,
    Refference,

    Plus,
    Minus,
    Star,
    Slash,
    Mod,

    Greater,
    GreaterEquals,
    Lesser,
    LesserEquals,
    Equality,
    BangEquals,

    And,
    Or,

    Bang,
    Asign,

    Null,
    True,
    False,
    If,
    Else,
    Fxn,
    While,
    Break,
    Continue,
    Import,
    Export,
    Return,
    Num,
    Str,
    Bool,
    IdentifierNotKeyword,
}


impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:?}]{:?} ", self.index, &self.ttype)
    }
}

#[derive(Clone)]
pub struct Token {
    pub index: usize,
    pub ttype: TokenType,
}


pub fn tokenize(src: String) -> (Vec<Token>, Vec<usize>) {
    let mut tok = Vec::<Token>::new();
    let text: Vec<char> = src.chars().collect();
    let mut new_lines: Vec<usize> = Vec::<usize>::new();
    let mut current: usize = 0;

    while current < text.len() {
        match text[current] {
            ' ' | '\t' | '\r' => {}

            '\n' => new_lines.push(current),

            ';' => tok.push(Token {
                index: current,
                ttype: TokenType::EoStmt,
            }),
            ':' => tok.push(Token {
                index: current,
                ttype: TokenType::Colon,
            }),
            '.' => tok.push(Token {
                index: current,
                ttype: TokenType::Period,
            }),
            ',' => tok.push(Token {
                index: current,
                ttype: TokenType::Comma,
            }),
            '?' => tok.push(Token {
                index: current,
                ttype: TokenType::Query,
            }),
            '@' => tok.push(Token {
                index: current,
                ttype: TokenType::Refference,
            }),
            '#' => {
                while check(current, &text, '\n', true) {
                    current = current + 1;
                }
                new_lines.push(current);
            }

            '(' => tok.push(Token {
                index: current,
                ttype: TokenType::OpenParent,
            }),
            ')' => tok.push(Token {
                index: current,
                ttype: TokenType::CloseParent,
            }),

            '[' => tok.push(Token {
                index: current,
                ttype: TokenType::OpenSquare,
            }),
            ']' => tok.push(Token {
                index: current,
                ttype: TokenType::CloseSquare,
            }),

            '{' => tok.push(Token {
                index: current,
                ttype: TokenType::OpenBrace,
            }),
            '}' => tok.push(Token {
                index: current,
                ttype: TokenType::CloseBrace,
            }),

            '+' => tok.push(Token {
                index: current,
                ttype: TokenType::Plus,
            }),
            '-' => tok.push(Token {
                index: current,
                ttype: TokenType::Minus,
            }),
            '*' => tok.push(Token {
                index: current,
                ttype: TokenType::Star,
            }),
            '/' => tok.push(Token {
                index: current,
                ttype: TokenType::Slash,
            }),
            '%' => tok.push(Token {
                index: current,
                ttype: TokenType::Mod,
            }),
            '&' => tok.push(Token {
                index: current,
                ttype: TokenType::And,
            }),
            '|' => tok.push(Token {
                index: current,
                ttype: TokenType::Or,
            }),

            '>' => {
                if check(current + 1, &text, '=', false) {
                    tok.push(Token {
                        index: current,
                        ttype: TokenType::GreaterEquals,
                    });
                    current = current + 1;
                } else {
                    tok.push(Token {
                        index: current,
                        ttype: TokenType::Greater,
                    })
                }
            }
            '<' => {
                if check(current + 1, &text, '=', false) {
                    tok.push(Token {
                        index: current,
                        ttype: TokenType::LesserEquals,
                    });
                    current = current + 1;
                } else {
                    tok.push(Token {
                        index: current,
                        ttype: TokenType::Lesser,
                    })
                }
            }
            '!' => {
                if check(current + 1, &text, '=', false) {
                    tok.push(Token {
                        index: current,
                        ttype: TokenType::BangEquals,
                    });
                    current = current + 1;
                } else {
                    tok.push(Token {
                        index: current,
                        ttype: TokenType::Bang,
                    })
                }
            }
            '=' => {
                if check(current + 1, &text, '=', false) {
                    tok.push(Token {
                        index: current,
                        ttype: TokenType::Equality,
                    });
                    current = current + 1;
                } else {
                    tok.push(Token {
                        index: current,
                        ttype: TokenType::Asign,
                    })
                }
            }

            '\"' => {
                let start: usize = current + 1;
                let mut length: usize = 0;
                let mut esc: bool = false;
                current = current + 1;

                while test_string(current, esc, &text) {
                    if text[current] == '\n' {
                        new_lines.push(current);
                    }
                    length = length + 1;
                    esc = false;
                    if text[current] == '\\' {
                        esc = true;
                    }
                    current = current + 1;
                }

                //current = current + 1;

                let s: String = substring(&text, start, length);
                let value = parse_string_literal(&s);

                tok.push(Token {
                    index: start - 1,
                    ttype: TokenType::StringLiteral(value),
                });
            }

            '0'..='9' => {
                let start: usize = current;
                let mut length: usize = 0;

                while test_number(current, &text) {
                    current = current + 1;
                    length = length + 1;
                }

                if check(current, &text, '.', false) {
                    length = length + 1;
                    current = current + 1;

                    while test_number(current, &text) {
                        current = current + 1;
                        length = length + 1;
                    }
                } else {
                    current = current - 1;
                }

                let _lex_ = substring(&text, start, length);
                let num: f64 = _lex_.clone().parse::<f64>().unwrap();
                tok.push(Token {
                    index: start,
                    ttype: TokenType::NumberLiteral(num),
                });
            }
            _ => {
                if text[current].is_alphabetic() || text[current] == '_' {
                    let start: usize = current;
                    let mut length: usize = 0;

                    while test_identifier(current, &text) {
                        current = current + 1;
                        length = length + 1;
                    }

                    let word: String = substring(&text, start, length);

                    if is_keyword(&word) {
                        tok.push(Token {
                            index: start,
                            ttype: get_keyword(&word),
                        });
                    } else {
                        tok.push(Token {
                            index: start,
                            ttype: TokenType::Identifier(word),
                        });
                    }

                    current = current - 1;
                }
            }
        }

        current = current + 1;
    }

    tok.push(Token {
        index: text.len(),
        ttype: TokenType::Eof,
    });

    return (tok, new_lines);
}

pub fn is_keyword(word: &str) -> bool {
    match word {
        "true" | "false" | "if" | "else" | "fxn" | "while" | "break" | "continue" | "import"
        | "return" | "num" | "str" | "bool" | "null" | "export" => true,
        _ => false,
    }
}

pub fn get_keyword(word: &str) -> TokenType {
    match word {
        "true" => TokenType::True,
        "false" => TokenType::False,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "fxn" => TokenType::Fxn,
        "while" => TokenType::While,
        "break" => TokenType::Break,
        "continue" => TokenType::Continue,
        "import" => TokenType::Import,
        "return" => TokenType::Return,
        "num" => TokenType::Num,
        "str" => TokenType::Str,
        "bool" => TokenType::Bool,
        "null" => TokenType::Null,
        "export" => TokenType::Export,
        _ => TokenType::IdentifierNotKeyword,
    }
}

fn check(index: usize, text: &Vec<char>, test: char, neg: bool) -> bool {
    if index < text.len() {
        if !neg {
            return text[index] == test;
        } else {
            return text[index] != test;
        }
    } else {
        false
    }
}

fn test_identifier(current: usize, text: &Vec<char>) -> bool {
    if current < text.len() {
        text[current].is_alphanumeric() || text[current] == '_'
    } else {
        false
    }
}

fn test_number(current: usize, text: &Vec<char>) -> bool {
    if current < text.len() {
        return text[current].is_digit(10) && text[current] != ' ';
    } else {
        false
    }
}

fn test_string(current: usize, esc: bool, text: &Vec<char>) -> bool {
    if current < text.len() {
        esc || text[current] != '"'
    } else {
        false
    }
}

fn substring(source: &Vec<char>, start: usize, length: usize) -> String {
    let mut sub: String = "".to_string();
    let mut i: usize = start;
    while i < start + length {
        sub.push(source[i]);
        i = i + 1;
    }
    return sub;
}

fn parse_string_literal(src: &str) -> String {
    let mut ret: String = "".to_string();
    let mut esc: bool = false;

    for c in src.chars() {
        if esc {
            match c {
                'n' => ret.push('\n'),
                't' => ret.push('\t'),

                _ => ret.push(c),
            }
            esc = false;
            continue;
        }

        if c == '\\' {
            esc = true;
        } else {
            ret.push(c);
        }
    }

    return ret;
}
