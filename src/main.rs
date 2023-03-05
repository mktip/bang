#![allow(unused_variables)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "./src/grammar.pest"]
struct LangParser;

fn main() {
    let mut variables: HashMap<String, Value> = HashMap::new();
    // let txt = "1 + 2 * 5 + 3 - 2";
    let txt = "let a = b\nb\nlet b = 1 + a b + a";
    // let exp =  2_i32.pow(2);

    match LangParser::parse(Rule::program, txt) {
        Ok(mut pairs) => {
            // dbg!(&pairs);
            // start interpreting the program
            for pair in pairs.next().unwrap().into_inner() {
                let val = match pair.as_rule() {
                    Rule::expr => eval_expr(pair, &variables),
                    Rule::stmt => eval_stmt(pair, &mut variables),
                    Rule::EOI => Ok(Value::Num(-1)),
                    _ => unreachable!(),
                };

                match val {
                    Ok(Value::Num(val)) => {
                        dbg!(val);
                    }
                    Err(RunErr(msg)) => {
                        println!("{}", msg);
                    }
                }
            }
        }
        Err(e) => panic!("Error: {:?}", e),
    }
}

fn eval_stmt(
    pair: pest::iterators::Pair<Rule>,
    variables: &mut HashMap<String, Value>,
) -> Result<Value, RunErr> {
    let pair = pair.into_inner().next().unwrap();

    match pair.as_rule() {
        Rule::ltd => {
            let mut inner = pair.into_inner();
            let idt = inner.next().unwrap().as_str();
            let eq = inner.next().unwrap();
            if let Ok(val) = eval_expr(eq, &variables) {
                variables.insert(idt.to_string(), val);
            }
        }
        _ => {
            unreachable!()
        }
    };

    Ok(Value::Num(-1))
}

fn eval_expr(
    pair: pest::iterators::Pair<Rule>,
    variables: &HashMap<String, Value>,
) -> Result<Value, RunErr> {

    match pair.as_rule() {
        Rule::binadd => eval_binadd(pair, variables),
        Rule::binmul => eval_binmul(pair, variables),
        Rule::binpow => eval_binpow(pair, variables),
        Rule::num => eval_num(pair, variables),
        Rule::expr => eval_expr(pair.into_inner().next().unwrap(), variables),
        Rule::idt => eval_idt(pair, variables),
        _ => {
            unreachable!()
        }
    }
}

#[derive(Debug, Clone)]
enum Value {
    Num(i32),
}

struct RunErr(String);

fn eval_idt(
    pair: pest::iterators::Pair<Rule>,
    variables: &HashMap<String, Value>,
) -> Result<Value, RunErr> {
    let idt = pair.as_str();
    match variables.get(idt) {
        Some(val) => Ok(val.clone()),
        None => {
            Err(RunErr(format!("Error: {} is not defined at {:?}", idt, pair.as_span())))
        }
    }
}

fn eval_binadd(
    pair: pest::iterators::Pair<Rule>,
    variables: &HashMap<String, Value>,
) -> Result<Value, RunErr> {
    let inner = pair.into_inner();
    let mut op = "+";

    // reduce over the inner pairs of the binadd rule but skip every second pair
    inner.fold(Ok(Value::Num(0)), |acc, pair| {
        match &pair.as_rule() {
            Rule::add => {
                op = "+";
                return acc;
            }
            Rule::sub => {
                op = "-";
                return acc;
            }
            _ => {}
        };

        match (acc, eval_expr(pair, variables), op) {
            (Ok(Value::Num(acc)), Ok(Value::Num(val)), "+") => Ok(Value::Num(acc + val)),
            (Ok(Value::Num(acc)), Ok(Value::Num(val)), "-") => Ok(Value::Num(acc - val)),
            (_, Err(e), _) => Err(e),
            _ => unreachable!(),
        }
    })
}

fn eval_binmul(
    pair: pest::iterators::Pair<Rule>,
    variables: &HashMap<String, Value>,
) -> Result<Value, RunErr> {
    let inner = pair.into_inner();
    let mut op = "*";

    // reduce over the inner pairs of the binmul rule
    inner.fold(Ok(Value::Num(1)), |acc, pair| {
        match &pair.as_rule() {
            Rule::mul => {
                op = "*";
                return acc;
            }
            Rule::div => {
                op = "/";
                return acc;
            }
            _ => {}
        };
        match (acc, eval_expr(pair, variables), op) {
            (Ok(Value::Num(acc)), Ok(Value::Num(val)), "*") => Ok(Value::Num(acc * val)),
            (Ok(Value::Num(acc)), Ok(Value::Num(val)), "/") => Ok(Value::Num(acc / val)),
            _ => unreachable!(),
        }
    })
}

fn eval_binpow(
    pair: pest::iterators::Pair<Rule>,
    variables: &HashMap<String, Value>,
) -> Result<Value, RunErr> {
    let mut inner = pair.into_inner();
    let left = inner.next();
    inner.next();

    // reduce over the inner pairs of the binpow rule
    inner
        .step_by(2)
        .fold(eval_expr(left.unwrap(), variables), |acc, pair| {
            match (acc, eval_expr(pair, variables)) {
                (Ok(Value::Num(acc)), Ok(Value::Num(val))) => Ok(Value::Num(acc.pow(val as u32))),
                _ => unreachable!(),
            }
        })
}

fn eval_num(
    pair: pest::iterators::Pair<Rule>,
    variables: &HashMap<String, Value>,
) -> Result<Value, RunErr> {
    match pair.as_rule() {
        Rule::num => Ok(Value::Num(pair.as_str().parse().unwrap())),
        _ => unreachable!(),
    }
}
