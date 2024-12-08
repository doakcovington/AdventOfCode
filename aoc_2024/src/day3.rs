fn find_mul_instances(filename: &str) -> Result<Vec<(usize, String, f64, f64)>, Box<dyn std::error::Error>> {
  // Create a regex to match mul(x,y) where x and y are numbers
  let mul_regex = Regex::new(r"mul\((-?\d+(?:\.\d+)?),\s*(-?\d+(?:\.\d+)?)\)")?;
  
  let file = File::open(filename)?;
  let reader = BufReader::new(file);

  // Vector to store matches: (line number, full match, x value, y value)
  let mut matches = Vec::new();

  // Iterate through lines with their line numbers
  for (line_number, line_result) in reader.lines().enumerate() {
      let line = line_result?;
      
      for capture in mul_regex.captures_iter(&line) {
          let x: f64 = capture[1].parse()?;
          let y: f64 = capture[2].parse()?;
          
          matches.push((
              line_number + 1,
              line.clone(),     
              x,                
              y                 
          ));
      }
  }

  Ok(matches)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let filename = "/Users/doak/Code/AdventOfCode/aoc_2024/puzzle_inputs/day3_part1.txt";
  let mut result: f64 = 0.0;
  match find_mul_instances(filename) {
      Ok(matches) => {
          if matches.is_empty() {
          } else {
              for (line_num, full_line, x, y) in matches {
                  result += x * y;
              }
          }
      }
      Err(e) => {
          eprintln!("Error searching file: {}", e);
      }
  }
  println!("Result {}", result);
  Ok(())
}

// PART 2
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

// Custom struct to track calculation state
struct CalculationState {
    should_calculate: bool,
}

fn find_conditional_mul_instances(filename: &str) -> Result<Vec<(usize, String, Option<f64>)>, Box<dyn std::error::Error>> {
    // Regex for mul(x,y) and do()/don't() commands
    let mul_regex = Regex::new(r"mul\((-?\d+(?:\.\d+)?),\s*(-?\d+(?:\.\d+)?)\)")?;
    let do_regex = Regex::new(r"do\(\)")?;
    let dont_regex = Regex::new(r"don't\(\)")?;

    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // Vector to store matches: (line number, full match, calculated result)
    let mut matches = Vec::new();
    
    // Initialize calculation state
    let mut state = CalculationState { 
        should_calculate: false 
    };

    // Iterate through lines with their line numbers
    for (line_number, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        
        // Check for do() and don't() commands first
        if do_regex.is_match(&line) {
            state.should_calculate = true;
            matches.push((
                line_number + 1, 
                line.clone(), 
                None
            ));
            continue;
        }

        if dont_regex.is_match(&line) {
            state.should_calculate = false;
            matches.push((
                line_number + 1, 
                line.clone(), 
                None
            ));
            continue;
        }

        // Find mul(x,y) instances
        for capture in mul_regex.captures_iter(&line) {
            // Parse the x and y values
            let x: f64 = capture[1].parse()?;
            let y: f64 = capture[2].parse()?;
            
            // Calculate result only if calculation is allowed
            let result = if state.should_calculate { 
                Some(x * y) 
            } else { 
                None 
            };

            matches.push((
                line_number + 1,  // Line number (1-indexed)
                line.clone(),     // Full line containing the match
                result            // Calculation result (or None)
            ));
        }
    }

    Ok(matches)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "/Users/doak/Code/AdventOfCode/aoc_2024/puzzle_inputs/day3_part2.txt";

    match find_conditional_mul_instances(filename) {
        Ok(matches) => {
            if matches.is_empty() {
                println!("No mul(x,y) instances found in the file.");
            } else {
                println!("Processed {} lines:", matches.len());
                for (line_num, full_line, result) in matches {
                    match result {
                        Some(calc_result) => {
                            println!("Line {}: {} | Calculated: {}", 
                                     line_num, full_line, calc_result);
                        },
                        None => {
                            // If do() or don't() or a mul() that wasn't calculated
                            println!("Line {}: {} | Not Calculated", 
                                     line_num, full_line);
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Error searching file: {}", e);
        }
    }

    Ok(())
}