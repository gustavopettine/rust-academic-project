use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

use crate::student::{Student, Status};

const DB_FILE: &str = "students.txt";

pub fn create_student(student: &Student) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(DB_FILE)?;

    writeln!(
        &mut file,
        "{},{},{}",
        student.name, student.id, status_to_str(&student.status)
    )?;

    Ok(())
}

pub fn read_students() -> io::Result<Vec<Student>> {
    let file = File::open(DB_FILE)?;
    let reader = BufReader::new(file);

    let mut students = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let fields: Vec<&str> = line.split(',').collect();

        let name = fields[0].to_string();
        let id: u32 = fields[1].parse().expect("Invalid id");
        let status = str_to_status(fields[2]);

        let student = Student {
            name,
            id,
            status,
        };

        students.push(student);
    }

    Ok(students)
}

pub fn update_student(old_id: u32, new_student: &Student) -> io::Result<()> {
    let students = read_students()?;
    let mut file = File::create(DB_FILE)?;

    for student in students {
        if student.id == old_id {
            writeln!(
                &mut file,
                "{},{},{}",
                new_student.name,
                new_student.id,
                status_to_str(&new_student.status)
            )?;
        } else {
            writeln!(
                &mut file,
                "{},{},{}",
                student.name,
                student.id,
                status_to_str(&student.status)
            )?;
        }
    }

    Ok(())
}

pub fn delete_student(id: u32) -> io::Result<()> {
    let students = read_students()?;
    let mut file = File::create(DB_FILE)?;

    for student in students {
        if student.id != id {
            writeln!(
                &mut file,
                "{},{},{}",
                student.name,
                student.id,
                status_to_str(&student.status)
            )?;
        }
    }

    Ok(())
}

fn status_to_str(status: &Status) -> &str {
    match status {
        Status::Active => "Active",
        Status::Enrolled => "Enrolled",
        Status::Graduated => "Graduated",
    }
}

fn str_to_status(status_str: &str) -> Status {
    match status_str {
        "Active" => Status::Active,
        "Enrolled" => Status::Enrolled,
        "Graduated" => Status::Graduated,
        _ => panic!("Invalid status"),
    }
}
