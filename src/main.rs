#![allow(unused_variables)]

// import the generated parser
use bang::eval::{eval_prgm, RunErr, Value};
use bang::parse::BangParser;

fn main() {
    let txt = "let a = b\nb\nlet b = 1 + a b + a";

    let val = match BangParser::parse(txt) {
        Ok(node) => eval_prgm(node),
        Err(e) => panic!("Error Parsing: {:?}", e),
    };

    match val {
        Ok(Value::Num(val)) => {
            println!("{}", val);
        }
        Ok(_) => {
            println!("unimplemented val");
        }
        Err(RunErr(msg)) => {
            println!("{}", msg);
        }
    }
}
