use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
struct WordLocation {
    row: usize,
    col: usize,
    direction: String,
}

fn find_xmas(grid: &Vec<Vec<char>>) -> HashSet<WordLocation> {
    let word = "xmas";
    let rows = grid.len();
    let cols = grid[0].len();
    let mut found_locations = HashSet::new();

    // Directions: right, left, down, up, diagonal down-right, diagonal down-left, 
    // diagonal up-right, diagonal up-left
    let directions = [
        (0, 1, "right"),
        (0, -1, "left"),
        (1, 0, "down"),
        (-1, 0, "up"),
        (1, 1, "down-right"),
        (1, -1, "down-left"),
        (-1, 1, "up-right"),
        (-1, -1, "up-left"),
    ];

    for start_row in 0..rows {
        for start_col in 0..cols {
            for &(row_delta, col_delta, dir_name) in &directions {
                // Check forward
                if let Some(location) = check_word_forward(grid, word, start_row, start_col, row_delta, col_delta, dir_name) {
                    found_locations.insert(location);
                }
                
                // Check backward
                if let Some(location) = check_word_forward(grid, &word.chars().rev().collect::<String>(), start_row, start_col, row_delta, col_delta, &format!("reverse-{}", dir_name)) {
                    found_locations.insert(location);
                }
            }
        }
    }

    found_locations
}

fn check_word_forward(
    grid: &Vec<Vec<char>>, 
    word: &str, 
    start_row: usize, 
    start_col: usize, 
    row_delta: i32, 
    col_delta: i32, 
    dir_name: &str
) -> Option<WordLocation> {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    
    // Check if the entire word fits within the grid
    let end_row = start_row as i32 + (word.len() as i32 - 1) * row_delta;
    let end_col = start_col as i32 + (word.len() as i32 - 1) * col_delta;
    
    if end_row < 0 || end_row >= rows || end_col < 0 || end_col >= cols {
        return None;
    }

    // Check if word matches
    for (i, ch) in word.chars().enumerate() {
        let current_row = start_row as i32 + (i as i32 * row_delta);
        let current_col = start_col as i32 + (i as i32 * col_delta);
        
        if current_row < 0 || current_row >= rows || 
           current_col < 0 || current_col >= cols ||
           grid[current_row as usize][current_col as usize].to_ascii_lowercase() != ch.to_ascii_lowercase() {
            return None;
        }
    }

    Some(WordLocation {
        row: start_row,
        col: start_col,
        direction: dir_name.to_string(),
    })
}

fn read_grid_from_file(filename: &str) -> Result<Vec<Vec<char>>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    
    let grid: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    
    Ok(grid)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = "puzzle_inputs/day4_part1.txt";  // Replace with your file path
    let grid = read_grid_from_file(filename)?;

    let xmas_locations = find_xmas(&grid);

    if xmas_locations.is_empty() {
        println!("No instances of 'xmas' found.");
    } else {
        println!("Found {} instances of 'xmas':", xmas_locations.len() / 2);
        for location in &xmas_locations {
            println!(
                "Found at row {}, column {} going {}",
                location.row, location.col, location.direction
            );
        }
    }

    Ok(())
}
