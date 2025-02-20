use std::collections::HashMap;
use std::io;

fn main() {
    let mut employees: HashMap<String, String> = HashMap::new();

    loop {
        println!("Select menu:");
        println!("1. Add employee.");
        println!("2. List of all people in a department.");
        println!("3. List of all people in the company.");

        let mut selected_menu = String::new();
        io::stdin()
            .read_line(&mut selected_menu)
            .expect("Faild to read line");

        match selected_menu.trim().parse::<u32>().unwrap() {
            1 => {
                println!("Please input information. (e.g. Add Sally to Engineering)");
                let mut user_input = String::new();
                io::stdin()
                    .read_line(&mut user_input)
                    .expect("Faild to read line");
                let (name, department) = get_name_and_department(&user_input[..]);
                employees.insert(name, department);
            }
            2 => {
                println!("What department?");
                let mut user_input = String::new();
                io::stdin()
                    .read_line(&mut user_input)
                    .expect("Faild to read line");
                for (name, department) in &employees {
                    if department == user_input.trim() {
                        println!("{name}");
                    }
                }
            }
            3 => {
                let mut all_employees = Vec::new();
                for name in employees.keys() {
                    all_employees.push(name);
                }
                all_employees.sort();
                for name in &all_employees {
                    println!("{name}");
                }
            }
            _ => println!("Wrong input. Please re-type."),
        };

        println!("{:?}", employees);
    }
}

fn get_name_and_department(input: &str) -> (String, String) {
    let mut iter = input.split_whitespace();
    let mut name = String::new();
    let mut department = String::new();
    while let Some(word) = iter.next() {
        match word {
            "Add" => {
                name = iter.next().unwrap().to_string();
            }
            "to" => {
                department = iter.next().unwrap().to_string();
            }
            _ => break,
        }
    }
    (name, department)
}
