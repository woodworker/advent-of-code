#[path = "../common.rs"]
mod common;

use std::collections::HashMap;

fn main() {

    let mut passports: Vec<HashMap<String, String>> = vec![];
    if let Ok(lines) = self::common::read_lines("./src/day4/input.txt") {
        // Consumes the iterator, returns an (Optional) String

        let mut passport: HashMap<String, String> = HashMap::new();
        for line in lines {
            if let Ok(passportline) = line {
                if passportline == "" {
                    passports.push(passport);
                    passport = HashMap::new();
                } else {
                    for item in passportline.split(" ") {
                        let pair = item.split(":").collect::<Vec<&str>>();
                        if pair.len()==2 {
                            // let key = PassportKey::from_str(pair[0]);
                            passport.insert(pair[0].to_string(), pair[1].to_string());
                        }
                    }
                }
            }
        }
        passports.push(passport);
    }

    let mut valid = 0;
    for passport in passports {
        if (passport.len() == 7 && !passport.contains_key("cid")) || passport.len()==8 {
            valid = valid + 1;
        }
    }

    println!("Valid passports: {:}", valid);
}
