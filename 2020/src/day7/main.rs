#[path = "../common.rs"]
mod common;

use std::collections::HashMap;

fn main() {
    let mut rules: Vec<(String, String)> = vec![];

    if let Ok(lines) = self::common::read_lines("./src/day7/input.txt") {
        for line in lines {
            if let Ok(rulesline) = line {
                let rulesplit = rulesline.split(" contain ").collect::<Vec<&str>>();
                let bagtype = rulesplit[0].replace("bags", "bag");
                for rule in rulesplit[1].split(",") {
                    let cleanrule = rule.trim().replace(".","").replace("bags", "bag");
                    if cleanrule != "no other bag" {
                        let parts = cleanrule.splitn(2, ' ').collect::<Vec<&str>>();
                        
                        rules.push((parts[1].to_string(), bagtype.to_string()));
                    }
                }
            }
        }
    }

    let mut baglist = HashMap::new();
    for rule in rules.clone() {
        if rule.0 == "shiny gold bag".to_string() {
            baglist.insert(rule.1, true);
        }
    }
    loop {
        let baglen = baglist.len();
        for rule in rules.clone() {
            if baglist.contains_key(&rule.0) {
                baglist.insert(rule.1, true);
            }
        }

        if baglist.len() == baglen {
            break;
        }
    }

    println!("count {:?}", baglist.len());
}
