pub struct Employee{
    pub name: String,
    pub department: String,
}


impl Employee {
    pub fn new(name:String, department: String) -> Self {
        Self {
            name, 
            department,
        }
    }
    pub fn transfer(&mut self, department : String) {
       self.department = department;
    }
}