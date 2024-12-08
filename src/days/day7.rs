use std::{fs::{read_to_string, File}, io::Write, path::Path};
use itertools::Itertools;

struct Equation {
    test_value: i64,
    operands: Vec<i64>,
}

#[derive(Copy, Clone)]
enum Operators {
    Sum,
    Multiplication,
    Concatenation,
}

fn get_operator_combinations(operators: Vec<Operators>, size: usize) -> Vec<Vec<Operators>> {
    if size == 1 {
        return operators.iter().map(|op| vec![*op]).collect();
    } else {
        let result: Vec<Vec<Operators>> = operators.clone().iter().cartesian_product(
            get_operator_combinations(operators.clone(), size - 1)
        )
        .map(|(op, prev)| {
            let mut res = prev.clone();
            res.push(*op);
            return res;
        })
        .collect();

        return result;
    }
}

fn concatenate_numbers(a: i64, b: i64) -> i64 {
    let mut a_str = a.to_string();

    a_str.push_str(&b.to_string());

    return a_str.parse::<i64>().unwrap();
}

fn apply_operation(acc: &mut i64, operator: Operators, operand: i64) {
    match operator {
        Operators::Sum => *acc += operand,
        Operators::Multiplication => *acc *= operand,
        Operators::Concatenation => *acc = concatenate_numbers(*acc, operand),
    }
}

fn verify_ecuation(equation: &Equation, operators: Vec<Operators>) -> bool {
    let n_operators = equation.operands.len() - 1;

    let combinatory = get_operator_combinations(operators, n_operators);

    for operator_combination in combinatory {
        let mut result: i64 = *equation.operands.get(0).unwrap();
        let operations = operator_combination.iter().zip(equation.operands.iter().skip(1));

        for operation in operations {
            apply_operation(&mut result, *operation.0, *operation.1);
        }

        if result == equation.test_value {
            return true;
        }
    }

    return false;
}

pub fn day7(input: &Path, result: &Path) {
    let lines: Vec<String> = read_to_string(input) 
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let equations: Vec<Equation> = lines.iter()
        .map(|s| s.split(": ")
            .map(String::from)
            .collect::<Vec<String>>()
        )
        .map(|v: Vec<String>| {
            return Equation {
                test_value: v.get(0).expect("Expected test value to exist").parse::<i64>().expect("Expected test value to be int"),
                operands: v.get(1).expect("Expected operands to exist").split(" ").map(|s| s.to_string().parse::<i64>().expect("Expected operand to be int")).collect::<Vec<i64>>()
            }
        })
        .collect();

    let mut acc: i64 = 0;
    let mut part_2_acc: i64 = 0;
    for equation in equations {
        if verify_ecuation(&equation,     vec![Operators::Sum, Operators::Multiplication]) {
            acc += equation.test_value;
            part_2_acc += equation.test_value;
        } else if verify_ecuation(&equation,     vec![Operators::Sum, Operators::Multiplication, Operators::Concatenation]) {
            part_2_acc += equation.test_value;
        }
    }

    // Write to output file
    let mut file = File::create(result).expect("Unable to create file");
    let text: String = format!("Solution to part 1: {}\nSolution to part 2: {}", acc.to_string(), part_2_acc.to_string());
    file.write_all(text.as_bytes()).unwrap();
}