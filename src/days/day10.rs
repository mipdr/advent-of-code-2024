use std::{
    collections::HashSet,
    fs::{read_to_string, File},
    io::Write,
    path::Path,
};

use itertools::Itertools;

fn find_path(trail: &Vec<(usize, usize)>, map: &Vec<Vec<u32>>) -> Vec<Vec<(usize, usize)>> {
    let row_size = map.get(0).unwrap().len();
    let column_size = map.len();

    let current_position = *trail.last().unwrap();

    let mut candidate_steps = vec![];

    if current_position.0 != 0 {
        candidate_steps.push((current_position.0 - 1, current_position.1));
    }

    if current_position.1 != 0 {
        candidate_steps.push((current_position.0, current_position.1 - 1));
    }

    if current_position.0 != column_size - 1 {
        candidate_steps.push((current_position.0 + 1, current_position.1));
    }

    if current_position.1 != row_size - 1 {
        candidate_steps.push((current_position.0, current_position.1 + 1));
    }

    return candidate_steps
        .iter()
        .filter(|candidate| {
            map[candidate.0][candidate.1] == map[current_position.0][current_position.1] + 1
        })
        .map(|candidate| {
            let mut new_trail = trail.clone();
            new_trail.push(*candidate);

            return new_trail;
        })
        .collect::<Vec<Vec<(usize, usize)>>>();
}

pub fn day10(input: &Path, result: &Path) {
    let lines: Vec<String> = read_to_string(input)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut starting_trail_positions: Vec<(usize, usize)> = vec![];

    let map: Vec<Vec<u32>> = lines
        .iter()
        .enumerate()
        .map(|(nr, line)| {
            line.chars()
                .enumerate()
                .map(|(nc, c)| {
                    let n = c.to_string().parse::<u32>().unwrap();
                    if n == 0 {
                        starting_trail_positions.push((nr, nc));
                    }
                    return n;
                })
                .collect()
        })
        .collect();

    let mut trails: Vec<Vec<(usize, usize)>> = starting_trail_positions
        .iter()
        .map(|start| vec![*start])
        .collect();

    for _ in 0..9 {
        trails = trails
            .iter()
            .flat_map(|trail| find_path(trail, &map))
            .collect()
    }

    let groups = trails.iter().chunk_by(|trail| *trail.get(0).unwrap());

    let counts: Vec<((usize, usize), HashSet<(usize, usize)>)> = groups
        .into_iter()
        .map(|(starting_position, trails)| {
            (
                starting_position,
                trails
                    .map(|t| *t.last().unwrap())
                    .collect::<HashSet<(usize, usize)>>(),
            )
        })
        .collect();

    let acc = counts
        .iter()
        .map(|(_pos, ends)| ends.len())
        .reduce(|a, b| a + b)
        .unwrap();

    // Write to output file
    let mut file = File::create(result).expect("Unable to create file");
    let text: String = format!(
        "Solution to part 1: {}\nSolution to part 2: {}",
        acc.to_string(),
        trails.len().to_string()
    );
    file.write_all(text.as_bytes()).unwrap();
}
