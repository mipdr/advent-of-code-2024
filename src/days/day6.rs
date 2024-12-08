use std::{fs::{read_to_string, File}, io::Write, path::Path};

#[derive(Copy, Clone)]
enum Directions {
    Up,
    Down,
    Right,
    Left
}

fn turn_90_degrees_right(direction: Directions) -> Directions {
    match direction {
        Directions::Down => Directions::Left,
        Directions::Left => Directions::Up,
        Directions::Up => Directions::Right,
        Directions::Right => Directions::Down,
    }
}

fn step_forward(nr: usize, nc: usize, direction: Directions) -> (usize, usize) {
    let mut next_nr = nr;
    let mut next_nc = nc;
    match direction {
        Directions::Down => next_nr = nr + 1,
        Directions::Left => next_nc = nc - 1,
        Directions::Up => next_nr = nr - 1,
        Directions::Right => next_nc = nc + 1,
    }

    return (next_nr, next_nc);
}

fn check_increment_out_of_bounds(nr: usize, nc: usize, nrows: usize, ncols: usize, direction: Directions) -> bool {
    match direction {
        Directions::Down => nr + 1 == nrows,
        Directions::Left => nc == 0,
        Directions::Up => nr == 0,
        Directions::Right => nc +1 == ncols,
    }
}

fn replace_nth_char(s: &str, idx: usize, newchar: char) -> String {
    s.chars().enumerate().map(|(i,c)| if i == idx { newchar } else { c }).collect()
}

pub fn day6(input: &Path, result: &Path) {
    let mut lines: Vec<String> = read_to_string(input) 
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let ncols = lines[0].len();
    let nrows = lines.len();

    let starting_position = lines
        .iter()
        .enumerate()
        .find_map(|(rn, row)| row
            .chars()
            .enumerate()
            .find_map(|(cn, c): (usize, char)| -> Option<(usize, usize)> {
                if c == '^' {
                    return Some((rn, cn));
                } else {
                    return None;
                }
            })
        )
        .unwrap();

    let mut exited = false;
    let mut nr: usize = starting_position.0;
    let mut nc: usize = starting_position.1;
    let mut next_nr: usize;
    let mut next_nc: usize;
    let mut direction = Directions::Up;
    // Forward in time
    while !exited {

        if check_increment_out_of_bounds(nr, nc, nrows, ncols, direction) {
            exited = true;
        } else {
            (next_nr, next_nc) = step_forward(nr, nc, direction);

            if lines[next_nr].chars().nth(next_nc).unwrap() == '#' {
                direction = turn_90_degrees_right(direction);
                (next_nr, next_nc) = step_forward(nr, nc, direction);
            }

            lines[nr] = replace_nth_char(&lines[nr], nc, 'X');
            lines[next_nr] = replace_nth_char(&lines[next_nr], next_nc, 'º');
            (nr, nc) = (next_nr, next_nc);
        }

        // Print for debugging
        // for line in &lines {
        //     println!("{}", line);
        // }
        // println!("\n");
    }

    // for line in &lines {
    //     println!("{}", line);
    // }
    // println!("\n");

    let count = lines
        .iter()
        .map(|line| line.chars().filter(|c| *c == 'X').count())
        .reduce(|a, b| a + b)
        .unwrap() + 1; // +1 for initial position that is not counted

    // Write to output file
    let mut file = File::create(result).expect("Unable to create file");
    let text: String = format!("Solution to part 1: {}\nSolution to part 2: TO DO", count.to_string());
    file.write_all(text.as_bytes()).unwrap();


}