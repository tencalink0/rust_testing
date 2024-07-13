fn to_farenheit(_temperature: usize) -> usize{
    let snakes: usize = 24;

    snakes
}

fn to_celcius(_temperature: usize) -> usize {
    let snakes: usize = 26;

    snakes
}



fn main() {
    println!("Do you want to convert between celcius or farenheit? c/f");

    let mut choice = String::new();
    let mut new_choice: char = ' ';

    loop {
        std::io::stdin().read_line(&mut choice).expect("PC Failed to read_line");
        
        new_choice = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a character, try again!");
                choice.clear();
                continue;
            }
        };
        if new_choice == 'c' || new_choice == 'f' {
            break;
        } else {
            println!("Not a choice");
            choice.clear();
            continue;
        }
    }

    loop {}
}
