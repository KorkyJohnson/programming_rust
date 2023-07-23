fn main() {
    // let number = 7;

    // if number < 5 {
    //     println!("condition was true");
    // } else {
    //     println!("condition was false");
    // }

    // let num = 6;

    // if num % 4 == 0 {
    //     println!("Number is divisible by 4")
    // } else if num % 3 == 0 {
    //     println!("Number is divisible by 3")
    // } else if num % 2 == 0 {
    //     println!("Number is divisible by 2")
    // } else { 
    //     println!("Number is not divisible by 4, 3, or 2")
    // }

    // loop conditions
    // let mut cnt = 0;

    // let result = loop {
    //     cnt += 1;

    //     if cnt == 10 {
    //         break cnt * 2;
    //     }
    // };
    // println!("The result is {}", result)

    // for loop via iteration
    // let a = [10, 20, 30, 40, 50];
    // for element in a.iter() {
    //     println!("The value is : {}", element)
    // }

    // for loop via range (forward)
    for num in 1..4{
        println!("num: {}",num)
    }

    // for loop via range (reverse) 
    for num in (1..4).rev() {
        println!("num: {}",num)
    }
}
