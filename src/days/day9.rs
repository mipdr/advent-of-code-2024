use std::{
    fs::{read_to_string, File},
    io::Write,
    path::Path,
};

pub fn day9(input: &Path, result: &Path) {
    let input: String = read_to_string(input).unwrap();

    // Expand the memory map
    let mut expanded_memory_map: Vec<i64> = vec![];
    let mut current_number: i64;
    let mut current_expanded_memory_symbol: i64;
    for (i, c) in input.chars().enumerate() {
        current_number = c.to_string().parse::<i64>().unwrap();

        if i % 2 == 0 {
            current_expanded_memory_symbol = i as i64 / 2;
        } else {
            current_expanded_memory_symbol = -1;
        }

        for _ in 0..current_number {
            expanded_memory_map.push(current_expanded_memory_symbol);
        }
    }

    let mut expanded_iter_fw = expanded_memory_map.iter().enumerate();
    let mut expanded_iter_bw = expanded_memory_map.iter().enumerate().rev();
    let mut expanded_iter_fw_item: (usize, &i64) = expanded_iter_fw.next().unwrap();
    let mut expanded_iter_bw_item: (usize, &i64) = expanded_iter_bw.next().unwrap();

    let mut finished = false;

    let mut checksum: i64 = 0;

    while !finished {
        if expanded_iter_fw_item.0 > expanded_iter_bw_item.0 {
            finished = true;
            continue;
        }

        if *expanded_iter_fw_item.1 != -1 {
            checksum += *expanded_iter_fw_item.1 * expanded_iter_fw_item.0 as i64;
            expanded_iter_fw_item = expanded_iter_fw.next().unwrap();
            continue;
        }

        if *expanded_iter_bw_item.1 == -1 {
            expanded_iter_bw_item = expanded_iter_bw.next().unwrap();
            continue;
        }

        // Swap values, accumulate in checksum and proceed
        checksum += *expanded_iter_bw_item.1 * expanded_iter_fw_item.0 as i64;
        expanded_iter_fw_item = expanded_iter_fw.next().unwrap();
        expanded_iter_bw_item = expanded_iter_bw.next().unwrap();
    }

    // Write to output file
    let mut file = File::create(result).expect("Unable to create file");
    let text: String = format!(
        "Solution to part 1: {}\nSolution to part 2: TO DO",
        checksum.to_string()
    );
    file.write_all(text.as_bytes()).unwrap();
}
