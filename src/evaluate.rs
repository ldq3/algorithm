// 优先级
// 结合性：左结合和右结合
// 元数：一元运算符和二元运算符
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(PartialEq)]
enum Associativity {
    Left,
    Right,
}

#[derive(PartialEq)]
enum Arity {
    Unary,
    Binary,
}

impl Operator {
    fn attri(&self) -> (usize, Associativity, Arity) {
        match self {
            Operator::Add => (1, Associativity::Left, Arity::Binary),
            Operator::Sub => (1, Associativity::Left, Arity::Binary),
            Operator::Mul => (2, Associativity::Left, Arity::Binary),
            Operator::Div => (2, Associativity::Left, Arity::Binary),
        }
    }
}

enum Elem {
    Num(i32),
    Op(Operator),
    LP, // left parenthesis
    RP, // right parenthesis 
}

enum SymbolStackElem {
    LP, // left parenthesis
    Op(Operator),
}

fn evaluate(expression: String) -> Result<i32, ()> { 
    let mut s_iter = expression.chars().peekable();

    let mut parser = || -> Result<Elem, ()> {
        if let Some(c) = s_iter.next() {
            let mut num_str = String::new();
            if c.is_numeric() {
                num_str.push(c);
                while s_iter.peek().unwrap().is_numeric() {
                    let c = s_iter.next().unwrap();
                    num_str.push(c);
                }
                return Ok(Elem::Num(num_str.parse().unwrap()));
            }

            match c {
                '(' => return Ok(Elem::LP),
                ')' => return Ok(Elem::RP),
                '+' => return Ok(Elem::Op(Operator::Add)),
                '-' => return Ok(Elem::Op(Operator::Sub)),
                '*' => return Ok(Elem::Op(Operator::Mul)),
                '/' => return Ok(Elem::Op(Operator::Div)),
                _ => (),
            }
        }
        
        Err(())
    };

    // main
    let mut sym_stack: Vec<SymbolStackElem> = Vec::new(); // symbol stack which stores the operator and parenthesis
    let mut num_stack: Vec<i32> = Vec::new();
    
    let mut process = | num: Option<i32>, op: Option<Operator> | -> Option<i32> {
        // use num_stack
        match (num, op) {
            (None, None) => {
                if num_stack.len() == 1 {
                    num_stack.pop()
                } else {
                    None
                }
            },
            (Some(value), None) => {
                num_stack.push(value);
                None
            },
            (None, Some(op)) => {
                let (_, _, arity) = op.attri();
        
                let num = if arity == Arity::Unary {
                    num_stack.pop().unwrap()
                } else {
                    0
                };

                let (num_right, num_left) = if arity == Arity::Binary {
                    (num_stack.pop().unwrap(), num_stack.pop().unwrap())
                } else {
                    (0, 0)
                };
        
                match op {
                    Operator::Add => num_stack.push(num_left + num_right),
                    Operator::Sub => num_stack.push(num_left - num_right),
                    Operator::Mul => num_stack.push(num_left * num_right),
                    Operator::Div => num_stack.push(num_left / num_right),
                } 

                None
            },
            _ => None
        }

       
    };

    while let Ok(elem) = parser() {
        match elem {
            Elem::Num(n) => { process(Some(n), None); },
            Elem::Op(op) => {
                match sym_stack.last() {
                    None | Some(SymbolStackElem::LP) => { sym_stack.push(SymbolStackElem::Op(op)); }, // thanks to Rust compiler
                    Some(SymbolStackElem::Op(_)) => {
                        let last_op = if let SymbolStackElem::Op(last_op) = sym_stack.pop().unwrap() { last_op } else { Operator::Add }; // FIXME
                        let (prior_last, assoc, _) = last_op.attri();
                        let (prior_curr, _, _) = op.attri();

                        if (prior_last < prior_curr) | (prior_last == prior_curr && assoc == Associativity::Right) {
                            sym_stack.push(SymbolStackElem::Op(op));
                        } else {
                            process(None, Some(last_op));
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
                            process(None, Some(op));
                        },
                    }
                }
            },
        }
    }
    
    match process(None, None) {
        None => Err(()),
        Some(res) => Ok(res),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn eval() {
        assert_eq!(evaluate("1 + 2 * 3 + 4 * 5 + 6".to_string()), Ok(71));
    }
}