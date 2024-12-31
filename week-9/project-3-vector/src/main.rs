fn main() {
    // 1. Define the datasets
    let commissioners = vec![
        "Aigbogun Alamba Duadu",
        "Murtala Afeef Bendu",
        "Okoricha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];

    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    let zones = vec![
        "South West",
        "North East",
        "South South",
        "South West",
        "South West",
    ];

    // 2. Combine the datasets into one with formatted output
    // Set fixed column widths to align the columns properly
    println!("{:<4} {:<30} {:<20} {:<20}", "S/N", "Name of Commissioner", "Ministry", "Geographical Zone");
    for i in 0..commissioners.len() {
        // Each dataset is of the same length, so we use the index `i` to access each element
        println!(
            "{:<4} {:<30} {:<20} {:<20}",
            i + 1, // S/N starts from 1
            commissioners[i],
            ministries[i],
            zones[i]
        );
    }
}
