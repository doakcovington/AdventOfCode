fn day_2_part_1 () {
  // set safe_count to 0
  let mut safe_count = 0;

  // read input from file
  let file = File::open("/Users/doak/Code/AdventOfCode/aoc_2024/puzzle_inputs/day2_part1.txt")?;
  let reader = BufReader::new(file);

  // convert each line into an array
  let mut reports: Vec<Vec<i32>> = Vec::new();
  
  for line in reader.lines() {
    // Convert each line to a vector of numbers
    let numbers: Vec<i32> = line?
        .split_whitespace()  // Split line by whitespace
        .filter_map(|s| s.parse().ok()) // Parse each substring to i32, filtering out any parse errors
        .collect();

    // Only add non-empty arrays
    if !numbers.is_empty() {
        reports.push(numbers);
    }
    for (seq_index, sequence) in reports.iter().enumerate() {
      // Check if differences are within 3 AND sequence is monotonic
      let max_difference_check = sequence.windows(2)
          .all(|window| (window[1] - window[0]).abs() <= 3);
      
      // Check for increasing
      let is_increasing = sequence.windows(2)
          .all(|window| window[1] > window[0]);
      
      // Check for decreasing
      let is_decreasing = sequence.windows(2)
          .all(|window| window[1] < window[0]);
      
      // Detailed differences
      let differences: Vec<_> = sequence.windows(2)
          .map(|window| window[1] - window[0])
          .collect();
      
      println!("Sequence {}:", seq_index);
      println!("  Original:    {:?}", sequence);
      println!("  Differences: {:?}", differences);
      
      // Comprehensive analysis
      match (is_increasing, is_decreasing, max_difference_check) {
          (true, false, true) => safe_count += 1,
          (false, true, true) => safe_count += 1,
          _ => println!("Not a safe report"),
      }
  }
  
}
  