use crate::interpreter::Interpreter;
use crate::statement::Statement;
use crate::{expression::Value, symbol_table::SymbolTable};
use std::cell::RefCell;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::rc::Rc;

#[derive(Clone)]
pub enum Callable {
    None,
    Native(usize, NativeFunction),
    Custom(usize, BadgerFunction),
}
#[derive(Clone)]
pub enum NativeFunction {
    Print,
    PrintLn
}
#[derive(Clone)]
pub struct BadgerFunction {
    name: String,
    params: Vec<String>,
    body: Vec<Option<Statement>>,
}
impl Debug for NativeFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            NativeFunction::Print => {
                write!(f, "print")
            },
            NativeFunction::PrintLn => {
                write!(f, "println")
            }
        }
    }
}

impl Debug for Callable {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Callable::None => {
                return write!(f, "Not assigned");
            }
            Callable::Native(_, fxn) => {
                return write!(f, "Native function {:?}", fxn);
            }
            Callable::Custom(_, nf) => {
                return write!(f, "Custom function {:?}", &nf.name);
            }
        }
    }
}

impl BadgerFunction {
    pub fn new(
        name:&str,
        params:&Vec<String>,
        body:&Vec<Statement>
    ) -> BadgerFunction {
        let mut ins_body = Vec::<Option<Statement>>::new();
        for b in body {
            ins_body.push(Some(b.to_owned()));
        }

        return BadgerFunction {
            name:name.to_owned(),
            params:params.to_owned(),
            body:ins_body
        };
    }
    pub fn call(
        &self,
        args_val: Vec<Value>,
        table: Rc<RefCell<SymbolTable>>,
        debug_lines: &Vec<usize>,
        index: usize,
    ) -> Result<Value, String> {
        let mut local_table:SymbolTable = SymbolTable::new(Some(table));
        for (i,p) in self.params.iter().enumerate() {
            local_table.add_symbol(p,args_val[i].clone(),&index,debug_lines)?;
        }
        let mut interpret = Interpreter::new(Rc::<RefCell::<SymbolTable>>::new(RefCell::new(local_table)),&self.body,debug_lines);
        return interpret.interpret();
    }
}

impl NativeFunction {
    pub fn call(&self, args_val: Vec<Value>, _index: usize) -> Result<Value, String> {
        match self {
            NativeFunction::Print => {
                print!("{:?}", args_val[0]);
                return Ok(Value::Number(0.0));
            },
            NativeFunction::PrintLn => {
                print!("{:?}\n", args_val[0]);
                return Ok(Value::Number(0.0));
            }
        }
    }
}

impl Callable {
    pub fn call(
        &self,
        args_val: Vec<Value>,
        table: Rc<RefCell<SymbolTable>>,
        debug_lines: &Vec<usize>,
        index: usize,
    ) -> Result<Value, String> {
        match self {
            Callable::None => Ok(Value::Number(0.0)),
            Callable::Native(_, fxn) => return fxn.call(args_val, index),
            Callable::Custom(_, fxn) => {
                fxn.call(args_val, table, debug_lines, index)
            }
        }
    }

    pub fn arity(&self) -> usize {
        match self {
            Callable::None => 0,
            Callable::Native(args, _) => *args,
            Callable::Custom(nargs, _) => *nargs,
        }
    }

    pub fn is_assigned(&self) -> bool {
        match &self {
            Callable::None => false,
            _ => true,
        }
    }
}
