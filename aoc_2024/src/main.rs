// WDGTR

use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // Create two empty vecs for input lists
    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();

    let file = File::open("/Users/doak/Code/AdventOfCode/aoc_2024/puzzle_inputs/day1_part2.txt")?;
    let reader = BufReader::new(file);

    // Iterate through lines
    for line in reader.lines() {
        // Handle potential line reading error
        let line = line?;
        
        // Split the line into words
        let words: Vec<&str> = line.split_whitespace().collect();
        
        // Check if the line has at least two words
        if words.len() >= 2 {
            // Convert words to integers
            match (words[0].parse::<i32>(), words[1].parse::<i32>()) {
                (Ok(first), Ok(second)) => {
                    first_list.push(first);
                    second_list.push(second);
                },
                _ => {
                    // Optional: Handle parsing errors
                    println!("Could not parse line: {}", line);
                }
            }
        }
    }


    // Sort both arrays from smallest to largest
    first_list.sort();
    second_list.sort();
    let mut puzzle_count = 0;

    // Add values to count
    let mut count = 0;
    while count < first_list.len() {
        let mut total = 0;
        // total = first_list[count] + second_list[count];
        if first_list[count] >= second_list[count] {
            total = first_list[count] - second_list[count]
        } else {
            total = second_list[count] - first_list[count]
        };
        puzzle_count += total;
        total = 0;
        count += 1;
    }
   
    println!("puzzle count: {:?}", puzzle_count);
    part_two(first_list, second_list);
    Ok(())
}

fn part_two(list1: Vec<i32>, list2: Vec<i32>) {
    let mut first_count = 0;
    let mut total = 0;
    // loop through each index in the first array
    while first_count < list1.len() {
        // check each number in the second array
        let mut second_count = 0;
        let mut number = list1[first_count];
        let mut multiplier = 0;
        while second_count < list2.len() {
            // check if the current index of first array matches current index of second array
            if list1[first_count] == list2[second_count] {
                // add 1 to multiplier
                multiplier += 1;
                second_count += 1;
            } else {
                second_count += 1;
            }
        }
        total += multiplier * number;
        second_count = 0;
        first_count += 1;
    }
    println!("Total: {}", total);
}