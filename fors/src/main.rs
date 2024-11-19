use std::{collections::HashMap, fs};

use student::Student;
mod student;

fn main() {
    let mut student_map = HashMap::<String, Student>::new();
    let mut st = "".to_string();
    for i in 0..=999999{
        let s = Student::new(i.to_string(), i.to_string(), i, i.to_string());
        student_map.insert(i.to_string(), s);
    }
    st = serde_json::to_string(&student_map).unwrap();
    fs::write("./student.json", st);
}
