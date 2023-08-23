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
                panic!("{}", msg);
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
                panic!("{}", msg);
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
                panic!("{}", msg);
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
                panic!("{}", msg);
            }
        }
    }

    #[test]
    fn test_parser_fun_mult() {
        let txt = "
                let x = 10

                let y = 20

                fun add(a, b)
                  a + b
                end

                let j = 40

                add(j, y)

                add(j, x)
                ";
        let exp = 50;

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
                panic!("{}", msg);
            }
        }
    }

    #[test]
    fn test_parser_match() {
        let txt = "
            match 10
                2 => 3
                3 => 4
                1 => 20
                _ => 0
            end
            ";
        let exp = 0;

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
                panic!("{}", msg);
            }
        }
    }

    #[test]
    fn test_parser_fun_fun() {
        let txt = "
            fun wow()
            1
            end
            ";
        let exp = 0;

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
                panic!("{}", msg);
            }
        }
    }
}
