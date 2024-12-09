mod days;

use std::env;
use std::path::Path;

use days::day1::day1;
use days::day2::day2;
use days::day3::day3;
use days::day4::day4;
use days::day5::day5;
use days::day6::day6;
use days::day7::day7;
use days::day8::day8;
use days::day9::day9;

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
        day1(
            Path::new("data/input_1_sample.txt"),
            Path::new("solutions/output_1_sample.txt"),
        );
        day1(
            Path::new("data/input_1.txt"),
            Path::new("solutions/output_1.txt"),
        );
    }

    if aoc_run_day == 2 || run_all {
        day2(
            Path::new("data/input_2_sample.txt"),
            Path::new("solutions/output_2_sample.txt"),
        );
        day2(
            Path::new("data/input_2.txt"),
            Path::new("solutions/output_2.txt"),
        );
    }

    if aoc_run_day == 3 || run_all {
        day3(
            Path::new("data/input_3_sample.txt"),
            Path::new("solutions/output_3_sample.txt"),
        );
        day3(
            Path::new("data/input_3.txt"),
            Path::new("solutions/output_3.txt"),
        );
    }

    if aoc_run_day == 4 || run_all {
        day4(
            Path::new("data/input_4_sample.txt"),
            Path::new("solutions/output_4_sample.txt"),
        );
        day4(
            Path::new("data/input_4.txt"),
            Path::new("solutions/output_4.txt"),
        );
    }

    if aoc_run_day == 5 || run_all {
        day5(
            Path::new("data/input_5_sample.txt"),
            Path::new("solutions/output_5_sample.txt"),
        );
        day5(
            Path::new("data/input_5.txt"),
            Path::new("solutions/output_5.txt"),
        );
    }

    if aoc_run_day == 6 || run_all {
        day6(
            Path::new("data/input_6_sample.txt"),
            Path::new("solutions/output_6_sample.txt"),
        );
        day6(
            Path::new("data/input_6.txt"),
            Path::new("solutions/output_6.txt"),
        );
    }

    if aoc_run_day == 7 || run_all {
        day7(
            Path::new("data/input_7_sample.txt"),
            Path::new("solutions/output_7_sample.txt"),
        );
        day7(
            Path::new("data/input_7.txt"),
            Path::new("solutions/output_7.txt"),
        );
    }

    if aoc_run_day == 8 || run_all {
        day8(
            Path::new("data/input_8_sample.txt"),
            Path::new("solutions/output_8_sample.txt"),
        );
        day8(
            Path::new("data/input_8.txt"),
            Path::new("solutions/output_8.txt"),
        );
    }

    if aoc_run_day == 9 || run_all {
        day9(
            Path::new("data/input_9_sample.txt"),
            Path::new("solutions/output_9_sample.txt"),
        );
        day9(
            Path::new("data/input_9.txt"),
            Path::new("solutions/output_9.txt"),
        );
    }
}
