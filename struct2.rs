struct Student{
    name:String,
    roll: u64,
    dept: String,
    }
      
    fn main() {
       let student1 = Student{
            name: String::from("manisha"),
            roll: 41,
            dept:String::from("CSE"),
        };
        println!("name {:?}",student1.name);
        println!(" roll no is {}",student1.roll);
        println!("Department {:?}",student1.dept)
    }