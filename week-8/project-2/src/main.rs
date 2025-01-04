struct Person {
    name: String,
    years_of_experience: u32,
}

fn main() {
    // Create a vector of Person instances
    let people = vec![
        Person {
            name: String::from("Alice"),
            years_of_experience: 5,
        },
        Person {
            name: String::from("Bob"),
            years_of_experience: 8,
        },
        Person {
            name: String::from("Charlie"),
            years_of_experience: 3,
        },
        Person {
            name: String::from("Diana"),
            years_of_experience: 10,
        },
    ];

    // Initialize a variable to keep track of the person with the most experience
    let mut person_with_most_experience = &people[0];

    // Iterate through the vector to find the person with the highest experience
    for person in people.iter() {
        if person.years_of_experience > person_with_most_experience.years_of_experience {
            person_with_most_experience = person;
        }
    }

    // Print the result
    println!("The person with the most programming experience is: {}", person_with_most_experience.name);
}
