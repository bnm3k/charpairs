#![allow(unused_variables, dead_code)]

use core::fmt;
enum Node {
    Pair((Box<Node>, Box<Node>)),
    Char(char),
}

impl Node {
    fn pretty_print(&self, f: &mut fmt::Formatter<'_>, level: usize) -> fmt::Result {
        use Node::*;
        match self {
            Pair((a, b)) => {
                write!(f, "{:indent$}Pair(\n", "", indent = level)?;
                a.pretty_print(f, level + 4)?;
                b.pretty_print(f, level + 4)?;
                write!(f, "{:indent$})\n", "", indent = level)
            }
            Char(c) => {
                write!(f, "{:indent$}Char('{}')\n", "", c, indent = level)
            }
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.pretty_print(f, 0)
    }
}

#[derive(Debug)]
enum Token {
    LParen,
    Char(char),
    Space,
    RParen,
}

fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    for char in input.chars() {
        match char {
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            ' ' => tokens.push(Token::Space),
            c => tokens.push(Token::Char(c)),
        }
    }
    return tokens;
}

fn main() {
    let input = "(a b)";
    let _tokens = lex(input);
    let a = Box::new(Node::Char('a'));
    let b = Box::new(Node::Char('b'));
    let p: Node = Node::Pair((a, b));
    println!("{}", p);
}
