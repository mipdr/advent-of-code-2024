use std::fs::{self, read_to_string, File};
use std::io::Write;
use std::path::Path;

fn day1(input: &Path, result: &Path) {
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

    // Write to output file
    let mut file = File::create(result).expect("Unable to create file");
    file.write_all(sum.to_string().as_bytes()).unwrap();
}

fn main() {
    
    day1(Path::new("data/input_1_sample.txt"), Path::new("solutions/output_1_sample.txt"));
    day1(Path::new("data/input_1.txt"), Path::new("solutions/output_1.txt"));

}
