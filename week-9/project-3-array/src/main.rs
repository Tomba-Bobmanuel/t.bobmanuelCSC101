fn main() {
    // 1. Define the datasets as arrays
    let commissioners: [&str; 5] = [
        "Aigbogun Alamba Duadu",
        "Murtala Afeef Bendu",
        "Okoricha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let ministries: [&str; 5] = [
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones: [&str; 5] = [
        "South West",
        "North East",
        "South South",
        "South West",
        "South West",
    ];

    // 2. Combine the datasets into one with formatted output
    println!("{:<4} {:<30} {:<20} {:<20}", "S/N", "Name of Commissioner", "Ministry", "Geographical Zone");
    for i in 0..commissioners.len() {
        // Access each array element by index
        println!(
            "{:<4} {:<30} {:<20} {:<20}",
            i + 1, // S/N starts from 1
            commissioners[i],
            ministries[i],
            zones[i]
        );
    }
}
