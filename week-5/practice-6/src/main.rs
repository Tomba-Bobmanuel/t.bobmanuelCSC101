fn main() {
    let n1 = "Electrical ".to_string();
    let n2 = "Electronic ".to_string();
    let n3 = "Engineering".to_string();
    
    // Concatenate the strings properly
    let n4 = n1 + &n2 + &n3;  // This will work, but `n1` is moved here, so we use references for `n2` and `n3`

    println!("\nThe {} is informed by the aspiration to train electrical/electronic 
        engineering professionals in the area of design, building,
         and maintenance of electrical control systems.", n4);

    let w1 = "computer".to_string();
    let w2 = "science".to_string();
    
    // Concatenate w1 and w2
    let w3 = w1 + &w2;

    println!("\n{} is aimed at developing competent, creative, 
        innovative, entrepreneurial, ethically-minded persons capable
         of creating value in the diverse field of computer science.", w3);
}
