fn main() {

    let A : i32 = 10;
    let B : i32 = 20;
    println!("Value of A : {} ",A);
    println!("Value of B : {} ",B);

    let mut res = A>B;
    println!("A is greater than B {} ",res);

    let mut res = A<B;
    println!("A is lesser than B {} ",res);

    let mut res = A>=B;
    println!("A is greater than  or equal to B {} ",res);

    let mut res = A<=B;
    println!("A is lesser than or equal to B {} ",res);

    let mut res = A==B;
    println!("A is equal to  B {} ",res);

    let mut res = A!=B;
    println!("A is not equal to  B {} ",res);


}
