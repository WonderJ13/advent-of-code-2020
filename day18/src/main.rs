use std::io;
use std::io::BufRead;

#[derive(Debug)]
enum LexItem {
    Op(char),
    Num(u64),
    OpenParen,
    CloseParen,
}

fn lex_input(expr: String) -> Vec<LexItem> {
    let mut items = vec![];
    expr.chars().for_each(|c| {
        match c {
            '0'..='9' => items.push(LexItem::Num(c.to_digit(10).unwrap().into())),
            '+' | '*' => items.push(LexItem::Op(c)),
            '(' => items.push(LexItem::OpenParen),
            ')' => items.push(LexItem::CloseParen),
            _ => (),
        };
    });
    items
}

#[derive(Debug)]
enum ParseItem {
    Add,
    Mul,
    Num(u64),
    Paren,
}

#[derive(Debug)]
struct ParseNode {
    item: ParseItem,
    left_summand: Option<Box<ParseNode>>,
    right_summand: Option<Box<ParseNode>>,
}

fn parse_expr(items: &Vec<LexItem>, pos: isize) -> Result<(ParseNode, isize), String> {
    //Plan, parse left hand side, parse operator, parse right hand side,
    //put it all together and clap your hands

    let (left_summand, new_pos) = parse_term(&items, pos)?;
    match items.get(new_pos as usize) {
        Some(LexItem::Op('+')) => {
            let (right_summand, pos_after_right) = parse_expr(&items, new_pos-1)?;
            let node = ParseNode{
                item: ParseItem::Add,
                left_summand: Some(Box::new(left_summand)),
                right_summand: Some(Box::new(right_summand)),
            };

            Ok((node, pos_after_right))
        },
        Some(LexItem::Op('*')) => {
            let (right_summand, pos_after_right) = parse_expr(&items, new_pos-1)?;
            let node = ParseNode{
                item: ParseItem::Mul,
                left_summand: Some(Box::new(left_summand)),
                right_summand: Some(Box::new(right_summand)),
            };

            Ok((node, pos_after_right))
        },
        _ => Ok((left_summand, new_pos)),
    }
}

fn parse_term(items: &Vec<LexItem>, pos: isize) -> Result<(ParseNode, isize), String> {
    match items.get(pos as usize) {
        Some(LexItem::Num(n)) => {
            let node = ParseNode{
                item: ParseItem::Num(*n),
                left_summand: None,
                right_summand: None,
            };
            Ok((node, pos - 1))
        },
        Some(LexItem::CloseParen) => {
            let (child_node, new_pos) = parse_expr(&items, pos-1)?;
            if let Some(LexItem::OpenParen) = items.get(new_pos as usize) {
                let node = ParseNode{
                    item: ParseItem::Paren,
                    left_summand: Some(Box::new(child_node)),
                    right_summand: None,
                };
                Ok((node, new_pos - 1))
            } else {
                Err(format!("Expected ending paren, might be EOL"))
            }
        },
        _ => Err(format!("Unexpected token at index {}: got {:?}", pos, items[pos as usize])),
    }
}

fn parse(items: &Vec<LexItem>) -> ParseNode {
    let (node, _) = parse_expr(items, (items.len() - 1) as isize).unwrap();
    node
}

fn solve_expression(tree: &ParseNode) -> u64 {
    match tree.item {
        ParseItem::Num(n) => {
            n
        },
        ParseItem::Add => solve_expression(&tree.left_summand.as_ref().unwrap()) + solve_expression(&tree.right_summand.as_ref().unwrap()),
        ParseItem::Mul => solve_expression(&tree.left_summand.as_ref().unwrap()) * solve_expression(&tree.right_summand.as_ref().unwrap()),
        ParseItem::Paren => solve_expression(&tree.left_summand.as_ref().unwrap()),
    }
}

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut count = 0;
    for line in stdin.lines() {
        let lex_items = lex_input(line.unwrap());
        let parse_tree = parse(&lex_items);
        count += solve_expression(&parse_tree);
    }
    println!("{}", count);
}
