use pest::Parser;
use std::collections::LinkedList;
#[derive(Parser)]
#[grammar = "./src/grammar.pest"]
struct LangParser;

pub struct BangParser;

#[derive(Debug, Clone)]
pub enum AstNode {
    Program(LinkedList<AstNode>),
    Expr(Box<AstNode>),
    Idt(String),
    Num(String),
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Let(String, Box<AstNode>, Box<AstNode>),
    Fun(String, LinkedList<AstNode>, Box<AstNode>, Box<AstNode>),
    Funcall(String, LinkedList<AstNode>),
    BinAdd(Box<AstNode>, Box<AstNode>, Box<AstNode>),
    BinMul(Box<AstNode>, Box<AstNode>, Box<AstNode>),
    BinPow(Box<AstNode>, Box<AstNode>, Box<AstNode>),
    EOI,
}

fn astify(pair: pest::iterators::Pair<Rule>) -> Option<AstNode> {
    let node = match pair.as_rule() {
        Rule::program => {
            let mut nodes = LinkedList::new();
            for pair in pair.into_inner() {
                nodes.push_back(astify(pair)?);
            }
            AstNode::Program(nodes)
        }
        Rule::expr => AstNode::Expr(Box::new(astify(pair.into_inner().next().unwrap())?)),
        Rule::num => AstNode::Num(pair.as_str().to_string()),
        Rule::idt => AstNode::Idt(pair.as_str().to_string()),
        Rule::ltd => {
            let mut inner = pair.into_inner();
            let idt = inner.next().unwrap().as_str().to_string();
            let val = Box::new(astify(inner.next().unwrap())?);
            let body = Box::new(astify(inner.next().unwrap())?);
            AstNode::Let(idt, val, body)
        }
        Rule::fun => {
            let mut inner = pair.into_inner();
            let idt = inner.next().unwrap().as_str().to_string();

            let args =
                inner
                    .next()
                    .unwrap()
                    .into_inner()
                    .fold(LinkedList::new(), |mut acc, pair| {
                        acc.push_back(astify(pair).unwrap());
                        acc
                    });

            let body = Box::new(astify(inner.next().unwrap())?);
            let scope = Box::new(astify(inner.next().unwrap())?);
            AstNode::Fun(idt, args, body, scope)
        }
        Rule::funcall => {
            let mut inner = pair.into_inner();
            let idt = inner.next().unwrap().as_str().to_string();

            let params =
                inner
                    .next()
                    .unwrap()
                    .into_inner()
                    .fold(LinkedList::new(), |mut acc, pair| {
                        acc.push_back(astify(pair).unwrap());
                        acc
                    });

            AstNode::Funcall(idt, params)
        }
        Rule::binadd => {
            let mut inner = pair.into_inner();
            let lhs = astify(inner.next().unwrap());

            let mid = inner.fold((lhs, None), |acc, pair| match astify(pair) {
                Some(AstNode::Add) => (acc.0, Some(AstNode::Add)),
                Some(AstNode::Sub) => (acc.0, Some(AstNode::Sub)),
                None => (None, None),
                rhs => (
                    Some(AstNode::BinAdd(
                        Box::new(acc.0.unwrap()),
                        Box::new(acc.1.unwrap()),
                        Box::new(rhs.unwrap()),
                    )),
                    None,
                ),
            });

            mid.0?
        }

        Rule::binmul => {
            let mut inner = pair.into_inner();
            let lhs = astify(inner.next().unwrap());

            let mid = inner.fold((lhs, None), |acc, pair| match astify(pair) {
                Some(AstNode::Mul) => (acc.0, Some(AstNode::Mul)),
                Some(AstNode::Div) => (acc.0, Some(AstNode::Div)),
                None => (None, None),
                rhs => (
                    Some(AstNode::BinMul(
                        Box::new(acc.0.unwrap()),
                        Box::new(acc.1.unwrap()),
                        Box::new(rhs.unwrap()),
                    )),
                    None,
                ),
            });

            mid.0?
        }
        Rule::binpow => {
            let mut inner = pair.into_inner();
            let lhs = astify(inner.next().unwrap());

            let mid = inner.fold((lhs, None), |acc, pair| match astify(pair) {
                Some(AstNode::Pow) => (acc.0, Some(AstNode::Pow)),
                None => (None, None),
                rhs => (
                    Some(AstNode::BinPow(
                        Box::new(acc.0.unwrap()),
                        Box::new(acc.1.unwrap()),
                        Box::new(rhs.unwrap()),
                    )),
                    None,
                ),
            });

            mid.0?
        }
        Rule::add => AstNode::Add,
        Rule::sub => AstNode::Sub,
        Rule::mul => AstNode::Mul,
        Rule::div => AstNode::Div,
        Rule::pow => AstNode::Pow,
        Rule::EOI => AstNode::EOI,
        _ => {
            dbg!(pair.as_rule());
            unreachable!()
        }
    };

    Some(node)
}

#[derive(Debug, Clone)]
pub struct BangParseError(String);

impl BangParser {
    pub fn parse(txt: &str) -> Result<AstNode, BangParseError> {
        let pre_ast = LangParser::parse(Rule::program, txt);
        astify(pre_ast.unwrap().next().unwrap()).ok_or(BangParseError("Parse error".to_string()))
    }
}
