use std::{fs::{read_to_string, File}, io::Write, path::Path};

fn check_safe(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    let mut safe_update = true;

    for rule in rules {
        // Check if update is rule conformant

        // General case for n occurences
        // let occurences_first_rule_number: Vec<usize> = update.iter()
        //     .enumerate()
        //     .filter(|(_i, page_number)| **page_number == rule.0)
        //     .map(|(i, _page_number)| i)
        //     .collect();

        // let occurences_second_rule_number: Vec<usize> = update.iter()
        //     .enumerate()
        //     .filter(|(_i, page_number)| **page_number == rule.1)
        //     .map(|(i, _page_number)| i)
        //     .collect();

        let first_number_index_option= update.iter().enumerate().find(|(_i, el)| **el == rule.0);
        let second_number_index_option = update.iter().enumerate().find(|(_i, el)| **el == rule.1);

        // Rule does not apply
        if first_number_index_option.is_none() || second_number_index_option.is_none() {
            continue;
        }

        let (first_number_index, _) = first_number_index_option.unwrap();
        let (second_number_index, _) = second_number_index_option.unwrap();

        // General case for n occurences
        // let rule_conformant = occurences_first_rule_number.iter()
        //     .all(|i| occurences_second_rule_number.iter()
        //         .all(|j| j > i)
        //     );

        let rule_conformant = second_number_index > first_number_index;

        safe_update = safe_update && rule_conformant;
    }

    return safe_update;
}

fn fix_update(update: &mut Vec<i32>, rules: &Vec<(i32, i32)>) {
    for rule in rules {
        let first_number_index_option= update.iter().enumerate().find(|(_i, el)| **el == rule.0);
        let second_number_index_option = update.iter().enumerate().find(|(_i, el)| **el == rule.1);

        // Rule does not apply
        if first_number_index_option.is_none() || second_number_index_option.is_none() {
            continue;
        }

        let (first_number_index, _) = first_number_index_option.unwrap();
        let (second_number_index, _) = second_number_index_option.unwrap();

        let rule_conformant = second_number_index > first_number_index;

        if !rule_conformant {
            // Fix according to rule
            let aux = update[first_number_index];
            update[first_number_index] = update[second_number_index];
            update[second_number_index] = aux;
        }
    }

    while !check_safe(update, rules) {
        fix_update(update, rules);
    }
}

pub fn day5(input: &Path, result: &Path) {
    let lines: Vec<String> = read_to_string(input) 
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    // See problem description in AoC
    let mut scanning_second_section = false;
    let mut raw_rules: Vec<String> = [].to_vec();
    let mut raw_updates: Vec<String> = [].to_vec();
    for line in lines {
        if line == "" {
            scanning_second_section = true;
        } else if scanning_second_section {
            raw_updates.push(line.to_string());
        } else {
            raw_rules.push(line.to_string());
        }
    }

    // Parse rules
    let rules = raw_rules.iter()
        // Split lines by '|' and transform to numbers
        .map(|s| s.split('|')
            .map(String::from)
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
        )
        // Use pairs instead of vectors
        .map(|v: Vec<i32>| -> (i32, i32) { return (*v.get(0).unwrap(), *v.get(1).unwrap()) })
        .collect::<Vec<(i32, i32)>>();

    // Parse updates
    let updates = raw_updates.iter()
        .map(|s| s.split(',')
            .map(String::from)
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
        ).collect::<Vec<Vec<i32>>>();
    
    let mut safe_update;
    let mut result_acc = 0;
    let mut result_acc_2 = 0;
    for update in &updates {
        safe_update = check_safe(update, &rules);


        if safe_update {
            let middle_index = update.len() / 2;
            result_acc += update.get(middle_index).unwrap();
        } else {
            let mut fixed_update = update.clone();
            fix_update(&mut fixed_update, &rules);
            let middle_index = fixed_update.len() / 2;
            result_acc_2 += fixed_update.get(middle_index).unwrap();
        }
    }


    // Write to output file
    let mut file = File::create(result).expect("Unable to create file");
    let text: String = format!("Solution to part 1: {}\nSolution to part 2: {}", result_acc.to_string(), result_acc_2.to_string());
    file.write_all(text.as_bytes()).unwrap();
}