use regex::Regex;
use std::collections::LinkedList;

#[derive(Debug)]
enum Expression<'a> {
    Number(f32),
    Operator(Operation),
    Compose(&'a Expression<'a>, Operation, &'a Expression<'a>),
}

#[derive(Debug)]
enum Operation {
    Add,
    Substract,
    Multiply,
    Divide,
}

fn str_to_expression(s: &str) -> Expression<'static> {
    let number_regex: Regex = Regex::new(r"\d+(\.\d+)?").unwrap(); // TODO: this could be a global
                                                                   // thing

    if number_regex.is_match(s) {
        str_to_number(s)
    } else {
        str_to_operator(s)
    }
}

fn str_to_number(s: &str) -> Expression<'static> {
    match s.parse() {
        Ok(num) => Expression::Number(num),
        Err(_) => panic!("unexpected number char {s}"),
    }
}

fn str_to_operator(s: &str) -> Expression<'static> {
    let operation = match s {
        "+" => Operation::Add,
        "-" => Operation::Substract,
        "*" => Operation::Multiply,
        "/" => Operation::Divide,
        other => panic!("unexpected operator char {other}"),
    };

    Expression::Operator(operation)
}

fn str_to_stack(s: &str) -> LinkedList<Expression> {
    let mut stack: LinkedList<Expression> = LinkedList::new();
    let mut expr_str = String::new();

    let mut i = 0;
    let chars: Vec<char> = s.trim().chars().collect();
    let total_chars = s.trim().chars().count();

    while i < total_chars {
        let ch = chars[i];
        i += 1;

        if ch == ' ' {
            continue;
        }

        expr_str.push(ch);

        // if it is a number and also not the last char
        if ch.is_digit(10) && i != total_chars {
            let next = chars[i];

            if next.is_digit(10) {
                continue;
            }
        }

        let expr = str_to_expression(&expr_str);

        stack.push_front(expr);
        expr_str.clear();
    }

    stack
}

fn main() {
    let input = "2 + 3 * 4 / 2"; // 8
    let stack = str_to_stack(input);

    println!("{:?}", stack);
}
