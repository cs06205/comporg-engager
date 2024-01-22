use std::fs;
use serde_json;
use rand::Rng;
use std::env;
use std::process;
use rand::seq::SliceRandom;

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
    } else if env::args().len() == 3 {
        if let Some(arg) = env::args().nth(1) {
            if arg == "groups" {
                if let Some(arg) = env::args().nth(2) {
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

                    students_present.shuffle(&mut rand::thread_rng());

                    
                    let (mut counter, mut group_counter) = (0, 1);
                    
                    let n = students_present.len();

                    for student in students_present {
                        if (n-counter) >= 2 && counter % 3 == 0 {
                            println!("\nGROUP {}:", group_counter);
                            group_counter += 1;
                        }

                        println!("{}", student);
                        counter += 1;
                    }
                } else {
                    println!("UNEXPECTED BEHAVIOR: Please tell the developer that Bug bb882e3d1bee97da occurred.");
                    process::exit(2);
                }
            } else {
                println!("USAGE: comporg-engager [groups] <CLASS_JSON_FILE_PATH>");
                process::exit(1);
            }
        } else {
            println!("UNEXPECTED BEHAVIOR: Please tell the developer that Bug c6c614a3a55ebbfc occurred.");
            process::exit(2);
        }

    } else {
        println!("USAGE: comporg-engager [groups] <CLASS_JSON_FILE_PATH>");
        process::exit(1);
    }
}
