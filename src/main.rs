use std::{collections::HashMap, error::Error, fs};

use colored::Colorize;
use utils::{add_student, del_student, put_student_id, read_json_to_hash, write_hash_to_file};

use crate::student::Student;

mod student;
mod utils;
const help_s: &str = "
            
------------------------
1.Brower student info
2.Add student
3.Delete student info
4.Query student infomation
5.Modify student info
6.Student length
------------------------
0.exit

    ";
fn main() -> Result<(), Box<dyn Error>> {
    let mut studentArray = Vec::<Student>::new();
    let mut student_map = HashMap::<String, Student>::new();
    // Read local file infomation
    let read_result = read_json_to_hash();
    match read_result {
        Ok(v) => student_map = v,
        Err(e) => write_hash_to_file(&student_map)?,
    }
    // Output helpful infomation
    println!("{}", help_s);
    loop {
        println!("{}", "Please input some".yellow().bold());
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut input_number = input.trim().parse::<i32>().unwrap_or(-1);
        match input_number {
            // Print student infomation
            1 => {
                println!("{:#?}", student_map);
            }
            // Add student infomation
            2 => {
                let result = add_student(&mut student_map);
                match result {
                    Ok(v) => {}
                    Err(e) => {
                        eprintln!("{}", format!("{}", e).red().bold());
                        println!(
                            "{}",
                            "Input error,application will skip this input".red().bold()
                        );
                        continue;
                    }
                }
            }
            // Detele student infomation5
            3 => {
                let result = del_student(&mut student_map);
                match result {
                    Ok(_v) => {
                        println!("{}", "Deleted successfully".green().bold())
                    }
                    Err(e) => {
                        eprintln!("{}", format!("{}", e).red().bold());
                    }
                }
            }
            // Query student infomation
            4 => {
                let mut id_input = "".to_string();
                println!("{}", "Please input your query id".white());
                std::io::stdin().read_line(&mut id_input)?;
                // println!("input is {}",id_input);
                let tesult = student_map.get(&id_input.trim().to_string());
                match tesult {
                    Some(v) => {
                        println!("{}", format!("{:?}", v).green());
                    }
                    None => {
                        println!("{}", "Not found".red().bold())
                    }
                }
            }
            // Modify data
            5 => {
                let result = put_student_id(&mut student_map);
                match result {
                    Ok(v) => {
                        println!("{}", "Successfully".green().bold())
                    }
                    Err(e) => {
                        eprintln!("{}", format!("{}", e).red().bold());
                    }
                }
            }
            // Print length
            6=>{
                let length=student_map.len();
                println!("Student length is {}",format!("{}",length).green().bold())
            }
            // Exit this application
            0 => {
                println!("{}", "Application exit".green().bold());
                return Ok(());
            }
            // Other situations
            _ => {
                println!("{}", help_s)
            }
        }
    }
}
