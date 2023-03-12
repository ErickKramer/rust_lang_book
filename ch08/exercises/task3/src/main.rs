use std::collections::HashMap;
use std::io;

fn add_employee(company: &mut HashMap<String, Vec<String>>) {
    println!("Enter command as 'Add <name> to <department>': ");
    let mut command = String::new();
    io::stdin()
        .read_line(&mut command)
        .expect("Failed to get the given command");
    let command = command.trim();
    let words: Vec<&str> = command.split_whitespace().collect();
    if !command.starts_with("Add") || words.len() != 4 {
        println!("Invalid command given");
        return;
    }
    let name = words[1].to_string();
    let department_name = words[3].to_string().to_lowercase();

    let names = company.entry(department_name).or_insert(Vec::new());
    names.push(name);
}

fn report_department(company: &HashMap<String, Vec<String>>) {
    println!("Enter the deparment to check: ");
    let mut department_name = String::new();
    io::stdin()
        .read_line(&mut department_name)
        .expect("Failed to get the given department_name");
    let department_name = department_name.trim().to_string();
    if let Some(names) = company.get(&department_name) {
        println!("People that belong to department: {}", department_name);
        let mut sorted_names = names.clone();
        sorted_names.sort();
        for name in sorted_names {
            println!("{}", name);
        }
    } else {
        println!("No employees found for department {}", department_name);
    }
}

fn report_company(company: &HashMap<String, Vec<String>>) {
    for (department, names) in company {
        println!("People that belong to department: {}", department);
        let mut sorted_names = names.clone();
        sorted_names.sort();
        for name in sorted_names {
            println!("{}", name);
        }
    }
}

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Choose one of the following options");
        println!("1: To add a new employee to a department");
        println!("2: To report all the employees of a given department");
        println!("3: To report all the employeess at the company");
        println!("4: Quit");

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option_number = option.trim();
        match option_number {
            "1" => add_employee(&mut company),
            "2" => report_department(&company),
            "3" => report_company(&company),
            "4" => {
                println!("Bye...");
                std::process::exit(0);
            }
            _ => println!("Invalid input"),
        }
    }
}
