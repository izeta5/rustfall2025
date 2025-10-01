// // Car
// // Struct responsible for data
// struct Car {
//     seats: u8,
//     model: String,
// }

// // Methods are added by IMPL statement

// impl Car{
//     fn new(s:u8,m:String) -> Car { // static method
//         Car {
//             seats: s,
//             model: m,
//         }
//     }
//     fn get_model(&self) -> &String {
//         return &self.model
//     }
//     fn set_model(&mut self, new_model: String) {
//         self.model = new_model
//     }
// }

struct Student {
    name: String,
    major: String,
}

impl Student {
    fn new(n:String, m:String) -> Student {
        Student {
            name: n,
            major: m,
        }
    }
    fn get_major(&self) -> &String {
        return &self.major
    }
    fn set_major(&mut self, new_major: String) {
        self.major = new_major;
    }
}

fn main() {
    
    let mut my_student = Student::new("Rolando".to_string(), "Petroleum Engineering".to_string());
    println!("The student is named {} and studied {}.",my_student.name,my_student.get_major());
    my_student.set_major("Computer Engineering".to_string());
    println!("The student is named {} and studies {}.",my_student.name,my_student.get_major());
    
}
  
    // These were class notes ignore!
    // let mut my_car = Car::new(4,"Tacoma".to_string());
    // println!("Number of seats {}",my_car.seats);
    // println!("Type of car {}",my_car.get_model());
    // println!("Type of car {}",my_car.get_model());
    // my_car.set_model("Corolla".to_string());
    // println!("Type of car {}",my_car.get_model());
