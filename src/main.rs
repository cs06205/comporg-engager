use std::fs;
use serde_json;
use rand::Rng;
use std::env;
use std::process;

fn main() {
    if env::args().len() == 2 {
        if let Some(arg) = env::args().nth(1) {            
            let data = fs::read_to_string(arg)
            .expect("Unable to read file.");

            let json: serde_json::Value = serde_json::from_str(&data)
            .expect("Json error.");

            let mut students_present: Vec<String> = Vec::new();

            if let Some(array) = json.as_array() {
                for student in array {
                    if student["present"] == true {
                        students_present.push(student["name"].to_string().trim_matches('\"').to_string());
                    }
                }
            }

            let lucky_one = rand::thread_rng().gen_range(0..students_present.len());
            println!("Lucky One: {}", students_present[lucky_one]);
            
        } else {
            println!("UNEXPECTED BEHAVIOR: Please tell the developer that Bug 0d7c3528509aeacb occurred.");
            process::exit(2);
        }
    } else {
        println!("USAGE: rowan-engager <CLASS_JSON_FILE_PATH>");
        process::exit(1);
    }
}
