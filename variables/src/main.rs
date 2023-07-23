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
    // let sum = 5 + 10;
    // let diff = 95.5 - 4.3;
    // let product = 4 * 30;
    // let quot = 56.7 / 32.2;
    // let remainder = 43 % 5;
    // let character = 't';
    // let text = "text";
    // println!("Sum: {}, Diff: {}, Product: {}, Quotient: {}, Remainder: {}\n", sum, diff, product, quot, remainder);
    // println!("Character: {}, String: {}\n", character, text);

    // // tuple example
    // let tup = (500, 6.4, 1);
    // let (x, y, z) = tup;
    // println!("The value of x is : {}\nThe value of y is : {}\nThe value of z is : {}", tup.0, tup.1, tup.2);

    // // arrays
    // let a = [1,2,3,4,5];
    // let first = a[0];
    // let second = a[1];
    // println!("First: {}, Second: {}", first, second);

    // functions
    // println!("Hello World");
    // another_function("Hello parameter".to_string());

    // expressions
    // let y = {
    //     let x = 3;
    //     x + 1
    // };
    // println!("The value of y is: {}", y);

    // functions w/ return values
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

// fn another_function(parm: String){
//     println!("{}",parm);
// }

fn plus_one(x: i32) -> i32 {
    x + 1
}
