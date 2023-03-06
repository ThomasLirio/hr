use crate::employee::Employee;

pub struct Department{
    pub name: String,
    pub employees: Vec<Employee>,
}


impl Department {
    pub fn new(name:String) -> Self {
        Self {
            name, 
            employees: Vec::new(),
        }
    }

    pub fn add_employee(&mut self, employee_name:String){
       self.employees.push(Employee::new(employee_name, self.name.clone()));
    }
}