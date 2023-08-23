use crate::parse::AstNode;
use std::collections::HashMap;
use std::collections::LinkedList;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]

pub enum Value {
    Num(i32),
    Fun(LinkedList<AstNode>, LinkedList<AstNode>, Rc<RefCell<Env>>),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub struct RunErr(pub String);

#[derive(Debug, Clone)]
pub struct Env {
    env: HashMap<String, Rc<RefCell<Value>>>,
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
            Some(val) => Some(val.borrow().clone()),
            None => match &self.parent {
                Some(env) => env.get(idt),
                None => None,
            },
        }
    }

    fn insert(&mut self, idt: String, val: Rc<RefCell<Value>>) {
        self.env.insert(idt, val);
    }
}

pub fn eval_prgm(pair: AstNode) -> Result<Value, RunErr> {
    let mut env = Env::new();
    let mut val = Value::Num(-1);

    let lst = match pair {
        AstNode::Program(lst) => lst,
        _ => {
            unreachable!()
        }
    };

    for pair in lst {
        val = match pair {
            AstNode::Expr(expr) => eval_expr(*expr, &mut env)?,
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
        AstNode::Let(_, _) => eval_ltd(pair, env),
        AstNode::Idt(_) => eval_idt(pair, env),
        AstNode::Expr(expr) => eval_expr(*expr, env),
        AstNode::Fun(_, _, _) => eval_fun(pair, env),
        AstNode::Funcall(_, _) => eval_funcall(pair, env),
        AstNode::Match(_, _) => eval_match(pair, env),
        _ => {
            unreachable!()
        }
    }
}

fn eval_match(pair: AstNode, env: &mut Env) -> Result<Value, RunErr> {
    let (expr, cases) = match pair {
        AstNode::Match(expr, cases) => (expr, cases),
        _ => {
            unreachable!()
        }
    };

    let expr = match *expr {
        AstNode::Expr(expr) => *expr,
        _ => {
            unreachable!()
        }
    };

    let expr = eval_expr(expr, &mut env.new_child())?;

    let res = cases.into_iter().find_map(|case| {
        let (pat, eqv) = match case {
            AstNode::Branch(pat, eqv) => (pat, eqv),
            _ => {
                unreachable!()
            }
        };

        let pat = match *pat {
            AstNode::Expr(new) => *new,
            AstNode::Default => {
                return Some(eval_expr(*eqv, &mut env.new_child()));
            }
            _ => {
                unreachable!()
            }
        };

        let pat = eval_expr(pat, &mut env.new_child());

        match (&expr, &pat) {
            (&Value::Num(a), Ok(Value::Num(b))) if a.eq(b) => {
                Some(eval_expr(*eqv, &mut env.new_child()))
            }
            _ => None,
        }
    });

    Ok(res.unwrap()?)
}

fn eval_ltd(pair: AstNode, env: &mut Env) -> Result<Value, RunErr> {
    let val = match pair {
        AstNode::Let(idt, eqv) => {
            if let Ok(val) = eval_expr(*eqv, &mut env.new_child()) {
                env.insert(idt.to_string(), Rc::new(RefCell::new(val.clone())));
                Ok(val)
            } else {
                Err(RunErr("Error when evaluating val of let".to_string()))
            }

            // eval_expr(*body, &mut env.new_child())?
        }
        _ => {
            unreachable!()
        }
    };

    val
}

fn eval_fun(pair: AstNode, env: &mut Env) -> Result<Value, RunErr> {
    match pair {
        AstNode::Fun(idt, args, eqv) => {
            let fenv = Rc::new(RefCell::new(env.new_child()));

            // for body in eqv.clone() {
            //     dbg!(&body);
            // }
            let fun = Rc::new(RefCell::new(Value::Fun(args, eqv, fenv.clone())));

            fenv.borrow_mut().insert(idt.to_string(), fun.clone());

            env.insert(idt.to_string(), fun);

            Ok(Value::Num(0))
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
                None => return Err(RunErr(format!("Error function {} is not defined", &idt))),
            };

            match fun {
                Value::Fun(args, eqv, fenv) => {
                    let mut new_env = fenv.borrow_mut().new_child();

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
                        new_env.insert(arg.to_string(), Rc::new(RefCell::new(val)));
                    }

                    new_env.insert(
                        idt.to_string(),
                        Rc::new(RefCell::new(Value::Fun(args, eqv.clone(), fenv))),
                    );

                    let mut res = Err(RunErr("Error when evaluating function".to_string()));

                    for body in eqv {
                        res = eval_expr(body, &mut new_env);
                    }

                    res
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
        None => Err(RunErr(format!("Error: identifier {} is not defined", &idt))),
    }
}

fn eval_binadd(pair: AstNode, env: &mut Env) -> Result<Value, RunErr> {
    let (lhs, op, rhs) = match pair {
        AstNode::BinAdd(lhs, op, rhs) => (lhs, op, rhs),
        _ => {
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
