use std::fs::File;
use std::io::{BufRead, BufReader};

// "test_inputs" or "real_inputs"
const INPUT_FILE_PATH: &str = "real_inputs/day2.txt";

fn main() {
    let input_file_lines =
        BufReader::new(File::open(INPUT_FILE_PATH).expect("Could not open input file!")).lines();

    let mut reports: Vec<Vec<i32>> = Vec::new();

    for input_line in input_file_lines {
        if let Ok(input_line) = input_line {
            // Split the string, parse the "levels" and put them into a list
            let levels: Vec<i32> = input_line
                .split(" ")
                .map(|x| x.parse::<i32>().expect("Could not parse integer!"))
                .collect();

            reports.push(levels);
        } else {
            panic!("Could not read input line. Exiting!");
        }
    }

    // Part 1 - Count safe reports
    let mut safe_reports = 0;
    for report in reports {
        //Check if report is increasing or decreasing
        let is_safe;
        if report[0] < report[1] {
            is_safe = is_all_increasing(&report);
        } else {
            is_safe = is_all_decreasing(&report);
        }

        if is_safe {
            safe_reports += 1;
        }
    }

    println!("Safe Reports count: {}", safe_reports);
}

fn is_all_increasing(report: &Vec<i32>) -> bool {
    for i in 1..report.len() {
        let diff = report[i] - report[i - 1];

        // not safe if diff<1 or diff>3
        if (diff < 1) | (diff > 3) {
            return false;
        }
    }

    return true;
}
fn is_all_decreasing(report: &Vec<i32>) -> bool {
    for i in 1..report.len() {
        let diff = report[i - 1] - report[i];

        // not safe if diff<1 or diff>3
        if (diff < 1) | (diff > 3) {
            return false;
        }
    }

    return true;
}
