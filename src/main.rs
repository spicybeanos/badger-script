use std::env;
use std::fs;
pub mod expression_parser;
pub mod expression;
pub mod tokenizer;
pub mod symbol_table;
use tokenizer::get_line_from_index;
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

    for tok_ in tokens {
        println!(
            "{:?} at line {}",
            tok_,
            get_line_from_index(tok_.index, &lines)
        );
    }
}
