use crate::{encoder::Encoder, statement::Statement};

pub struct Compiler<'a> {
    pub ir_code: Vec<String>,
    pub source: &'a Vec<Option<Statement>>,
    pub lines: &'a Vec<usize>,
}

impl<'a> Compiler<'a> {
    pub fn compile(&mut self) -> Result<usize, String> {
        for s in self.source {
            match s {
                Some(_s) => {
                    self.compile_statement(_s.clone())?;
                }
                _ => {}
            }
        }

        return Ok(0);
    }

    pub fn compile_statement(&mut self, src: Statement) -> Result<usize, String> {
        match src {
            Statement::Expr(expr) => {
                let mut encoder = Encoder::new();
                encoder.encode(expr, self.lines)?;
                for c in encoder.code {
                    self.ir_code.push(c);
                }
                // self.counter = self.counter + 1;
                return Ok(1);
            }
            Statement::Return(expr) => {
                let mut encoder = Encoder::new();
                let comp = encoder.encode(expr, self.lines);

                match comp {
                    Ok(last) => {
                        for c in encoder.code {
                            self.ir_code.push(c);
                        }
                        self.ir_code.push(format!("print {}", last));
                    }
                    Err(exc) => {
                        return Err(exc);
                    }
                }
                // self.counter = self.counter + 1;
                return Ok(1);
            }
            Statement::VarDecl(name, val, _kind,_index) => {
                let mut encoder = Encoder::new();
                let tval = encoder.encode(val, self.lines)?;
                self.ir_code.push(format!("new {}", name));
                self.ir_code.push(format!("{} = {}", name, tval));
                // self.counter = self.counter + 1;
                return Ok(1);
            }
            Statement::Block(stmts) => {
                let block_id = self.ir_code.len();
                self.ir_code.push(format!("label {}", block_id));
                let stmt_len = stmts.len();
                for s in stmts {
                    self.compile_statement(s)?;
                }
                // self.counter = self.counter + 1;
                return Ok(stmt_len + 1);
            }
            Statement::IfStmt(cond, then, otherwise) => {
                let mut encoder = Encoder::new();
                let tval = encoder.encode(cond, self.lines)?;
                for c in encoder.code {
                    self.ir_code.push(c);
                }
                let start = self.ir_code.len();
                self.ir_code.push(format!("jne {} ", tval));
                let _length = self.compile_statement(*then)?;
                let end_then = self.ir_code.len();
                self.ir_code.push(format!("label {}", end_then));
                self.ir_code[start].push_str(&format!("{}", end_then));

                match *otherwise {
                    Some(other) => {
                        self.ir_code.push("jump ".to_owned());
                        let start = self.ir_code.len();
                        let _length = self.compile_statement(other)?;
                        let end_else = self.ir_code.len();
                        self.ir_code.push(format!("label {}", end_else));
                        self.ir_code[start].push_str(&format!("{}", end_else));
                    }
                    _ => {}
                }

                // self.counter = self.counter + 1;
                return Ok(1);
            }
            Statement::WhileStmt(cond, repeat) => {
                let mut encoder = Encoder::new();
                let tval = encoder.encode(cond, self.lines)?;
                for c in encoder.code {
                    self.ir_code.push(c);
                }
                let start = self.ir_code.len();
                self.ir_code.push(format!("label {}", start));
                self.ir_code.push(format!("jne {} ", tval));
                let _length = self.compile_statement(*repeat)?;
                self.ir_code.push(format!("je {} {}", tval, start + 2));
                let end_then = self.ir_code.len();
                self.ir_code.push(format!("label {}", end_then));
                self.ir_code[start+1].push_str(&format!("{}", end_then));

                return Ok(1);
            }
        }
    }

    // fn compile_block(
    //     block:&Vec<Statement>,
    //     lines: &'a Vec<usize>
    // ) -> Vec<String>
    //     {
    //     let mut code = Vec::<String>::new();
    //     let mut new_src = Vec::<Option<Statement>>::new();

    //     for s in block {
    //         new_src.push(Some(s.clone()))
    //     }

    //     let cmp = Compiler{
    //         counter:0,
    //         ir_code:Vec::<String>::new(),
    //         source: &new_src,
    //         lines: lines
    //     }

    //     for s in block {
    //         cmp.compile_statement(s.clone());
    //     }

    //     return cmp.ir_code;
    // }
}
