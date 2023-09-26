#![allow(dead_code)]
mod input;
use std::{collections::HashMap, io, vec};

use input::get_input_string;

fn main() {
    let mut company = HashMap::from([
        (
            "dept 1".to_string(),
            vec!["hari".to_string(), "Rams".to_string()],
        ),
        (
            "dept 2".to_string(),
            vec!["Site".to_string(), "gita".to_string()],
        ),
    ]);
    loop {
        let mut input = String::new();
        println!("\n1. To add department\n2. To add Employee\n3. To list all departments\n4. List employees of a department\n5.Exit");
        io::stdin().read_line(&mut input).unwrap();
        let input: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid option 1 to 5");
                continue;
            }
        };
        if input > 5 {
            break;
        }
        match input {
            1 => {
                let dept = get_input_string("Enter the department name: ");
                add_department(dept, &mut company);
                continue;
            }
            2 => {
                add_employee(&mut company);
                continue;
            }
            3 => {
                display_dept(&company);
                continue;
            }
            4 => {
                list_employees(&company);
                continue;
            }
            5 => break,
            _ => {
                println!("Please enter from 1 to 5");
                continue;
            }
        }
    }
}

fn display_dept(company: &HashMap<String, Vec<String>>) {
    for em in company.keys().into_iter() {
        println!("{}", em);
    }
}

fn add_department(dept: String, company: &mut HashMap<String, Vec<String>>) {
    let exists = company.contains_key(&dept);
    if exists {
        println!("Company already exits");
    } else {
        company.entry(dept).or_insert(vec![String::new()]);
    }
}
fn add_employee(company: &mut HashMap<String, Vec<String>>) {
    let dept_name = get_input_string("Enter department name: ");
    let emps = company.entry(dept_name.clone()).or_insert(Vec::new());
    let emp = get_input_string("Enter employee name");
    emps.push(emp);
}

fn list_employees(emp: &HashMap<String, Vec<String>>) {
    let dept_name = get_input_string("Enter department name: ");
    let emps = match emp.get(&dept_name) {
        Some(emp) => emp,
        None => return,
    };
    for em in emps.iter() {
        println!("{}", em);
    }
}
