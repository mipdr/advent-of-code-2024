mod days;

use std::env;
use std::path::Path;

use days::day1::day1;
use days::day2::day2;
use days::day3::day3;
use days::day4::day4;


fn main() {

    let aoc_run_day_env_var = env::var("AOC_RUN_DAY");
    let mut aoc_run_day: i32 = 0;
    let mut run_all: bool = false;

    if aoc_run_day_env_var.is_ok() {
        aoc_run_day = aoc_run_day_env_var.unwrap().parse::<i32>().unwrap();

        if aoc_run_day < 1 || aoc_run_day > 24 {
            panic!("Invalid AOC_RUN_DAY variable");
        }
    } else {
        run_all = true;
    }

    if aoc_run_day == 1 || run_all {
        day1(Path::new("data/input_1_sample.txt"), Path::new("solutions/output_1_sample.txt"));
        day1(Path::new("data/input_1.txt"), Path::new("solutions/output_1.txt"));
    }

    if aoc_run_day == 2 || run_all {
        day2(Path::new("data/input_2_sample.txt"), Path::new("solutions/output_2_sample.txt"));
        day2(Path::new("data/input_2.txt"), Path::new("solutions/output_2.txt"));
    }

    if aoc_run_day == 3 || run_all {
        day3(Path::new("data/input_3_sample.txt"), Path::new("solutions/output_3_sample.txt"));
        day3(Path::new("data/input_3.txt"), Path::new("solutions/output_3.txt"));
    }

    if aoc_run_day == 4 || run_all {
        day4(Path::new("data/input_4_sample.txt"), Path::new("solutions/output_4_sample.txt"));
        day4(Path::new("data/input_4.txt"), Path::new("solutions/output_4.txt"));
    }
}
