fn main() {
    let name ="Aisha Lawal";
    let uni:&str = "Pan-Atlantic University";
    let addr:&str = "Rd 52 lekki-Epe Expressway, Ibeju-Lekki, Lagos";
    println!("Name: {}",name );
    println!("University: {}",uni, addr );


    let department:&'static str = "Computer Science";
    let school:&'static str = "School of Science and Technology";
    println!("Department: {}, \nSchool: {}",department,school );
}

