// write a test for the parser implemented in the main file
// run with `cargo test`
// cargo test -- --nocapture
#[cfg(test)]

mod tests {
    use crate::eval::{eval_prgm, RunErr, Value};
    use crate::parse::BangParser;

    #[test]
    fn test_parser_bin_mul() {
        let txt = "1 + 2 * 5 + 3 - 2";
        let exp = 1 + 2 * 5 + 3 - 2;

        let val = match BangParser::parse(txt) {
            Ok(node) => eval_prgm(node),
            Err(e) => panic!("Error: {:?}", e),
        };

        match val {
            Ok(Value::Num(val)) => {
                assert_eq!(val, exp);
            }
            Ok(_) => {
                println!("unimplemented val");
            }
            Err(RunErr(msg)) => {
                println!("{}", msg);
            }
        }
    }

    #[test]
    fn test_parser_bin_pow() {
        let txt = "2 ** 5 + 2 - 10 * 20 ** 2";
        let exp = 2_i32.pow(5) + 2 - 10 * 20_i32.pow(2);

        let val = match BangParser::parse(txt) {
            Ok(node) => eval_prgm(node),
            Err(e) => panic!("Error: {:?}", e),
        };

        match val {
            Ok(Value::Num(val)) => {
                assert_eq!(val, exp);
            }
            Ok(_) => {
                println!("unimplemented val");
            }
            Err(RunErr(msg)) => {
                println!("{}", msg);
            }
        }
    }

    #[test]
    fn test_parser_let_val() {
        let txt = "let a = 1 + 2 * 5 + 3 - 2
                   let b = 2 ** 10
                   let c = 2
                   a * c + b";
        let exp = (1 + 2 * 5 + 3 - 2) * 2 + 2_i32.pow(10);

        let val = match BangParser::parse(txt) {
            Ok(node) => {
                dbg!(&node);
                eval_prgm(node)
            }
            Err(e) => panic!("Error: {:?}", e),
        };

        match val {
            Ok(Value::Num(val)) => {
                assert_eq!(val, exp);
            }
            Ok(_) => {
                println!("unimplemented val");
            }
            Err(RunErr(msg)) => {
                println!("{}", msg);
            }
        }
    }

    #[test]
    fn test_parser_fun() {
        let txt = "fun add(a, b)
                     a + b
                   end
                   add(1, 2)";
        let exp = 3;

        let val = match BangParser::parse(txt) {
            Ok(node) => {
                dbg!(&node);
                eval_prgm(node)
            }
            Err(e) => panic!("Error: {:?}", e),
        };

        match val {
            Ok(Value::Num(val)) => {
                assert_eq!(val, exp);
            }
            Ok(_) => {
                println!("unimplemented val");
            }
            Err(RunErr(msg)) => {
                println!("{}", msg);
            }
        }
    }
}
