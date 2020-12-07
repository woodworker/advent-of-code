
#[path = "../common.rs"]
mod common;

fn main() {
    let mut nums: Vec<i32> = vec![];

    if let Ok(lines) = self::common::read_lines("./src/day1/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(num) = line {
                nums.push(num.parse::<i32>().unwrap());
            }
        }
    }

    'outer: for num in nums.clone() {
        for innernum in nums.clone() {
            if num + innernum == 2020 {
                println!("{} + {} = 2020", num, innernum);
                let result = num * innernum;
                println!("{} * {} = {}", num, innernum, result);
                break 'outer;
            }
        }
    }
}
