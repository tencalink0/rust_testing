fn fibonacci(limit: u32) -> usize{
    let mut counter = 1; 
    let mut prev_counter = 1;

    for _ in 0..limit {
        let temp_counter = counter;
        counter += prev_counter;
        prev_counter = temp_counter;
    }

    counter
}

fn main() {
    struct Verb {
        
    }
    println!("How many iterations of the fibonacci sequence should be performed?");

    let mut user_input = String::new();

    std::io::stdin()
        .read_line(&mut user_input)
        .expect("System Error");

    let user_input: u32 = match user_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Failed to parse");
            std::process::exit(1);
        }
    };

    let fib = fibonacci(user_input);
    println!("{}", fib);
}
