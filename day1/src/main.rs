use std::fs::File;
use std::io::{BufRead, BufReader};

// "test_inputs" or "real_inputs"
const INPUT_FILE_PATH: &str = "test_inputs/day1.txt";
const SEPARATOR: &str = "   "; // Three spaces

fn main() {
    let file_handle = File::open(INPUT_FILE_PATH);

    let bufreader = BufReader::new(file_handle.unwrap());

    let mut line_parts: Vec<Vec<String>> = Vec::new();
    // Read all the lines and get the parts
    for line in bufreader.lines() {
        let current_line = line.unwrap().to_string();
        let current_line_parts = current_line
            .split(SEPARATOR)
            .map(|s| String::from(s))
            .collect();

        line_parts.push(current_line_parts);
    }

    // Parse the numbers on both lines and store them in two lists
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line_part in line_parts {
        //println!("{} {}", line_part[0], line_part[1]);
        let line_part_first = line_part[0].parse::<i32>().unwrap();
        let line_part_second = line_part[1].parse::<i32>().unwrap();

        list1.push(line_part_first);
        list2.push(line_part_second);
    }

    // Sort both the lists
    list1.sort();
    list2.sort();

    // Part 1
    // Find total distance according to question
    let mut total_distance = 0;

    for (index, _) in list1.iter().enumerate() {
        // println!("{} {}", index, element);

        let mut distance = list1[index] - list2[index];
        if distance < 0 {
            distance *= -1;
        }

        total_distance += distance;
    }
    println!("Total distance: {}", total_distance);

    // Part 2
    // Find total distance according to question
    let mut total_similarity = 0;
    for element in list1 {
        // Find out how many times it exists in list2
        let count_in_list2 = list2.iter().filter(|&x| *x == element).count();

        let similarity = element * (count_in_list2 as i32);

        total_similarity += similarity;
    }

    println!("Total Similarity: {}", total_similarity);
}
