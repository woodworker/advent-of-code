#[path = "../common.rs"]
mod common;

use std::str::FromStr;

fn main() {
    let mut counter = 0;
    if let Ok(lines) = self::common::read_lines("./src/day2/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(passwordline) = line {
                match PolicedPassword::from_str(&passwordline) {
                    Ok(p) => {
                        if p.is_valid() {
                            counter = counter+1;
                        }
                    }
                    _ => println!("Could not parse {}", passwordline),
                }

            }
        }
    }
    println!("Valid Passwords: {}", counter);
}

#[derive(Debug)]
struct PolicedPassword {
    min: i32,
    max: i32,
    letter: char,
    password: String,
}

impl PolicedPassword {
    fn is_valid(&self) -> bool {
        let letter_count = self.password.matches(self.letter).count() as i32;

        letter_count >= self.min && letter_count <= self.max
    }
}

impl FromStr for PolicedPassword {
    type Err = ();

    fn from_str(password_line: &str) -> Result<Self, Self::Err> {
        let mainparts: Vec<&str> = password_line.split(": ").collect();
        if mainparts.len() != 2 {
            ()
        }

        let policyparts: Vec<&str> = mainparts[0].split(" ").collect();
        if policyparts.len() != 2 {
            ()
        }

        let min_max = policyparts[0].split("-").collect::<Vec<&str>>();
        if min_max.len() != 2 {
            ()
        }

        Ok(PolicedPassword{
            min: min_max[0].parse().unwrap(),
            max: min_max[1].parse().unwrap(),
            letter: policyparts[1].chars().next().unwrap(),
            password: mainparts[1].to_string()
        })
    }
}


#[cfg(test)]
mod tests {
    use crate::PolicedPassword;
    use std::str::FromStr;

    #[test]
    fn is_valid() {
        match PolicedPassword::from_str("1-3 a: abcde") {
            Ok(p) => assert_eq!(p.is_valid(), true),
            Err(_) => assert!(false),
        }
    }
    #[test]
    fn is_invalid() {
        match PolicedPassword::from_str("1-3 f: abcde") {
            Ok(p) => assert_eq!(p.is_valid(), false),
            Err(_) => assert!(false),
        }
    }
}
