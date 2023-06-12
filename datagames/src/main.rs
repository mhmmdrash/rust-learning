// HashMap
use std::collections::HashMap;

fn main() {
    let mut goals = HashMap::new();

    goals.insert("FCB".to_string(), 12);
    goals.insert("RM".to_string(), 2);

    // println!("FCB scored {} goals", goals.get("FCB").unwrap());
    // println!("RM scored {} goals", goals.get("RM").unwrap());

    for (key, val) in &goals {
        println!("{} scored {} goals", key, val);
    }
}