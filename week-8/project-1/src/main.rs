use std::io;

enum PublicServantCategory {
    OfficeAdministrator,
    Academic,
    Lawyer,
    Teacher,
}

struct Staff {
    position: PublicServantCategory,
    years_of_experience: u32,
}

fn get_aps_level(staff: &Staff) -> String {
    match staff.position {
        PublicServantCategory::OfficeAdministrator => {
            if staff.years_of_experience <= 2 {
                String::from("APS 1-2: Intern")
            } else if staff.years_of_experience <= 5 {
                String::from("APS 3-5: Administrator")
            } else if staff.years_of_experience <= 8 {
                String::from("APS 5-8: Senior Administrator")
            } else if staff.years_of_experience <= 10 {
                String::from("EL1 8-10: Office Manager")
            } else {
                String::from("EL2 10-13: Director")
            }
        }
        PublicServantCategory::Academic => {
            if staff.years_of_experience <= 2 {
                String::from("APS 1-2: –")
            } else if staff.years_of_experience <= 5 {
                String::from("APS 3-5: Research Assistant")
            } else if staff.years_of_experience <= 8 {
                String::from("APS 5-8: PhD Candidate")
            } else if staff.years_of_experience <= 10 {
                String::from("EL1 8-10: Post-Doc Researcher")
            } else {
                String::from("SES: CEO")
            }
        }
        PublicServantCategory::Lawyer => {
            if staff.years_of_experience <= 2 {
                String::from("APS 1-2: Paralegal")
            } else if staff.years_of_experience <= 5 {
                String::from("APS 3-5: Junior Associate")
            } else if staff.years_of_experience <= 8 {
                String::from("APS 5-8: Associate")
            } else if staff.years_of_experience <= 10 {
                String::from("EL1 8-10: Senior Associate 1-2")
            } else {
                String::from("SES: Partner")
            }
        }
        PublicServantCategory::Teacher => {
            if staff.years_of_experience <= 2 {
                String::from("APS 1-2: Placement")
            } else if staff.years_of_experience <= 5 {
                String::from("APS 3-5: Classroom Teacher")
            } else if staff.years_of_experience <= 8 {
                String::from("APS 5-8: Snr Teacher")
            } else if staff.years_of_experience <= 10 {
                String::from("EL1 8-10: Leading Teacher")
            } else {
                String::from("EL2 10-13: Deputy Principal")
            }
        }
    }
}

fn main() {
    // Display options for selecting staff position
    println!("Select the staff position by entering the number corresponding to the role:");
    println!("1. Office Administrator");
    println!("2. Academic");
    println!("3. Lawyer");
    println!("4. Teacher");

    // Read the user's choice
    let mut position_input = String::new();
    io::stdin().read_line(&mut position_input).expect("Failed to read line");
    let position_input = position_input.trim();

    let position = match position_input {
        "1" => PublicServantCategory::OfficeAdministrator,
        "2" => PublicServantCategory::Academic,
        "3" => PublicServantCategory::Lawyer,
        "4" => PublicServantCategory::Teacher,
        _ => {
            println!("Invalid selection.");
            return;
        }
    };

    // Ask the user for years of experience
    println!("Enter years of experience:");
    let mut years_input = String::new();
    io::stdin().read_line(&mut years_input).expect("Failed to read line");
    let years_of_experience: u32 = match years_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input for years of experience.");
            return;
        }
    };

    // Create a Staff object and determine the APS level
    let staff = Staff {
        position,
        years_of_experience,
    };

    let aps_level = get_aps_level(&staff);
    println!("The staff holds the position: {}", aps_level);
}
