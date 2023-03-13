use crate::parse::AstNode;
use std::collections::HashMap;
use std::collections::LinkedList;

#[derive(Debug, Clone)]
pub enum Value {
    Num(i32),
    Fun(LinkedList<AstNode>, AstNode, Env),
}

#[derive(Debug, Clone)]
pub struct RunErr(pub String);

#[derive(Debug, Clone)]
pub struct Env {
    env: HashMap<String, Value>,
    parent: Option<Box<Env>>,
}

impl Env {
    fn new() -> Env {
        Env {
            env: HashMap::new(),
            parent: None,
        }
    }

    fn new_child(&self) -> Env {
        Env {
            env: HashMap::new(),
            parent: Some(Box::new(self.clone())),
        }
    }

    fn get(&self, idt: &str) -> Option<Value> {
        match self.env.get(idt) {
            Some(val) => Some(val.clone()),
            None => match &self.parent {
                Some(env) => env.get(idt),
                None => None,
            },
        }
    }

    fn insert(&mut self, idt: String, val: Value) {
        self.env.insert(idt, val);
    }
}

pub fn eval_prgm(pair: AstNode) -> Result<Value, RunErr> {
    let env = Env::new();
    let mut val = Value::Num(-1);

    let lst = match pair {
        AstNode::Program(lst) => lst,
        _ => {
            unreachable!()
        }
    };

    for pair in lst {
        val = match pair {
            AstNode::Expr(expr) => eval_expr(*expr, &mut env.new_child())?,
            AstNode::EOI => continue,
            _ => {
                unreachable!()
            }
        };
    }

    Ok(val)
}

fn eval_expr(pair: AstNode, env: &mut Env) -> Result<Value, RunErr> {
    match pair {
        AstNode::BinAdd(_, _, _) => eval_binadd(pair, env),
        AstNode::BinMul(_, _, _) => eval_binmul(pair, env),
        AstNode::BinPow(_, _, _) => eval_binpow(pair, env),
        AstNode::Num(_) => eval_num(pair),
        AstNode::Let(_, _, _) => eval_ltd(pair, env),
        AstNode::Idt(_) => eval_idt(pair, env),
        AstNode::Expr(expr) => eval_expr(*expr, env),
        AstNode::Fun(_, _, _, _) => eval_fun(pair, env),
        AstNode::Funcall(_, _) => eval_funcall(pair, env),
        _ => {
            unreachable!()
        }
    }
}

fn eval_ltd(pair: AstNode, env: &mut Env) -> Result<Value, RunErr> {
    let val = match pair {
        AstNode::Let(idt, eqv, body) => {
            if let Ok(val) = eval_expr(*eqv, &mut env.new_child()) {
                env.insert(idt.to_string(), val);
            }

            eval_expr(*body, &mut env.new_child())?
        }
        _ => {
            dbg!(pair);
            unreachable!()
        }
    };

    Ok(val)
}

fn eval_fun(pair: AstNode, env: &mut Env) -> Result<Value, RunErr> {
    match pair {
        AstNode::Fun(idt, args, eqv, body) => {
            let fun = Value::Fun(args, *eqv, env.clone());
            env.insert(idt.to_string(), fun);

            eval_expr(*body, &mut env.new_child())
        }
        _ => {
            unreachable!()
        }
    }
}

fn eval_funcall(pair: AstNode, env: &mut Env) -> Result<Value, RunErr> {
    match pair {
        AstNode::Funcall(idt, params) => {
            let fun = match env.get(&idt) {
                Some(val) => val,
                None => return Err(RunErr(format!("Error: {} is not defined", &idt))),
            };

            match fun {
                Value::Fun(args, eqv, fenv) => {
                    let mut new_env = fenv.new_child();

                    for (arg, param) in args.iter().zip(params.iter()) {
                        let arg = match arg {
                            AstNode::Idt(idt) => idt,
                            _ => {
                                unreachable!()
                            }
                        };

                        let param = match param {
                            AstNode::Expr(expr) => *(*expr).clone(),
                            _ => {
                                unreachable!()
                            }
                        };

                        let val = eval_expr(param, &mut env.new_child())?;
                        new_env.insert(arg.to_string(), val);
                    }

                    eval_expr(eqv, &mut new_env)
                }
                _ => {
                    unreachable!()
                }
            }
        }
        _ => {
            unreachable!()
        }
    }
}

fn eval_idt(pair: AstNode, env: &mut Env) -> Result<Value, RunErr> {
    let idt = match pair {
        AstNode::Idt(idt) => idt,
        _ => {
            unreachable!()
        }
    };

    match env.get(&idt) {
        Some(val) => Ok(val.clone()),
        None => Err(RunErr(format!("Error: {} is not defined", &idt))),
    }
}

fn eval_binadd(pair: AstNode, env: &mut Env) -> Result<Value, RunErr> {
    let (lhs, op, rhs) = match pair {
        AstNode::BinAdd(lhs, op, rhs) => (lhs, op, rhs),
        _ => {
            dbg!(pair);
            unreachable!()
        }
    };

    match (
        eval_expr(*lhs, &mut env.new_child()),
        eval_expr(*rhs, &mut env.new_child()),
        *op,
    ) {
        (Ok(Value::Num(lhs)), Ok(Value::Num(rhs)), AstNode::Add) => Ok(Value::Num(lhs + rhs)),
        (Ok(Value::Num(lhs)), Ok(Value::Num(rhs)), AstNode::Sub) => Ok(Value::Num(lhs - rhs)),
        (_, Err(e), _) => Err(e),
        _ => unreachable!(),
    }
}

fn eval_binmul(pair: AstNode, env: &mut Env) -> Result<Value, RunErr> {
    let (lhs, op, rhs) = match pair {
        AstNode::BinMul(lhs, op, rhs) => (lhs, op, rhs),
        _ => {
            unreachable!()
        }
    };

    match (
        eval_expr(*lhs, &mut env.new_child()),
        eval_expr(*rhs, &mut env.new_child()),
        *op,
    ) {
        (Ok(Value::Num(lhs)), Ok(Value::Num(rhs)), AstNode::Mul) => Ok(Value::Num(lhs * rhs)),
        (Ok(Value::Num(lhs)), Ok(Value::Num(rhs)), AstNode::Div) => Ok(Value::Num(lhs / rhs)),
        (_, Err(e), _) => Err(e),
        _ => unreachable!(),
    }
}

fn eval_binpow(pair: AstNode, env: &mut Env) -> Result<Value, RunErr> {
    let (lhs, op, rhs) = match pair {
        AstNode::BinPow(lhs, op, rhs) => (lhs, op, rhs),
        _ => {
            unreachable!()
        }
    };

    match (
        eval_expr(*lhs, &mut env.new_child()),
        eval_expr(*rhs, &mut env.new_child()),
        *op,
    ) {
        (Ok(Value::Num(lhs)), Ok(Value::Num(rhs)), AstNode::Pow) => {
            Ok(Value::Num(lhs.pow(rhs as u32)))
        }
        (_, Err(e), _) => Err(e),
        _ => unreachable!(),
    }
}

fn eval_num(pair: AstNode) -> Result<Value, RunErr> {
    let num = match pair {
        AstNode::Num(num) => num,
        _ => {
            unreachable!()
        }
    };

    Ok(Value::Num(num.parse::<i32>().unwrap()))
}
