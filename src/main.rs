mod student;
mod crud;

use std::io;

use student::{Student, Status};

fn main() {
    loop {
        println!("Choose an option:");
        println!("1. Create student");
        println!("2. Read students");
        println!("3. Update student");
        println!("4. Delete student");
        println!("5. Exit");

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read input");

        match option.trim().parse() {
            Ok(1) => create_student(),
            Ok(2) => read_students(),
            Ok(3) => update_student(),
            Ok(4) => delete_student(),
            Ok(5) => break,
            _ => println!("Invalid option"),
        }
    }
}

fn create_student() {
    println!("Enter the name of the student:");
    let name = read_input();

    println!("Enter the id of the student:");
    let id: u32 = read_input().parse().expect("Invalid id");

    println!("Choose the status of the student:");
    println!("1. Active");
    println!("2. Enrolled");
    println!("3. Graduated");

    let status = match read_input().trim().parse() {
        Ok(1) => Status::Active,
        Ok(2) => Status::Enrolled,
        Ok(3) => Status::Graduated,
        _ => {
            println!("Invalid option, status set to Active");
            Status::Active
        }
    };

    let student = Student {
        name,
        id,
        status,
    };

    if let Err(e) = crud::create_student(&student) {
        println!("Error creating student: {}", e);
    } else {
        println!("Student created successfully!");
    }
}

fn read_students() {
    match crud::read_students() {
        Ok(students) => {
            if students.is_empty() {
                println!("No students found");
            } else {
                println!("List of students:");
                for student in students {
                    println!(
                        "{} - Id: {} - Status: {:?}",
                        student.name, student.id, student.status
                    );
                }
            }
        }
        Err(e) => println!("Error reading students: {}", e),
    }
}

fn update_student() {
    println!("Enter the id of the student you want to update:");
    let old_id: u32 = read_input().parse().expect("Invalid id");

    println!("Enter the new name of the student:");
    let new_name = read_input();

    println!("Enter the new id of the student:");
    let new_id: u32 = read_input().parse().expect("Invalid id");

    println!("Choose the new status of the student:");
    println!("1. Active");
    println!("2. Enrolled");
    println!("3. Graduated");

    let new_status = match read_input().trim().parse() {
        Ok(1) => Status::Active,
        Ok(2) => Status::Enrolled,
        Ok(3) => Status::Graduated,
        _ => {
            println!("Invalid option, status set to Active");
            Status::Active
        }
    };

    let new_student = Student {
        name: new_name,
        id: new_id,
        status: new_status,
    };

    if let Err(e) = crud::update_student(old_id, &new_student) {
        println!("Error updating student: {}", e);
    } else {
        println!("Student updated successfully!");
    }
}

fn delete_student() {
    println!("Enter the id of the student you want to delete:");
    let id: u32 = read_input().parse().expect("Invalid id");

    if let Err(e) = crud::delete_student(id) {
        println!("Error deleting student: {}", e);
    } else {
        println!("Student deleted successfully!");
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}
