#![allow(unused_variables, dead_code)]

use std::io::{self, BufRead, Write};

enum Node {
    Pair((Box<Node>, Box<Node>)),
    Char(char),
}

impl Node {
    fn pretty_print(&self, f: &mut std::fmt::Formatter<'_>, level: usize) -> std::fmt::Result {
        use Node::*;
        match self {
            Pair((a, b)) => {
                write!(f, "{:indent$}Pair(\n", "", indent = level)?;
                a.pretty_print(f, level + 2)?;
                b.pretty_print(f, level + 2)?;
                write!(f, "{:indent$})\n", "", indent = level)
            }
            Char(c) => {
                write!(f, "{:indent$}Char('{}')\n", "", c, indent = level)
            }
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.pretty_print(f, 0)
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    LParen,
    Char(char),
    Space,
    RParen,
}

impl Token {
    fn expect_lparen(&self) -> eyre::Result<()> {
        if let Token::LParen = self {
            Ok(())
        } else {
            eyre::bail!("Expected a '('")
        }
    }

    fn expect_rparen(&self) -> eyre::Result<()> {
        if let Token::RParen = self {
            Ok(())
        } else {
            eyre::bail!("Expected a ')'")
        }
    }

    fn expect_space(&self) -> eyre::Result<()> {
        if let Token::Space = self {
            Ok(())
        } else {
            eyre::bail!("Expected a ' '")
        }
    }
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

struct Scanner {
    tokens: Vec<Token>,
    i: usize,
}

impl Scanner {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            i: 0,
        }
    }
    fn peek(&self) -> &Token {
        &self.tokens[self.i]
    }

    fn take(&mut self) -> eyre::Result<&Token> {
        if let Some(t) = &self.tokens.get(self.i) {
            self.i += 1;
            Ok(t)
        } else {
            eyre::bail!("Incomplete input")
        }
    }

    fn remaining(&self) -> usize {
        self.tokens.len() - self.i
    }
}

struct Parser {
    scanner: Scanner,
}

impl Parser {
    fn new(scanner: Scanner) -> Self {
        Self { scanner: scanner }
    }

    fn parse(mut self) -> eyre::Result<Node> {
        let res = self.parse_node();
        if self.scanner.remaining() > 0 {
            eyre::bail!("Unexpected extra tokens");
        } else {
            res
        }
    }

    fn parse_node(&mut self) -> eyre::Result<Node> {
        let next = self.scanner.peek();
        match next {
            Token::LParen => self.parse_pair(),
            Token::Char(_) => self.parse_char(),
            _ => eyre::bail!("Expected ( or a Char"),
        }
    }

    fn parse_char(&mut self) -> eyre::Result<Node> {
        if let Token::Char(c) = self.scanner.take()? {
            Ok(Node::Char(*c))
        } else {
            eyre::bail!("Expected a Char")
        }
    }

    fn parse_pair(&mut self) -> eyre::Result<Node> {
        // LPAREN NODE SPACE_TOKEN NODE RPAREN
        self.scanner.take()?.expect_lparen()?;
        let left = Box::new(self.parse_node()?);
        self.scanner.take()?.expect_space()?;
        let right = Box::new(self.parse_node()?);
        self.scanner.take()?.expect_rparen()?;
        Ok(Node::Pair((left, right)))
    }
}

fn main() -> eyre::Result<()> {
    let mut stdin = io::stdin().lock();
    let mut line = String::new();
    loop {
        print!("> ");
        io::stdout().flush()?;
        stdin.read_line(&mut line)?;
        let input = &line[..line.len() - 1]; // ignore newline
        let tokens = lex(&input);
        let scanner = Scanner::new(tokens);
        let parser = Parser::new(scanner);
        match parser.parse() {
            Err(e) => println!("Error: {}", e),
            Ok(result) => print!("{}", result),
        }
        line.clear();
    }
}
