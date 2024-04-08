use std::collections::LinkedList;

#[derive(Debug)]
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

fn main() {
    let input = "(22 + 3) * 44 / 2"; // 550
    let tokens = str_to_list(input);

    println!("{:?}", tokens);
}
