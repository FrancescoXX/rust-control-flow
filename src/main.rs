fn main(){
//Basic if else
let number = 3;

if number < 5 {
    println!("condition was true");
} else {
    println!("condition was false");
}

//let with if else
let condition = true;
let number = if condition {
    5
} else {
    6
};

println!("The value of number is: {}", number);
println!("-------------------------------------------");

//Nested if expressions
let num = 15;

if num % 2 == 0 {
    println!("{} is even", num);
} else {
    println!("{} is odd", num);

    if num > 10 {
        println!("{} is also greater than 10", num);
    } else {
        println!("{} is not greater than 10", num);
    }
}
println!("-------------------------------------------");


//&& and || operators
let a = 10;
let b = 5;
let c = 20;

// Using && (AND) to check if 'a' is greater than 'b' AND 'b' is greater than 'c'
if a > b && b > c {
    println!("a is greater than b and b is greater than c");
} else {
    println!("Condition with && not met");
}

// Using || (OR) to check if 'a' is less than 'b' OR 'b' is less than 'c'
if a > b || b > c {
    println!("At least one condition with || is met");
} else {
    println!("Neither condition with || is met");
}
println!("-------------------------------------------");

//Match statement
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

let coin = Coin::Quarter;
println!("The value of the coin is: {}", value_in_cents(coin));
println!("-------------------------------------------");

//Infinite loop (commented out)
// loop {
//     println!("again!");
// }

// Returning a value from a loop
let mut counter = 0;

let result = loop {
    counter += 1;

    if counter == 10 {
        break counter * 2;
    }
};

println!("The result is {}", result);
println!("-------------------------------------------");


//While loop
let mut counter = 3;

while counter != 0 {
    println!("{}", counter);

    counter -= 1;
    //wait for 1 second
    std::thread::sleep(std::time::Duration::from_secs(1));
}

println!("LIFTOFF!!!");
println!("-------------------------------------------");

//For loop to iterate over a collection
let a = [10, 20, 30, 40, 50];

for element in a.iter() {
    println!("the value is: {}", element);
}

println!("-------------------------------------------");

//For loop for elements in a string
let s = "hello world";

for c in s.chars() {
    println!("{}", c);
}
println!("-------------------------------------------");

//For loop for elements in a range
for number in (1..4).rev() {
    println!("{}!", number);
}
println!("LIFTOFF!!!");
println!("-------------------------------------------");

//FizzBuzz game
for number in 1..=100 {
    if number % 3 == 0 && number % 5 == 0 {
        println!("FizzBuzz");
    } else if number % 3 == 0 {
        println!("Fizz");
    } else if number % 5 == 0 {
        println!("Buzz");
    } else {
        println!("{}", number);
    }
}
println!("-------------------------------------------");
}