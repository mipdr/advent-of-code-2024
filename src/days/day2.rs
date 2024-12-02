use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::Path;

fn is_safe_data(data: &Vec<i32>) -> bool {
    let it1 = data.iter();
    let it2 = it1.clone().skip(1);

    let consecutive_pairs = it1.zip(it2);
    let diffs: Vec<i32> = consecutive_pairs.map(|(x, y)| x - y ).collect();

    let safe = diffs.iter().all(|diff| diff.abs() < 4) 
        && (diffs.iter().all(|diff| *diff < 0) || diffs.iter().all(|diff| *diff > 0));

    return safe;
}

// This function assumes that the input data is not safe

fn is_safe_data_with_problem_dampener(data: &Vec<i32>) -> bool {

    // Easy and simple brute force approach
    let mut local_data;
    for (i, _) in data.iter().enumerate() {
        local_data = data.clone();
        local_data.remove(i);

        if is_safe_data(&local_data) {
            return true
        }
    }

    return false;

    // Elegant, beautiful and efficient algorithm that does not work

    // let it1 = data.iter();
    // let it2 = it1.clone().skip(1);

    // let consecutive_pairs = it1.zip(it2);
    // let diffs: Vec<i32> = consecutive_pairs.map(|(x, y)| x - y ).collect();

    // // We check if it is safe taking the Porblem Dampener into account

    // enum OutlierType {
    //     Ascending,
    //     Descending,
    //     Equal,
    // }
    
    // let outlier_type: OutlierType;

    // if diffs.iter().filter(|diff: &&i32| **diff < 0).count() == 1 {
    //     outlier_type = OutlierType::Ascending;
    // } else if diffs.iter().filter(|diff: &&i32| **diff > 0).count() == 1 {
    //     outlier_type = OutlierType::Descending;
    // } else if diffs.iter().filter(|diff: &&i32| **diff == 0).count() == 1 {
    //     outlier_type = OutlierType::Equal;
    // } else {
    //     return false;
    // }


    // let suspect_record_index = diffs.iter().enumerate().find(|(_i, diff) | {
    //     match outlier_type {
    //         OutlierType::Ascending => return **diff < 0,
    //         OutlierType::Descending => return **diff > 0,
    //         OutlierType::Equal => return **diff == 0,
    //     }
    // }).unwrap().0;

    // let mut line_without_suspect_1 = data.clone();
    // let mut line_without_suspect_2 = data.clone();

    // line_without_suspect_1.remove(suspect_record_index);
    // line_without_suspect_2.remove(suspect_record_index + 1);


    // return is_safe_data(&line_without_suspect_1) || is_safe_data(&line_without_suspect_2);

}

pub fn day2(input: &Path, result: &Path) {
    let lines: Vec<String> = read_to_string(input) 
        .unwrap()  // panic on possible file-reading errors
        .lines()
        .map(String::from)
        .collect();

    let mut data: Vec<i32>;
    // let mut it2;
    let mut safe_count: i32 = 0;
    let mut safe_count_with_dampener: i32 = 0;
    for line in lines {
        data = line.split(" ").map(|s| s.parse::<i32>().unwrap()).collect();

        if is_safe_data(&data) {
            safe_count += 1;
            safe_count_with_dampener += 1;
        } else if is_safe_data_with_problem_dampener(&data) {
            safe_count_with_dampener += 1
        }

        // Write to output file
        let mut file = File::create(result).expect("Unable to create file");
        let text: String = format!("Solution to part 1: {}\nSolution to part 2: {}", safe_count.to_string(), safe_count_with_dampener.to_string());
        file.write_all(text.as_bytes()).unwrap();
    }

}