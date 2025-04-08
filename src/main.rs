use std::env;
use std::fs;
pub mod expr_stmt_parser;
pub mod badger_debug;
pub mod expression;
pub mod tokenizer;
// pub  mod virtual_machine;
pub mod symbol_table;
pub mod interpreter;
pub mod statement;
use expr_stmt_parser::ExprStmtParser;
use interpreter::Interpreter;
use statement::Statement;
use symbol_table::SymbolTable;
use tokenizer::tokenize;
use tokenizer::Token;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!(
            "No input file specified!\nUsage : {0} [input file] (optional)[output file]",
            args[0]
        );
        return;
    }
    let _inp_file_path: String = args[1].clone();

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
    let mut parser: ExprStmtParser<'_> = 
    ExprStmtParser::new(&tokens,&lines,0);
    
    let mut table:SymbolTable = SymbolTable::new();
    let rst = parser.parse_statement();
    let stmt : Vec<Option<Statement>>;
    let mut interpreter :Interpreter<'_>;
    match rst {
        Ok(s) => {
            stmt = s;
            interpreter = Interpreter::new(&mut table, &stmt,&lines);
        },
        Err(er) => {println!("Could not parse statements:{:?}",er);return;}
    }

    let _exec_res = interpreter.interpret();
    
    match _exec_res {
        Ok(_) => {},
        Err(er) => println!("Error!\n{}",er)
    }
}
