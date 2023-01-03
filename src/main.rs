use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Hello! This is a guessing game!");

    let secret_number  = rand::thread_rng().gen_range(1..=100);


   loop {
       println!("Type any key and i will guess it");
       let mut guess = String::new();
       io::stdin()
           .read_line(&mut guess)
           .expect("Failed to read line");

       let guess: u32 = match guess.trim().parse(){
           Ok(num) => num,
           Err(_) => continue
       };

       println!("You typed: {guess}");

       match guess.cmp(&secret_number) {
           Ordering::Less => println!("Too small!"),
           Ordering::Greater => println!("Too big!"),
           Ordering::Equal => {
               println!("You win!");
               break;
           },
       }

   }

}
