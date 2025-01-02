use calamine::{open_workbook_auto, Writer, Xlsx};
use calamine::xlsx::XlsxWriter;
use std::error::Error;

#[derive(Debug)]
struct Student {
    name: String,
    matric_number: String,
    department: String,
    level: String,
}

impl Student {
    fn new(name: &str, matric_number: &str, department: &str, level: &str) -> Self {
        Student {
            name: name.to_string(),
            matric_number: matric_number.to_string(),
            department: department.to_string(),
            level: level.to_string(),
        }
    }

    fn display(&self) {
        println!(
            "Name: {}\nMatric Number: {}\nDepartment: {}\nLevel: {}\n",
            self.name, self.matric_number, self.department, self.level
        );
    }
}

fn save_to_excel(students: Vec<Student>, file_name: &str) -> Result<(), Box<dyn Error>> {
    let mut workbook = XlsxWriter::new();
    let sheet = workbook.add_worksheet(None)?;

    // Write headers
    sheet.write_row(&["Name", "Matric Number", "Department", "Level"])?;

    // Write student data
    for student in students {
        sheet.write_row(&[
            &student.name,
            &student.matric_number,
            &student.department,
            &student.level,
        ])?;
    }

    workbook.save(file_name)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let students = vec![
        Student::new("Alice Johnson", "2019001", "Computer Science", "200 Level"),
        Student::new("Bob Smith", "2019002", "Electrical Engineering", "300 Level"),
        Student::new("Charlie Brown", "2019003", "Mechanical Engineering", "100 Level"),
    ];

    for student in &students {
        student.display();
    }

    save_to_excel(students, "students.xlsx")?;
    println!("Data has been saved to students.xlsx");

    Ok(())
}
