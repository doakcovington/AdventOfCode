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