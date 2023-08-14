enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Province),
}

#[derive(Debug)]
enum Province {
    BC,
    AB,
    SK,
    MB,
    ON,
    PQ,
    NB,
    NS,
    NFLD,
    PE,
    YT,
    NWT,
    NU,
}

fn main() {
    println!("Hello, world!");

    value_in_cents(Coin::Quarter(Province::ON));
    let mut coin = Coin::Penny;
    let mut count = 0;
    match coin {
        Coin::Quarter(province) => println!("Province is from {:?}!", province),
        _ => count += 1,
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // _ => reroll(), // option None = reroll()
        _ => (), // option None = do nothing
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(province) => {
            println!("Province quarter from {:?}", province);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
