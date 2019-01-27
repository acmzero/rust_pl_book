use std::io;
use std::cmp::Ordering;

fn main() {

    println!("Guessing Game");
    loop {
        println!("Enter number: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error reading line");
        

        let random_number = 32;

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too Low"),
            Ordering::Greater => println!("Too Big"),
            Ordering:: Equal => {
                println!("You win!");
                break;
            }
        };

    }

    
}
