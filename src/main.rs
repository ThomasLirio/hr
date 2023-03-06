use std::{io, collections::HashMap};

use hr::company::Company;

fn main() {
    println!("What is your company name?");

    let mut company_name = String::new();

    io::stdin()
        .read_line(&mut company_name)
        .expect("failed to read line.");

    let mut _company = Company::new(&company_name);

    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("{} Organogram", company_name.trim());
        println!("Please choose an option:");
        println!("A - to add employees to a departments");
        println!("R - to remove employees from a department");
        println!("T - to transfer all employees from a department to another");
        println!("D - to delete a department and all its employees");
        println!("L - to list all  employees in a department");
        println!("Q - to quit Organogram or an inside option");


        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("failed to read line.");

        match choice.to_uppercase().trim() {
            "A" => add_employee(&mut departments),
            "R" => remove_employee(&mut departments),
            "T" => transfer_employees(&mut departments),
            "D" => delete_department(&mut departments),
            "L" => list_employees(&departments),
            "Q" => break,
            _ => continue,
        }

    }
   
}

fn add_employee(departments: &mut HashMap<String, Vec<String>>) {
    loop {
        println!("To add an employee to a department please write: 
            employee name dashdash  department name. 
            Ex: John Smith -- Research and Development");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        if input.trim().to_lowercase() == "q"{
            break;
        } 

        let mut counter = 0; 
        let mut depart = String::new();
        let mut employee = String::new();
        for w in input.split("--") {
            if w.is_empty() || w.trim().len() < 2 {
                println!("Invalid input");
                continue;
            }
            if counter == 0 {
                employee = capitalize(w.trim());
            } else if counter == 1 {
                depart = w.trim().to_lowercase();
            }
            counter+=1; 
        }

        if counter != 2 {
            println!("Invalid input");
            continue;
        } else {
             departments.entry(depart).or_insert(Vec::new()).push(employee);
        }
    }
    if departments.is_empty() {
        println!("No departments found.");
    } else {
        println!("{:?}", departments);
    }
}

fn remove_employee(departments: &mut HashMap<String, Vec<String>>) {
    loop {
        println!("To remove an employee from a department please write: 
            employee name dashdash  department name. 
            Ex: John Smith -- Research and Development");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        if input.trim().to_lowercase() == "q"{
            break;
        } 

        let mut counter = 0; 
        let mut depart = String::new();
        let mut employee = String::new();
        for w in input.split("--") {
            if w.is_empty() {
                println!("Invalid input");
                continue;
            }
            if counter == 0 {
                employee = capitalize(w.trim());
            } else if counter == 1 {
                depart = w.trim().to_lowercase();
            }
            counter+=1; 
        }

        if counter != 2 {
            println!("Invalid input");
            continue;
        } else if departments.contains_key(&depart){
            departments.get_mut(&depart).map(|val| val.retain(|e| *e != employee));
        }
    }
    if departments.is_empty() {
        println!("No departments found.");
    } else {
        println!("{:?}", departments);
    }
}

fn transfer_employees(departments: &mut HashMap<String, Vec<String>>) {

    loop {
        println!("To transfer the employees from a department to another  type the departments' name separated by two dashes, 
        Ex: Research --  Development.");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        if input.trim().to_lowercase() == "q"{
            break;
        } 

        let mut counter = 0; 
        let mut dep_source = String::new();
        let mut dep_target = String::new();
        for w in input.split("--") {
            if w.is_empty() {
                println!("Invalid input");
                continue;
            }
            if counter == 0 {
                dep_source = w.trim().to_lowercase();
            } else if counter == 1 {
                dep_target = w.trim().to_lowercase();
            }
            counter+=1; 
        }
        
        if counter != 2 {
            println!("Invalid input");
            continue;
        } else if !departments.contains_key(&dep_source) {
            println!("{dep_source} not found");
            continue;
        } else if !departments.contains_key(&dep_target) {
            println!("{dep_target} not found");
            continue;
        } else {
			let mut  p2 = departments[&dep_source].clone();
            departments.get_mut(&dep_target)
                       .expect("no {dep_target} found")
                       .append(&mut p2);

            departments.insert(dep_source,Vec::new());
            
        }

        if departments.is_empty() {
            println!("No departments found.");
        } else {
            println!("{:?}", departments);
        }
 
    }

}

fn delete_department(departments: &mut HashMap<String, Vec<String>>) {
     loop {
        println!("To delete a department type the department's name, Ex: Research and Development.");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        if input.trim().to_lowercase() == "q"{
            break;
        } else if departments.is_empty() {
            println!("No departments found.");
        } else if departments.contains_key(&input.trim().to_lowercase()) {
            departments.remove(&input.trim().to_lowercase());
        }   
    }
    
    if departments.is_empty() {
        println!("No departments found.");
    } else {
        println!("{:?}", departments);
    }

}


fn list_employees(departments: &HashMap<String, Vec<String>>) {
    loop {
        println!("To list the employees of a department type the department's name, Ex: Research and Development.
            If you want to list the employees by department type: Company");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        if input.trim().to_lowercase() == "q"{
            break;
        } else if input.trim().to_lowercase() == "company"{
            if departments.is_empty() {
                println!("No departments found.");
            } else {
                println!("{:?}", departments);
            }
        } else if input.trim().to_lowercase().len() > 0 {
            if departments.contains_key(&input.trim().to_lowercase()) {
                println!("{:?}", departments[&input.trim().to_lowercase()]);
            } 
            // else if departments.contains_key(&capitalize(input.trim())) {
            //     println!("{:?}", departments[&capitalize(input.trim())]);
            // } 
            else {
                println!("No departments found.");
            }
        }
    }
}

fn capitalize(s: &str) -> String {
    let mut caps = String::new();
    for w in s.split(" "){
        let mut c = w.trim().chars();
        let mut a = String::new();
        match c.next() {
            None => {},
            Some(f) => {a = f.to_uppercase().collect::<String>() + &c.as_str().to_lowercase()},
        }
        caps.push_str(&a);
        caps.push(' ');
    }
    caps
}