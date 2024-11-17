#[derive(Debug,serde::Serialize,serde::Deserialize)]
pub struct Student{
    id:String,
    name:String,
    age:i32,
    address:String,
}
impl Student{
   pub fn new(id:String,name:String,age:i32,address:String)->Student{
        Student{
            id,name,age,address,
        }
    }
}