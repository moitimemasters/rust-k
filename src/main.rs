use std::{collections::HashMap, env, fs};


use basic_types::KeyWordType;
use token::Token;

use crate::{basic_types::{OpertaionType, TokenType}, parser::parse_program};

mod token;
mod basic_types;
mod parser;


pub fn simulate_program(program: &Vec<Token>) {
    let mut stack: Vec<i64> = Vec::new();
    let mut id_index_table: HashMap<i64, usize> = HashMap::new();
    // let mut defer_stack: Vec<i64> = Vec::new();
    for i in 0..(program.len()) {
        let token = program[i];
        match token.get_keyword_type() {
            None => {}
            _ => {
                let id = token.get_id().unwrap();
                id_index_table.insert(id, i);
            }
        }
    }
    let mut i: usize = 0;
    while i < program.len() {
        let token = program[i];
        match token {
            Token { token_type: TokenType::Opertaion, value, defered: _, operation_type, keyword_type: _, comparative_type: _, references: _, id: _, otherwise: _ } => {
                match operation_type {
                    Some(OpertaionType::Push) => {
                        if let Some(x) = value {
                            stack.push(x);
                        } else {
                            unreachable!();
                        }
                    }
                    Some(OpertaionType::Dump) => {
                        let value = stack.pop().unwrap();
                        println!("{}", value);
                    }
                    Some(OpertaionType::Plus) => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(b + a);
                    }
                    Some(OpertaionType::Minus) => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(b - a);
                    }
                    Some(OpertaionType::Mul) => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(b * a);
                    }
                    Some(OpertaionType::Div) => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(b / a);
                    }
                    Some(OpertaionType::Mod) => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(b % a);
                    }
                    Some(OpertaionType::Shr) => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(b >> a);
                    }
                    Some(OpertaionType::Shl) => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(b << a);
                    }
                    Some(OpertaionType::Band) => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(b & a);
                    }
                    Some(OpertaionType::Bor) => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(b | a);
                    }
                    Some(OpertaionType::Xor) => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(b ^ a);
                    }
                    Some(OpertaionType::Blank) => {
                        println!();
                    }
                    Some(OpertaionType::Dup) => {
                        let a = stack.pop().unwrap();
                        stack.push(a);
                        stack.push(a);
                    }
                    Some(OpertaionType::Dup2) => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(b);
                        stack.push(a);
                        stack.push(b);
                        stack.push(a);
                    }
                    Some(OpertaionType::Swap) => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push(a);
                        stack.push(b);
                    }
                    Some(OpertaionType::Swap2) => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        let c = stack.pop().unwrap();
                        let d = stack.pop().unwrap();
                        stack.push(b);
                        stack.push(a);
                        stack.push(d);
                        stack.push(c);
                    }
                    Some(OpertaionType::Pop) => {
                        stack.pop().unwrap();
                    }
                    Some(OpertaionType::Put) => {
                        let a = stack.pop().unwrap();
                        print!("{}", a)
                    }
                    Some(OpertaionType::PutSpace) => {
                        print!(" ");
                    }
                    None => { unreachable!() }
                }
            },
            Token { token_type: TokenType::KeyWord, value: _, defered: _, operation_type: _, keyword_type, comparative_type: _, references, id: _, otherwise} => {
                match keyword_type {
                    Some(KeyWordType::While) => {

                    }
                    Some(KeyWordType::Do) => {
                        let a = stack.pop().unwrap();
                        if a == 0 {
                            // println!("otherwise");
                            let goto = otherwise.unwrap();
                            // println!("goto {}", goto);
                            i = id_index_table[&goto];
                        } else {
                            let goto = references.unwrap();
                            i = id_index_table[&goto]; 
                        }
                    }
                    Some(KeyWordType::End) => {
                        i = id_index_table[&references.unwrap()];
                    }
                    Some(KeyWordType::Else) => {
                        i = id_index_table[&references.unwrap()];
                    }
                    None => { unreachable!() }
                }
            },
            Token { token_type: TokenType::Comparative, value: _, defered: _, operation_type: _, keyword_type: _, comparative_type, references: _, id: _, otherwise: _ } => {
                match comparative_type {
                    Some(basic_types::ComparativeType::Less) => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push((b < a) as i64);
                    }
                    Some(basic_types::ComparativeType::LessEq) => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push((b <= a) as i64);
                    }
                    Some(basic_types::ComparativeType::Greater) => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push((b > a) as i64);
                    }
                    Some(basic_types::ComparativeType::GreaterEq) => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push((b >= a) as i64);
                    }
                    Some(basic_types::ComparativeType::Equal) => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push((b == a) as i64);
                    }
                    Some(basic_types::ComparativeType::And) => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push((b != 0 && a != 0) as i64);
                    }
                    Some(basic_types::ComparativeType::Or) => {
                        let a = stack.pop().unwrap();
                        let b = stack.pop().unwrap();
                        stack.push((b != 0 || a != 0) as i64);
                    }
                    Some(basic_types::ComparativeType::Not) => {
                        let a = stack.pop().unwrap();
                        stack.push((a == 0) as i64);
                    }
                    None => { unreachable!() }
                }
            }
            _ => { unreachable!() }
        }
        i += 1;
    }
}

fn main() {
    // let program: Vec<Token> = vec![Token::op_push(34), Token::op_push(35), Token::op_mul(), Token::op_dump()];
    // simulate_program(&program);
    // println!("\n\n\nProgram Started:");
    // let a: String = String::from_str("1 while dup 5 <= do dup dump 1 + end").unwrap();
    // let program = parse_program(a);
    // simulate_program(&program);
    let input_konch_file = env::args().nth(1).expect("no input file");
    let string_input = fs::read_to_string(input_konch_file).expect("failed opening file");
    // println!("{}", string_input);
    let program = parse_program(string_input);
    simulate_program(&program);
}