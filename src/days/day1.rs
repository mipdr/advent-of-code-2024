
use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::Path;

pub fn day1(input: &Path, result: &Path) {
    let mut array1: Vec<i32> = Vec::new();
    let mut array2: Vec<i32> = Vec::new();
    let mut diff: Vec<i32> = Vec::new();


    let mut string_parts: Vec<String>;
    for line in read_to_string(input) 
        .unwrap()  // panic on possible file-reading errors
        .lines()
        .map(String::from) {
        let local_line = line;
        string_parts = local_line.split("   ").map(String::from).collect();

        if string_parts.len() < 2 {
            continue;
        }


        array1.push(string_parts[0].parse().unwrap());
        array2.push(string_parts[1].parse().unwrap());
    }

    array1.sort();
    array2.sort();

    let pairs = array1.iter().zip(array2.iter());

    for (x, y) in pairs {
        diff.push((x - y).abs());
    }

    let sum = diff.into_iter().reduce(|a, b| a + b).unwrap();

    // println!("Solution is {}", sum);

    // Day 1 part 2

    let mut frequencies: HashMap<i32, i32> = HashMap::new();

    for el in array2.clone() {
        frequencies.insert(el, if frequencies.contains_key(&el) { frequencies[&el] + 1 } else { 1 });
    }



    // Write to output file
    let mut file = File::create(result).expect("Unable to create file");

    let aux: Vec<i32> = array1.into_iter()
        .map(|el: i32| -> i32 { el * if frequencies.contains_key(&el) { frequencies[&el] } else { 0 } }).collect();

    let result2: i32 = aux.into_iter().reduce(|a, b| a + b)
        .unwrap();


    let text: String = format!("Solution to part 1: {} \nSolution to part 2: {}", sum.to_string(), result2.to_string());
    file.write_all(text.as_bytes()).unwrap();

}