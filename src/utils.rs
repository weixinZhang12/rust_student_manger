use core::hash;
use std::{
    collections::HashMap,
    error::{self, Error},
    fs,
};

use colored::Colorize;
use serde_json::ser;
// type StudentMap = HashMap<String, Student>;
use crate::student::Student;

///check if there is have id
fn is_have_id(student_map: &HashMap<String, Student>, id: &String) -> bool {
    let result = student_map.get(id);
    match result {
        Some(v) => true,
        None => false,
    }
}
/// Add student infomation
pub fn add_student(studnet_map: &mut HashMap<String, Student>) -> Result<(), Box<dyn Error>> {
    // User input string
    let mut name_input = "".to_string();
    let mut age_input = "".to_string();
    let mut student_id_input = "".to_string();
    let mut address_input = "".to_string();

    let mut name = "".to_string();
    let mut age = 0;
    let mut student_id = "".to_string();
    let mut address = "".to_string();

    // Get user input
    println!("{}", "Please input Student student id".blue());
    std::io::stdin().read_line(&mut student_id_input)?;
    if is_have_id(&studnet_map, &&student_id_input.trim().to_string()) {
        // panic!()
        // return Err("s");
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "Student ID already exists",
        )));
    }
    println!("{}", "Please input Student name".blue());
    std::io::stdin().read_line(&mut name_input)?;
    println!("{}", "Please input Student age".blue());
    std::io::stdin().read_line(&mut age_input)?;

    println!("{}", "Please input Student address".blue());
    std::io::stdin().read_line(&mut address_input)?;
    // If id exist,end add student infomation

    name = name_input.trim().to_string();
    age = age_input.trim().parse::<i32>()?;
    student_id = student_id_input.trim().to_string();
    address = address_input.trim().to_string();
    let s = Student::new(student_id.clone(), name, age, address);
    studnet_map.insert(student_id, s);
    write_hash_to_file(&studnet_map)?;
    //    studnet_array.push(s);
    Ok(())
}

pub fn del_student(student_map: &mut HashMap<String, Student>) -> Result<(), Box<dyn Error>> {
    let mut student_id = "".to_string();
    println!("{}", "Please input need delete student id".blue());
    std::io::stdin().read_line(&mut student_id);
    let result = student_map.remove(&student_id.trim().to_string());
    match result {
        Some(v) => {
            write_hash_to_file(&student_map);
            Ok(())
        },
        None => {
            println!("{}", "Failed delete the student infomation".red().bold());
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Student id is not found",
            )))
        }
    }
}

pub fn read_json_to_hash() -> Result<HashMap<String, Student>, Box<dyn Error>> {
    let s = fs::read_to_string("./student.json")?;

    let dejson: HashMap<String, Student> = serde_json::from_str(s.as_str())?;
    Ok(dejson)
}
pub fn write_hash_to_file(map: &HashMap<String, Student>) -> Result<(), Box<dyn Error>> {
    let json_info = serde_json::to_string(map)?;
    fs::write("./student.json", json_info)?;
    Ok(())
}
