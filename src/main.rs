use bang::eval::{eval_prgm, RunErr, Value};
use bang::parse::BangParser;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = args[1].clone();

    let txt = std::fs::read_to_string(filename).unwrap();

    let val = match BangParser::parse(txt.as_str()) {
        Ok(node) => eval_prgm(node),
        Err(e) => panic!("Error Parsing: {:?}", e),
    };

    match val {
        Ok(Value::Num(val)) => {
            println!("{}", val);
        }
        Ok(Value::Fun(_, _, _)) => {
            println!("{:?}", val);
        }
        Ok(Value::Bool(_)) => {
            println!("{:?}", val);
        }
        Ok(Value::None) => {
            println!("{:?}", val);
        }
        Err(RunErr(msg)) => {
            println!("{}", msg);
        }
    }
}
