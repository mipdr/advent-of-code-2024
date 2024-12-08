use std::{
    collections::{HashMap, HashSet},
    fs::{read_to_string, File},
    io::Write,
    path::Path,
};

use itertools::Itertools;

fn get_antinodes(
    antennas: &Vec<(usize, usize)>,
    map_size: &(i32, i32),
    antinodes: &mut HashSet<(i32, i32)>,
    antinodes_part_2: &mut HashSet<(i32, i32)>,
) {
    let antenna_combinations = antennas
        .iter()
        .cartesian_product(antennas.iter())
        .filter(|(a, b)| a != b);

    let mut antinode1: (i32, i32);
    let mut antinode2: (i32, i32);
    let mut a1x;
    let mut a1y;
    let mut a2x;
    let mut a2y;
    for antenna_combination in antenna_combinations {
        a1x = antenna_combination.0 .0 as i32;
        a1y = antenna_combination.0 .1 as i32;
        a2x = antenna_combination.1 .0 as i32;
        a2y = antenna_combination.1 .1 as i32;

        // PART 1

        // For antennas A and B, antinodes are 2A - B and 2B - A
        antinode1 = (2 * a1x - a2x, 2 * a1y - a2y);
        antinode2 = (2 * a2x - a1x, 2 * a2y - a1y);

        if antinode1.0 >= 0
            && antinode1.0 < map_size.0
            && antinode1.1 >= 0
            && antinode1.1 < map_size.1
        {
            antinodes.insert(antinode1);
        }
        if antinode2.0 >= 0
            && antinode2.0 < map_size.0
            && antinode2.1 >= 0
            && antinode2.1 < map_size.1
        {
            antinodes.insert(antinode2);
        }

        // PART 2

        // For part 2, antinodes are in A + n(B - A) and B + n(A - B)

        antinode1 = (a1x, a1y);
        antinode2 = (a2x, a2y);

        while antinode1.0 >= 0
            && antinode1.0 < map_size.0
            && antinode1.1 >= 0
            && antinode1.1 < map_size.1 
        {
            antinodes_part_2.insert(antinode1);
            antinode1.0 += a2x - a1x;
            antinode1.1 += a2y - a1y;
        }

        while antinode2.0 >= 0
            && antinode2.0 < map_size.0
            && antinode2.1 >= 0
            && antinode2.1 < map_size.1 
        {
            antinodes_part_2.insert(antinode2);
            antinode2.0 += a1x - a2x;
            antinode2.1 += a1y - a2y;
        }
    }
}

pub fn day8(input: &Path, result: &Path) {
    let lines: Vec<String> = read_to_string(input)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let map: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let row_size = map[0].len();
    let column_size = map.len();
    let map_size: (i32, i32) = (row_size as i32, column_size as i32);

    for line in &map {
        assert_eq!(line.len(), row_size);
    }

    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (i, line) in map.iter().enumerate() {
        for (j, _c) in line.iter().enumerate() {
            let mut antenna_key = map[i][j];
            if antenna_key != '.' {
                if antennas.contains_key(&antenna_key) {
                    antennas.get_mut(&mut antenna_key).unwrap().push((i, j));
                } else {
                    antennas.insert(antenna_key, vec![(i, j)]);
                }
            }
        }
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    let mut antinodes_part_2: HashSet<(i32, i32)> = HashSet::new();

    for (_, antennas_record) in antennas {
        get_antinodes(
            &antennas_record,
            &map_size,
            &mut antinodes,
            &mut antinodes_part_2,
        );
    }

    // Write to output file
    let mut file = File::create(result).expect("Unable to create file");
    let text: String = format!(
        "Solution to part 1: {}\nSolution to part 2: {}",
        antinodes.len().to_string(),
        antinodes_part_2.len().to_string()
    );
    file.write_all(text.as_bytes()).unwrap();
}
