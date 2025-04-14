use std::env;
use std::fs;

pub mod badger_debug;
pub mod compiler;
pub mod encoder;
pub mod expression;
pub mod parser;
pub mod tokenizer;
// pub  mod virtual_machine;
pub mod interpreter;
pub mod statement;
pub mod symbol_table;
use compiler::Compiler;
use interpreter::Interpreter;
use parser::ExprStmtParser;
use statement::Statement;
use std::cell::RefCell;
use std::rc::Rc;
use symbol_table::SymbolTable;
use tokenizer::tokenize;
use tokenizer::Token;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!(
            "No input file specified!\nUsage : {0} (c|i) [input file]",
            args[0]
        );
        return;
    }
    let choice = args[1].clone();
    let _inp_file_path: String = args[2].clone();

    if choice != "c" && choice != "i" {
        println!(
            "Wrong choice specified!\nUsage : {0} (c|i) [input file]",
            args[0]
        );
        println!("c -> compile to IR code");
        println!("i -> interpret code");
        return;
    }

    let contents =
        fs::read_to_string(_inp_file_path).expect("Should have been able to read the file");

    let (tokens, lines): (Vec<Token>, Vec<usize>) = tokenize(contents);

    //for tok_ in &tokens {
    //    println!(
    //        "{:?} at line {}",
    //        tok_,
    //        get_line_from_index(tok_.index, &lines)
    //    );
    //}
    let mut parser: ExprStmtParser<'_> = ExprStmtParser::new(&tokens, &lines, 0);

    let table: SymbolTable = SymbolTable::new(None);
    let rst = parser.parse_statement();
    let stmt: Vec<Option<Statement>>;
    let mut interpreter: Interpreter<'_>;
    match rst {
        Ok(s) => {
            stmt = s;
            if choice == "i" {
                interpreter = Interpreter::new(
                    Rc::<RefCell<SymbolTable>>::new(RefCell::new(table)),
                    &stmt,
                    &lines,
                );
                let _exec_res = interpreter.interpret();

                match _exec_res {
                    Ok(_) => {}
                    Err(er) => println!("Error!\n{}", er),
                }
            } else if choice == "c" {
                let mut compiler = Compiler {
                    ir_code: Vec::<String>::new(),
                    source: &stmt,
                    lines: &lines,
                };
                let result = compiler.compile();

                match result {
                    Ok(_) => {
                        compiler.ir_code.push("end".to_owned());
                        for ir in compiler.ir_code {
                            println!("{} ",ir);
                        }
                    }
                    Err(er) => {
                        println!("Error compiling! {}",er);
                    }
                }
            }
        }
        Err(er) => {
            println!("Could not parse statements:{:?}", er);
            return;
        }
    }
}
