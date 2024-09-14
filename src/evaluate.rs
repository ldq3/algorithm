mod operation {
    // 优先级
    // 结合性：左结合和右结合
    // 元数：一元运算符和二元运算符

    #[derive(Debug)]
    pub enum Operation {
        Add,
        Sub,
        Mul,
        Div,
        Neg,
    }

    #[derive(PartialEq)]
    pub enum Associativity {
        Left,
        Right,
    }

    pub enum Arity {
        Unary,
        Binary,
    }

    impl Operation {
        pub fn assoc(&self) -> Associativity {
            match self {
                Self::Add | Self::Sub | Self::Mul | Self::Div => Associativity::Left,
                Self::Neg => Associativity::Right
            }
        }

        pub fn arity(&self) -> Arity {
            match self {
                Self::Add | Self::Sub | Self::Mul | Self::Div => Arity::Binary,
                Self::Neg => Arity::Unary
            }
        }

        pub fn priority(&self) -> u8 {
            match self {
                Self::Add | Self::Sub => 1,
                Self::Mul | Self::Div => 2,
                Self::Neg => 3
            }
        }
    }
}

use operation::*;

#[derive(Debug)]
enum Elem {
    Num(i32),
    Op(Operation),
    LP, // left parenthesis
    RP, // right parenthesis 
}

#[derive(Debug)]
enum SymbolStackElem {
    LP, // left parenthesis
    Op(Operation),
}

enum ParseErr {
    UnknownElem,
    Empty,
}

enum ProcessMode {
    Pop,
    Push(i32),
    Calculate(Operation),
}

enum ProcessRes {
    Res(i32),
    Ok,
    WrongExp,   
}

fn evaluate(expression: String) -> Result<i32, ()> { 
    let expression = expression.replace(" ", ""); 
    let mut s_iter = expression.chars().peekable();

    let mut parser = || -> Result<Elem, ParseErr> {
        if let Some(c) = s_iter.next() {
            let mut num_str = String::new();
            if c.is_numeric() {
                num_str.push(c);
                
                while s_iter.peek().is_some() {
                    if s_iter.peek().unwrap().is_numeric() {
                        let c = s_iter.next().unwrap();
                        num_str.push(c);
                    } else { break; }
                }
                return Ok(Elem::Num(num_str.parse().unwrap()));
            }

            match c {
                '(' => return Ok(Elem::LP),
                ')' => return Ok(Elem::RP),
                '+' => return Ok(Elem::Op(Operation::Add)),
                '-' => return Ok(Elem::Op(Operation::Sub)),
                '*' => return Ok(Elem::Op(Operation::Mul)),
                '/' => return Ok(Elem::Op(Operation::Div)),
                _   => return Err(ParseErr::UnknownElem),
            }
        }
        
        Err(ParseErr::Empty)
    };

    // main
    let mut sym_stack: Vec<SymbolStackElem> = Vec::new(); // symbol stack which stores the operator and parenthesis
    let mut num_stack: Vec<i32> = Vec::new();
    
    let mut process = | mode: ProcessMode | -> ProcessRes {
        // use num_stack
        match mode {
            ProcessMode::Pop => {
                if num_stack.len() == 1 {
                    ProcessRes::Res(num_stack.pop().unwrap())
                } else {
                    ProcessRes::WrongExp
                }
            },
            ProcessMode::Push(value) => {
                num_stack.push(value);
                ProcessRes::Ok
            },
            ProcessMode::Calculate(op) => {
                let num = if let Arity::Unary = op.arity() {
                    num_stack.pop().unwrap()
                } else {
                    0
                };

                let (num_right, num_left) = if let Arity::Binary = op.arity() {
                    (num_stack.pop().unwrap(), num_stack.pop().unwrap())
                } else {
                    (0, 0)
                };
        
                match op {
                    Operation::Add => num_stack.push(num_left + num_right),
                    Operation::Sub => num_stack.push(num_left - num_right),
                    Operation::Mul => num_stack.push(num_left * num_right),
                    Operation::Div => num_stack.push(num_left / num_right),
                    _ => ()
                } 
                println!("num_stack: {:?}", num_stack);
                ProcessRes::Ok
            },
        }
    };

    while let Ok(elem) = parser() {
        // println!("elem: {:?}", elem);
        match elem {
            Elem::Num(n) => { process(ProcessMode::Push(n)); },
            Elem::Op(op) => {
                match sym_stack.last() {
                    None | Some(SymbolStackElem::LP) => { sym_stack.push(SymbolStackElem::Op(op)); }, // thanks to Rust compiler
                    Some(SymbolStackElem::Op(last_op)) => {
                        if (last_op.priority() < op.priority()) | (last_op.priority() == op.priority() && op.assoc() == Associativity::Right) {
                            sym_stack.push(SymbolStackElem::Op(op));
                        } else {
                            let last_op = if let SymbolStackElem::Op(last_op) = sym_stack.pop().unwrap() { last_op } else { Operation::Add }; // FIXME
                            process(ProcessMode::Calculate(last_op));
                            sym_stack.push(SymbolStackElem::Op(op));
                        } 
                    },
                } 
            },
            Elem::LP => {
                sym_stack.push(SymbolStackElem::LP);
            },
            Elem::RP => {
                while let Some(sym) = sym_stack.pop() {
                    match sym {
                        SymbolStackElem::LP => break,
                        SymbolStackElem::Op(op) => {
                            process(ProcessMode::Calculate(op));
                        },
                    }
                }
            },
        }
    }
    
    while let Some(sym) = sym_stack.pop() {
        match sym {
            SymbolStackElem::LP => return Err(()),
            SymbolStackElem::Op(op) => {
                println!("symbol stack: {:?}", sym_stack);
                process(ProcessMode::Calculate(op));
            }
        } 
    }

    match process(ProcessMode::Pop) {
        ProcessRes::Res(res) => Ok(res),
        ProcessRes::WrongExp => Err(()),
        _ => Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn eval() {
        assert_eq!(evaluate("1+2*3+4*5+6".to_string()), Ok(33));
    }
}