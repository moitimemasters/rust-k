use crate::{basic_types::KeyWordType, token::Token};


pub fn get_available_operation(a: &str) -> Result<Token, &'static str> {
    match a {
        "+" => Ok(Token::op_plus()),
        "-" => Ok(Token::op_minus()),
        "*" => Ok(Token::op_mul()),
        "div" => Ok(Token::op_div()),
        "mod" => Ok(Token::op_mod()),
        ">>" => Ok(Token::op_shr()),
        "<<" => Ok(Token::op_shl()),
        "|" => Ok(Token::op_bor()),
        "&" => Ok(Token::op_band()),
        "^" => Ok(Token::op_xor()),
        "dump" => Ok(Token::op_dump()),
        "blank" => Ok(Token::op_blank()),
        "dup" => Ok(Token::op_dup()),
        "dup2" => Ok(Token::op_dup2()),
        "swap" => Ok(Token::op_swap()),
        "swap2" => Ok(Token::op_swap2()),
        "pop" => Ok(Token::op_pop()),
        "put" => Ok(Token::op_put()),
        _ => Err("Unimplemented"),
    }
}

pub fn get_available_comparable(a: &str) -> Result<Token, &'static str> {
    match a {
        "<" => Ok(Token::cmp_less()),
        "<=" => Ok(Token::cmp_less_eq()),
        ">" => Ok(Token::cmp_greater()),
        ">=" => Ok(Token::cmp_greater_eq()),
        "==" => Ok(Token::cmp_equal()),
        "and" => Ok(Token::cmp_and()),
        "or" => Ok(Token::cmp_or()),
        "not" => Ok(Token::cmp_not()),
        _ => Err("Unimplemented"),
    }
}

pub fn get_available_keyword(a: &str, id: i64) -> Result<Token, &'static str> {
    match a {
        "while" => Ok(Token::kw_while(id)),
        "do" => Ok(Token::kw_do(id, id)),
        "end" => Ok(Token::kw_end(id, id)),
        _ => Err("Unimplemented"),
    }
}

pub fn parse_program(string: String) -> Vec<Token> {
    let mut program: Vec<Token> = Vec::new();
    let mut current_keyword_id = -1;
    let mut do_while_end_stack: Vec<(Token, usize)> = Vec::new();
    let split_tokens = string.split_whitespace();
    split_tokens.enumerate().for_each(|(index, token)| {
        // println!("{}", token);
        if let Ok(x) = token.parse::<i64>() {
            // println!("push {}", x);
            program.push(Token::op_push(x));
        } else {
            if let Ok(result_token) = get_available_operation(token) {
                program.push(result_token);
            } else if let Ok(result_token) = get_available_comparable(token) {
                program.push(result_token);
            } else if let Ok(mut result_token) =
                get_available_keyword(token, current_keyword_id + 1)
            {
                current_keyword_id += 1;
                if token == "while" {
                    do_while_end_stack.push((result_token, index));
                } else if token == "do" {
                    // println!("do index is {}", index);
                    do_while_end_stack.push((result_token, index));
                } else if token == "end" {
                    if let Some((top_token, index)) = do_while_end_stack.pop() {
                        // println!("index is: {}", index);
                        match top_token.get_keyword_type() {
                            Some(KeyWordType::Do) => {
                                program[index].change_reference(Some(current_keyword_id));
                            }
                            // Some(KeyWordType::While) => {
                            //     // println!("while");
                            // }
                            // Some(KeyWordType::End) => {
                            //     // println!("end");
                            // }
                            _ => {
                                unreachable!("`end` needs `do`")
                            }
                        }
                    } else {
                        unreachable!("`end` needs `do`")
                    }
                    if let Some((top_token, index)) = do_while_end_stack.pop() {
                        match top_token.get_keyword_type() {
                            Some(KeyWordType::While) => {
                                result_token.change_reference(program[index].get_id());
                            }
                            _ => {
                                do_while_end_stack.push((top_token, index));
                            }
                        }
                    }
                }
                program.push(result_token);
            } else {
                unreachable!();
            }
        }
    });
    program
}
