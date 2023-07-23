fn main() {
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // shadowing
    // let x = 5;

    // let x = x + 1;

    // let x = x * 2;

    // println!("The value of x is {}", x); 

    // let spaces = "   ";
    // let spaces = spaces.len();

    // primative types
    let sum = 5 + 10;
    let diff = 95.5 - 4.3;
    let product = 4 * 30;
    let quot = 56.7 / 32.2;
    let remainder = 43 % 5;
    let character = 't';
    let text = "text";
    println!("Sum: {}, Diff: {}, Product: {}, Quotient: {}, Remainder: {}\n", sum, diff, product, quot, remainder);
    println!("Character: {}, String: {}\n", character, text);

    // tuple example
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is : {}\nThe value of y is : {}\nThe value of z is : {}", tup.0, tup.1, tup.2);
}
