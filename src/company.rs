use crate::department::Department;

pub struct Company {
    pub name: String,
    pub departments : Vec<Department>,
}

impl Company {
    pub fn new(name: &str) -> Self {
        Self { 
            name: name.to_string(), 
            departments: Vec::new(),
        }
    }
}