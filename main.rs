use std::io::{stdin, stdout, Write};

fn read(input: &mut String) {
    stdout().flush().expect("failed to flush");
    stdin().read_line(input).expect("failed to read");
}

fn menu() {
    println!("More advanced calculator --Rwy/Vrsth");
    println!("====================================");
    println!("");
}

fn is_string_numeric(str: String) -> bool {
    for c in str.chars() {
        if c.is_alphabetic() {
            return false;
        }
    }
    return true;
}

fn get_valid_number() -> f32 {
    let mut num = String::from("0");
    loop {
        print!("Enter number: ");
        num.clear();
        read(&mut num);
        if is_string_numeric(num.trim().to_string()) {
            break;
        }
    }

    return num.trim().parse().unwrap();
}

fn main() {
    menu();
    loop {
        let mut _num1 = 0 as f32;
        let mut _num2 = 0 as f32;
        let mut _op = String::new();
        let mut _use_last_number = false;

        let operators = String::from("+-*/");

        let num1: f32 = get_valid_number();
        loop {
            print!("Enter operator: ");
            _op.clear();
            read(&mut _op);
            if operators.contains(_op.chars().nth(0).unwrap()) {
                break;
            }
        }
        let op: char = _op.trim().chars().next().unwrap();
        let num2: f32 = get_valid_number();

        let result = match op {
            '+' => num1 + num2,
            '-' => num1 - num2,
            '*' => num1 * num2,
            '/' => num1 / num2,
            _ => panic!("Invalid operator"),
        };

        println!("{} {} {} = {}", num1, op, num2, result);

        print!("Do you want to continue [Y]es / [N]o :");
        let mut temp = String::new();
        read(&mut temp);

        if temp.to_lowercase() == "y" {
            let _use_last_number: bool = true;
        } else {
            let _use_last_number: bool = false;
            break;
        }
    }
}
