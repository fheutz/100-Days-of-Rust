fn main() {
    let number = 3;
    smaller_than_five(number);
    other_than_zero(number);
    divisibility(number);
    let number = return_five_or_six(true);
    smaller_than_five(number);
    counter();
    while_loop();
    for_elements();
}

fn smaller_than_five(x: u32) -> bool {
    if x < 5 {
        println!("condition was true");
        return true
    } else {
        println!("condition was false");
        return false
    }
}

fn other_than_zero(x: u32) -> bool {
    if x != 0 {
        println!("number was something other than zero");
        return true
    }
    false
}

fn divisibility (x: u32) {
    if x % 4 == 0 {
        println!("number is divisible by 4");
    } else if x % 3 == 0 {
        println!("number is divisible by 3");
    } else if x % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn return_five_or_six (b: bool) -> u32 {
    let number = if b {5} else { 6};
    number
}

fn counter () {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn while_loop () {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_elements() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}