struct Student{
    id:i32,
    name:String,
    age:i32,
    address:String,
}
impl Student{
    fn new(id:i32,name:String,age:i32)->Student{
        Student{
            id,name,age,address:String::from(""),
        }
    }
}