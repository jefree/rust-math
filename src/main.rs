use std::collections::LinkedList;

#[derive(Debug, Copy, Clone)]
enum Token {
    Number(f32),
    Add,
    Substract,
    Multiply,
    Divide,
    RParenthesis,
    LParenthesis,
}

fn str_to_number_expr(s: &str) -> Token {
    match s.parse() {
        Ok(num) => Token::Number(num),
        Err(_) => panic!("unexpected number char {s}"),
    }
}

fn build_operation_expr(ch: char) -> Token {
    match ch {
        '+' => Token::Add,
        '-' => Token::Substract,
        '*' => Token::Multiply,
        '/' => Token::Divide,
        other => panic!("unexpected operator char {other}"),
    }
}

fn build_number_expr(initial_digit: char, chars: &Vec<char>, i: &mut usize) -> Token {
    let mut number = String::new();
    number.push(initial_digit);

    while *i < chars.len() {
        if chars[*i].is_digit(10) {
            number.push(chars[*i]);
            *i += 1;
        } else {
            break;
        }
    }

    str_to_number_expr(&number)
}

fn str_to_list(s: &str) -> LinkedList<Token> {
    let chars: Vec<char> = s.trim().chars().collect();
    let mut tokens: LinkedList<Token> = LinkedList::new();

    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];
        i += 1;

        if ch == ' ' {
            continue;
        }

        if ch == '(' {
            tokens.push_back(Token::LParenthesis);
            continue;
        }

        if ch == ')' {
            tokens.push_back(Token::RParenthesis);
            continue;
        }

        if ch.is_digit(10) {
            let number = build_number_expr(ch, &chars, &mut i);
            tokens.push_back(number);
            continue;
        }

        let operation = build_operation_expr(ch);

        tokens.push_back(operation);
    }

    tokens
}

fn infix_to_postfix(infix: LinkedList<Token>) -> LinkedList<Token> {
    let mut postfix = LinkedList::new();
    let mut stack = LinkedList::new();

    for token in infix {
        println!("token: {:?}", token);
        println!("stack: {:?}", stack);
        println!("postfix: {:?}", postfix);
        println!("------------");

        if let Token::Number(_) = token {
            postfix.push_back(token);
            continue;
        }

        if let Token::LParenthesis = token {
            stack.push_front(token);
            continue;
        }

        if let Token::RParenthesis = token {
            loop {
                let top = stack.pop_front();

                if let None | Some(Token::LParenthesis) = top {
                    break;
                }

                postfix.push_back(top.unwrap());
            }

            continue;
        }

        let stack_op = stack.front();

        if let Some(Token::LParenthesis) | None = stack_op {
            stack.push_front(token);
            continue;
        }

        let stack_op = stack_op.unwrap();

        if prec(&token) > prec(stack_op) {
            stack.push_front(token);
        } else {
            loop {
                let next = stack.front();

                if let None | Some(&Token::LParenthesis) = next {
                    stack.push_front(token);
                    break;
                }

                if prec(&token) > prec(&next.unwrap()) {
                    stack.push_front(token);
                    break;
                }

                postfix.push_back(stack.pop_front().unwrap());
            }
        }
    }

    for remaining in stack {
        postfix.push_back(remaining);
    }

    postfix
}

fn prec(token: &Token) -> u8 {
    match token {
        Token::Multiply | Token::Divide => 2,
        Token::Add | Token::Substract => 1,
        _ => 0,
    }
}

fn main() {
    let input = "(1 + 2) * 3 / 4"; //
    let tokens = str_to_list(input);

    println!("INFIX: {:?}", tokens);
    println!("--------");

    let postfix = infix_to_postfix(tokens);

    println!("POSTFIX: {:?}", postfix);
}
