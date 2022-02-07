use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let rand_num = rand::thread_rng().gen_range(1..=100);

    let mut i = 0;

    loop {
        i += 1;

        println!("Input your guess below:");

        let mut guess = String::new();

        // Actually one line, don't get confused
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Note, below we 
    
        // let guess: u32 = guess.trim().parse().expect("Invalid number!");

        // Instead of crashing like above, a clean re-loop
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {}", guess);
    
        match guess.cmp(&rand_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Winner!!");
                break;
            }
        }
    }

    println!("The secret number is: {}!", rand_num);
    println!("It took {} attemps!", i);
}
