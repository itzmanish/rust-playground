// Using a hash map and vectors, create a text interface
// to allow a user to add employee names to a department in a company.
// For example, “Add Sally to Engineering” or “Add Amir to Sales.”
// "Show employee in Engineering", "Show people in company by department"
// Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.
use std::collections::hash_map::{Entry, HashMap};
use std::io::Write;

fn insert(dict: &mut HashMap<String, Vec<String>>, emp: String, department: String) {
    match dict.entry(department) {
        Entry::Occupied(mut e) => {
            e.get_mut().push(emp);
        }
        Entry::Vacant(e) => {
            e.insert(vec![emp]);
        }
    }
}

fn output(dict: &HashMap<String, Vec<String>>, department: &String) {
    let value = dict.get(department).unwrap();
    println!("==========================");
    println!("Employees in {} department", department);
    for each in value {
        println!("{}", each);
    }
}

fn output_all(dict: &mut HashMap<String, Vec<String>>) {
    println!("==========================");
    for (key, value) in dict {
        print!("Peoples in {} department: ", key);
        std::io::stdout().flush().unwrap();
        value.sort();
        for people in value {
            print!("\t {}", people);
            std::io::stdout().flush().unwrap();
        }
        println!();
    }
}

fn main() {
    // Need to create a hashmap<string,hashmap<string,Vector<hashmap<string,Vector<string>>>>>
    // eg - {"sales":["Amir"],"engineering": ["Sally"]}
    let mut company_data: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut input_string = String::new();
        print!("Enter statement: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin()
            .read_line(&mut input_string)
            .expect("Error during taking input");
        let input_arr: Vec<&str> = input_string.trim().split(" ").collect();
        match input_arr[0] {
            "Add" => {
                let employee = input_arr[1].to_string();
                let department = input_arr[3].to_string();
                insert(&mut company_data, employee, department);
            }
            "Show" => {
                match input_arr[1] {
                    "employee" => {
                        // show employee in a department
                        let department = input_arr[3].to_string();
                        output(&company_data, &department);
                    }
                    "people" => {
                        output_all(&mut company_data);
                    }
                    _ => {}
                }
            }
            "Quit" => {
                println!("==========================");
                println!("Bye!");
                break;
            }
            _ => {
                println!("Please select correct option");
            }
        }
    }
}
