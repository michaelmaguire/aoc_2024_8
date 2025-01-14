use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

struct Matrix {
    data: Vec<Vec<char>>,
}

impl Matrix {
    fn new(data: Vec<Vec<char>>) -> Self {
        Matrix { data }
    }

    fn from_file(file_path: &str) -> io::Result<Self> {
        let path = Path::new(file_path);
        let file = File::open(&path)?;
        let reader = io::BufReader::new(file);

        let mut result = Vec::new();
        for line in reader.lines() {
            let line = line?;
            result.push(line.chars().collect());
        }
        Ok(Matrix::new(result))
    }

    fn width(&self) -> usize {
        self.data.first().map_or(0, |row| row.len())
    }
    fn height(&self) -> usize {
        self.data.len()
    }
    fn check_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width() && y < self.height()
    }
    fn print(&self) {
        for row in &self.data {
            for &ch in row {
                print!("{}", ch);
            }
            println!();
        }
    }

    fn char_positions(&self) -> HashMap<char, Vec<(usize, usize)>> {
        let mut positions = HashMap::new();
        for (y, row) in self.data.iter().enumerate() {
            for (x, &ch) in row.iter().enumerate() {
                if ch != '.' {
                    positions.entry(ch).or_insert_with(Vec::new).push((x, y));
                }
            }
        }
        positions
    }

}

impl Matrix {
    fn coordinate_difference(&self, coord1: (usize, usize), coord2: (usize, usize)) -> (isize, isize) {
        let x_diff = coord1.0 as isize - coord2.0 as isize;
        let y_diff = coord1.1 as isize - coord2.1 as isize;
        (x_diff, y_diff)
    }
    fn add_difference(&self, coord: (usize, usize), diff: (isize, isize)) -> Option<(usize, usize)> {
        let new_x = coord.0 as isize + diff.0;
        let new_y = coord.1 as isize + diff.1;

        if new_x >= 0 && new_y >= 0 {
            let new_x = new_x as usize;
            let new_y = new_y as usize;
            if self.check_bounds(new_x, new_y) {
                return Some((new_x, new_y));
            }
        }
        None
    }
    fn subtract_difference(&self, coord: (usize, usize), diff: (isize, isize)) -> Option<(usize, usize)> {
        let new_x = coord.0 as isize - diff.0;
        let new_y = coord.1 as isize - diff.1;
    
        if new_x >= 0 && new_y >= 0 {
            let new_x = new_x as usize;
            let new_y = new_y as usize;
            if self.check_bounds(new_x, new_y) {
                return Some((new_x, new_y));
            }
        }
        None
    }
}

fn main() {
    println!("Hello, aoc_2024_8!");
    match Matrix::from_file("./src/input.txt") {
        Ok(mut matrix) => {
            matrix.print();
            let positions = matrix.char_positions();
            //for (ch, pos) in &positions {
            //    println!("{}: {:?}", ch, pos);
            //}


            let mut antinode_positions = std::collections::HashSet::new();

            for (_ch, positions) in &positions {
                for i in 0..positions.len() {
                    for j in i + 1..positions.len() {
                        let pos1 = positions[i];
                        let pos2 = positions[j];
                        //println!("Pair for {}: ({:?}, {:?})", ch, pos1, pos2);

                        let diff = matrix.coordinate_difference(pos1, pos2);
                        //println!("Difference: {:?}", diff);

                        if let Some(add_result) = matrix.add_difference(pos1, diff) {
                            //println!("Add result: {:?}", add_result);
                            antinode_positions.insert(add_result);
                        }

                        if let Some( sub_result) = matrix.subtract_difference(pos2, diff) {
                            //println!("Sub result: {:?}", sub_result);
                            antinode_positions.insert(sub_result);
                        }
                    }
                }
            }

            println!("Antinode positions: {:?}", antinode_positions);
            println!("Number of antinode positions: {}", antinode_positions.len());

            for &(x, y) in &antinode_positions {
                if matrix.check_bounds(x, y) {
                    matrix.data[y][x] = '#';
                }
            }

            println!("Updated matrix:");
            matrix.print();

        },
        Err(e) => eprintln!("Error reading input file: {}", e),
    }
}