fn main() {

    const SECONDS_IN_MINUTE: u32 = 60; // CONST

    println!("Seconds in minute:{}", SECONDS_IN_MINUTE);

    let x = 4; // immutable
    let mut y = 5; //mutable
    println!("x is: {}", x);
    println!("y is: {}", y);


    {
        let x = x + 5;  //  Shadowing

         println!("x is: {}", x);
    }

    let x = x + 1; // Redefining immutable variable
    y=10;

    println!("x is: {}", x);
    println!("y is: {}", y);
}
