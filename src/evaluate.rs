// 优先级
// 结合性：左结合和右结合
// 元数：一元运算符和二元运算符
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

enum Associativity {
    Left,
    Right,
}

enum Arity {
    Unary,
    Binary,
}

impl Operator {
    fn attri(self) -> (usize, Associativity, Arity) {
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
    Lp, // left parenthesis
    Op(Operator),
}

fn evaluate(expression: String) -> Result<i32, ()> { 
    let s_iter = s.chars().peekable();

    let parser = || -> Elem {
        if let Some(c) = s_iter.next() {
            let mut num_str = String::new();
            if c.is_numeric() {
                num_str.push(c);
                while s_iter.peek().unwrap().is_numeric() {
                    let c = s_iter.next();
                    num_str.push(c);
                }
                return elem = Elem::Num(num_str.parse());
            }

            match c {
                '(' => return Elem::Op(OpStackElem::LP),
                ')' => return Elem::RP,
                '+' => return Elem::Op(OpStackElem::Op(Operator::Add)),
                '-' => return Elem::Op(OpStackElem::Op(Operator::Sub)),
                '*' => return Elem::Op(OpStackElem::Op(Operator::Mul)),
                '/' => return Elem::Op(OpStackElem::Op(Operator::Div)),
                other => panic!("wrong element: {}", other),
            }
        }
    };

    // main
    let mut sym_stack: Vec<SymbolStackElem> = Vec::new(); // symbol stack which stores the operator and parenthesis
    let mut num_stack: Vec<usize> = Vec::new();
    
    let calculater = | op: Operator | -> i32 {
        // use num_stack
        let (_, _, arity) = op.attri();
        
        let num = if arity == Arity::Unary {
            num_stack.pop().unwrap()
        };

        let (num_right, num_left) = if arity == Arity::Binary {
            (num_stack.pop().unwrap(), num_stack.pop().unwrap())
        };
        
        match op {
            Operator::Add => num_stack.push(num_left + num_right),
            Operator::Sub => num_stack.push(num_left - num_right),
            Operator::Mul => num_stack.push(num_left * num_right),
            Operator::Div => num_stack.push(num_left / num_right),
        } 
    };

    while let elem = parser() {
        match elem {
            Elem::Num(n) => { num_stack.push(n); },
            Elem::Op(op) => {
                match sym_stack.last() {
                    None | SymbolStackElem::Lp => { sym_stack.push(op); }, // thanks to Rust compiler
                    Some(SymbolStackElem::Op(last_op)) => {
                        let (prior_last, assoc, _) = last_op.attri();
                        let (prior_curr, _, _) = op.attri();

                        if (prior_last < prior_curr) | (prior_last == prior_curr && assoc == Associativity::Right) {
                            sym_stack.push(op);
                        } else {
                            let res = calculater(*last_op);
                            num_stack.push(res);
                            sym_stack.push(op);
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
                            let res = calculater(op);
                            num_stack.push(res);
                        },
                    }
                }
            },
        }
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