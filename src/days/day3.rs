use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::Path;
use regex::Regex;

pub fn day3(input: &Path, result: &Path) {
    let lines: Vec<String> = read_to_string(input) 
        .unwrap()  // panic on possible file-reading errors
        .lines()
        .map(String::from)
        .collect();

    let program = lines.join("");

    let mul_intruction_finder = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut acc = 0;
    let mut acc2 = 0;
    for (_, [a, b]) in mul_intruction_finder.captures_iter(&program).map(|c| c.extract()) {
        acc += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
    }

    // Part 2
    let valid_sections: Vec<String> = program
        .split("do()")
        .map(|s: &str| s.split("don't()").next().unwrap())
        .map(String::from)
        .collect();

    for valid_section in valid_sections {
        for (_, [a, b]) in mul_intruction_finder.captures_iter(&valid_section).map(|c| c.extract()) {
            // println!("Found {} and {}", a, b);
            acc2 += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
        }
    }

    // Write to output file
    let mut file = File::create(result).expect("Unable to create file");
    let text: String = format!("Solution to part 1: {}\nSolution to part 2: {}", acc.to_string(), acc2.to_string());
    file.write_all(text.as_bytes()).unwrap();
}