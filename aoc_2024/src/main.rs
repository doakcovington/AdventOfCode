// WDGTR

use std::fs::File;
use std::io::{self, BufRead, BufReader};

// fn main() -> io::Result<()> {
//     // // Create two empty vecs for input lists
//     // let mut first_list: Vec<i32> = Vec::new();
//     // let mut second_list: Vec<i32> = Vec::new();

//     let file = File::open("/Users/doak/Code/AdventOfCode/aoc_2024/puzzle_inputs/day2_part1.txt")?;
//     let reader = BufReader::new(file);

//     // Iterate through lines
//     for line in reader.lines() {
//         // Handle potential line reading error
//         let line = line?;
        
//         // Split the line into words
//         let words: Vec<&str> = line.split_whitespace().collect();
        
//         // Check if the line has at least two words
//         if words.len() >= 2 {
//             // Convert words to integers
//             match (words[0].parse::<i32>(), words[1].parse::<i32>()) {
//                 (Ok(first), Ok(second)) => {
//                     first_list.push(first);
//                     second_list.push(second);
//                 },
//                 _ => {
//                     // Optional: Handle parsing errors
//                     println!("Could not parse line: {}", line);
//                 }
//             }
//         }
//     }


//     // Sort both arrays from smallest to largest
//     first_list.sort();
//     second_list.sort();
//     let mut puzzle_count = 0;

//     // Add values to count
//     let mut count = 0;
//     while count < first_list.len() {
//         let mut total = 0;
//         // total = first_list[count] + second_list[count];
//         if first_list[count] >= second_list[count] {
//             total = first_list[count] - second_list[count]
//         } else {
//             total = second_list[count] - first_list[count]
//         };
//         puzzle_count += total;
//         total = 0;
//         count += 1;
//     }
//     Ok(())
// }

// fn part_two(list1: Vec<i32>, list2: Vec<i32>) {
//     let mut first_count = 0;
//     let mut total = 0;
//     // loop through each index in the first array
//     while first_count < list1.len() {
//         // check each number in the second array
//         let mut second_count = 0;
//         let mut number = list1[first_count];
//         let mut multiplier = 0;
//         while second_count < list2.len() {
//             // check if the current index of first array matches current index of second array
//             if list1[first_count] == list2[second_count] {
//                 // add 1 to multiplier
//                 multiplier += 1;
//                 second_count += 1;
//             } else {
//                 second_count += 1;
//             }
//         }
//         total += multiplier * number;
//         second_count = 0;
//         first_count += 1;
//     }
//     println!("Total: {}", total);
// }

fn main() {
    let test_sequences: Vec<Vec<i32>> = vec![
        vec![7, 6, 4, 2, 1],     // One larger jump allowed
        vec![1, 2, 7, 8, 9],     // One non-increasing number
        vec![9, 7, 6, 2, 1],     // Multiple violations
        vec![1, 3, 2, 4, 5],     // Perfect increasing sequence
        vec![8, 6, 4, 4, 1],     // Decreasing sequence
        vec![1, 3, 6, 7, 9],     // Near-decreasing with one violation
    ];

    let mut safe_count = 0;

    for sequence in test_sequences {
        println!("\nOriginal Sequence: {:?}", sequence);
        let processed_sequence = process_sequence(sequence);
        println!("Final Sequence: {:?}", processed_sequence);
        println!("---");
    }

    // More complex example with repeated processing
    println!("\nRepeated Processing Example:");
    let mut complex_sequence = vec![1, 2, 4, 7, 10, 8];
    println!("Starting sequence: {:?}", complex_sequence);

    while complex_sequence.windows(2)
        .any(|window| i32::abs(window[1] - window[0]) > 3 || window[1] <= window[0]) 
    {
        complex_sequence = process_sequence(complex_sequence);
    }

    println!("Final constrained sequence: {:?}", complex_sequence);
}

fn process_sequence(mut sequence: Vec<i32>) -> Vec<i32> {
    // Clone the original sequence for comparison
    let original_sequence = sequence.clone();

    // Find the first index that breaks the constraints
    let break_index = sequence.windows(2)
        .enumerate()
        .find_map(|(index, window)| {
            let difference = i32::abs(window[1] - window[0]);
            
            // Check conditions:
            // 1. Not increasing or decreasing
            // 2. Difference greater than 3
            if difference > 3 || 
               (window[1] <= window[0]) 
            {
                Some(index + 1) // Return the index to remove
            } else {
                None // Continue if no constraint is broken
            }
        });

    // Remove the problematic index if found
    if let Some(index) = break_index {
        sequence.remove(index);
        println!("Removed index {} from {:?}", index, original_sequence);
        println!("New sequence: {:?}", sequence);
    }

    sequence
}