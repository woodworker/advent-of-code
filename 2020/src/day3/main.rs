#[path = "../common.rs"]
mod common;

fn main() {
    let mut forrest: Vec<Vec<bool>> = vec![];
    if let Ok(lines) = self::common::read_lines("./src/day3/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(forrestline) = line {
                let mut row = vec![];
                for c in forrestline.chars() { 
                    match c {
                        '.' => row.push(false),
                        '#' => row.push(true),
                        _ => println!("ERROR"),
                    }
                }
                forrest.push(row);
            }
        }
    }

    let row_advance = 1;
    let col_advance = 3;
    let col_mod = forrest[0].len();

    let mut trees = 0;
    let mut row = 0;
    let mut col = 0;
    loop {
        row = row + row_advance;
        col = col + col_advance;

        if forrest[row][col % col_mod] == true {
            trees = trees + 1;
        }

        if row+1 >= forrest.len() {
            break;
        }
    }
    println!("Trees: {}", trees);
}
