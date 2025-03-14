use std::io;

fn main() {
    let paid = query_user();
    let change = calc_change(paid);
    let coinage = calc_coins(change);
    println!("Change owed: {}", change);
    println!("Number of coins: {}", coinage);
}

fn query_user() -> u8 {
    println!("How much was paid (in cents)? ");
      
    let mut response = String::new();

    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read line.");

    // convert string response to an int 
    let payment = response
        .trim()
        .parse()
        .expect("Input is not a number or 'what do you think this is, a bank?.'");

    return payment;
}

fn calc_change(mut payment: u8) -> u8 {
    let price = 50;

    // ensure that the price paid is 50 cents or more
    // ask the user to pay more.
    while payment <= 50 {
        println!("This item costs 50 cents.");
        payment = query_user();
    }
    let change = payment - price;
    return change;
}

// calculate the number of coins returned using greedy algorithm
fn calc_coins(mut change: u8) -> u8 {
    let mut coin_count = 0;
    while change > 24 {
        change = change - 25;
        coin_count += 1;
    }
    while change > 9 {
        change = change - 10;
        coin_count += 1;
    }
    while change > 4 {
        change = change - 5;
        coin_count += 1;
    }
    while change > 0 {
        change = change -1;
        coin_count += 1;
    }
    return coin_count;
}