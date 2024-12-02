use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::Path;

pub fn day2(input: &Path, result: &Path) {
    let lines: Vec<String> = read_to_string(input) 
        .unwrap()  // panic on possible file-reading errors
        .lines()
        .map(String::from)
        .collect();

    let mut it1;
    let mut it2;
    let mut safe_count: i32 = 0;
    for line in lines {
        it1 = line.split(" ").map(|s| s.parse::<i32>().unwrap());
        it2 = it1.clone();

        let consecutive_pairs = it1.into_iter().skip(1).zip(it2.into_iter());

        let diffs: Vec<i32> = consecutive_pairs.map(|(x, y)| x - y ).collect();

        let safe = diffs.iter().all(|diff| diff.abs() < 4) 
            && (diffs.iter().all(|diff| *diff < 0) || diffs.iter().all(|diff| *diff > 0));

        if safe {
            safe_count += 1;
        }

        // Write to output file
        let mut file = File::create(result).expect("Unable to create file");
        let text: String = format!("Solution to part 1: {}", safe_count.to_string());
        file.write_all(text.as_bytes()).unwrap();
    }

}