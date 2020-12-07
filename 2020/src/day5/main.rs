#[path = "../common.rs"]
mod common;

fn main() {
    let rows = 127;
    let columns = 7;
    let mut max_seat_id = 0;
    if let Ok(lines) = self::common::read_lines("./src/day5/input.txt") {
        for line in lines {
            if let Ok(boardingpassline) = line {
                let mut seatrows = (0, rows);
                let mut seatcols = (0, columns);
                for c in boardingpassline.chars() {
                    match c {
                        'F' => {
                            let diff = seatrows.1 - seatrows.0;
                            seatrows = ( seatrows.0, seatrows.1 - (diff as f32 / 2.0).ceil() as i32 ) 
                        }
                        'B' => {
                            let diff = seatrows.1 - seatrows.0;
                            seatrows = ( seatrows.1 - (diff as f32 / 2.0).floor() as i32, seatrows.1 ) 
                        }
                        'L' => {
                            let diff = seatcols.1 - seatcols.0;
                            seatcols = ( seatcols.0, seatcols.1 - (diff as f32 / 2.0).ceil() as i32 ) 
                        }
                        'R' => {  
                            let diff = seatcols.1 - seatcols.0;
                            seatcols = ( seatcols.1 - (diff as f32 / 2.0).floor() as i32, seatcols.1 )
                        }

                        _ => println!("Upps")
                    }
                }

                let seatid = seatrows.0 * 8 + seatcols.0;
                if seatid > max_seat_id {
                    max_seat_id = seatid;
                }
            }
        }
    }

    println!("Max Seat ID: {}", max_seat_id);
}
