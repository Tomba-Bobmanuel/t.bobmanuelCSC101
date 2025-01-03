use std::f64::consts::PI;
use std::io;

fn area_of_trapezium(height: f64, base1: f64, base2: f64) -> f64 {
    height / 2.0 * (base1 + base2)
}

fn area_of_rhombus(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}

fn area_of_parallelogram(base: f64, altitude: f64) -> f64 {
    base * altitude
}

fn area_of_cube(side: f64) -> f64 {
    6.0 * side.powi(2)
}

fn volume_of_cylinder(radius: f64, height: f64) -> f64 {
    PI * radius.powi(2) * height
}

fn main() {
    println!("Select an option to calculate the area/volume:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");

    // Read the user's choice
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice: u32 = choice.trim().parse().expect("Invalid input");

    match choice {
        1 => {
            // Area of Trapezium
            println!("Enter height:");
            let mut height = String::new();
            io::stdin().read_line(&mut height).expect("Failed to read line");
            let height: f64 = height.trim().parse().expect("Invalid input");

            println!("Enter base1:");
            let mut base1 = String::new();
            io::stdin().read_line(&mut base1).expect("Failed to read line");
            let base1: f64 = base1.trim().parse().expect("Invalid input");

            println!("Enter base2:");
            let mut base2 = String::new();
            io::stdin().read_line(&mut base2).expect("Failed to read line");
            let base2: f64 = base2.trim().parse().expect("Invalid input");

            let area = area_of_trapezium(height, base1, base2);
            println!("Area of the Trapezium: {}", area);
        }
        2 => {
            // Area of Rhombus
            println!("Enter diagonal1:");
            let mut diagonal1 = String::new();
            io::stdin().read_line(&mut diagonal1).expect("Failed to read line");
            let diagonal1: f64 = diagonal1.trim().parse().expect("Invalid input");

            println!("Enter diagonal2:");
            let mut diagonal2 = String::new();
            io::stdin().read_line(&mut diagonal2).expect("Failed to read line");
            let diagonal2: f64 = diagonal2.trim().parse().expect("Invalid input");

            let area = area_of_rhombus(diagonal1, diagonal2);
            println!("Area of the Rhombus: {}", area);
        }
        3 => {
            // Area of Parallelogram
            println!("Enter base:");
            let mut base = String::new();
            io::stdin().read_line(&mut base).expect("Failed to read line");
            let base: f64 = base.trim().parse().expect("Invalid input");

            println!("Enter altitude:");
            let mut altitude = String::new();
            io::stdin().read_line(&mut altitude).expect("Failed to read line");
            let altitude: f64 = altitude.trim().parse().expect("Invalid input");

            let area = area_of_parallelogram(base, altitude);
            println!("Area of the Parallelogram: {}", area);
        }
        4 => {
            // Area of Cube
            println!("Enter the side length:");
            let mut side = String::new();
            io::stdin().read_line(&mut side).expect("Failed to read line");
            let side: f64 = side.trim().parse().expect("Invalid input");

            let area = area_of_cube(side);
            println!("Area of the Cube: {}", area);
        }
        5 => {
            // Volume of Cylinder
            println!("Enter radius:");
            let mut radius = String::new();
            io::stdin().read_line(&mut radius).expect("Failed to read line");
            let radius: f64 = radius.trim().parse().expect("Invalid input");

            println!("Enter height:");
            let mut height = String::new();
            io::stdin().read_line(&mut height).expect("Failed to read line");
            let height: f64 = height.trim().parse().expect("Invalid input");

            let volume = volume_of_cylinder(radius, height);
            println!("Volume of the Cylinder: {}", volume);
        }
        _ => {
            println!("Invalid choice! Please select a valid option.");
        }
    }
}

