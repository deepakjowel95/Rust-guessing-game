use std::io;

use std::process::exit;


use rand::Rng;

fn looping(){
	loop {main();}
}


fn main() {
	let mut status = false;
	// while status == false {
		println!("Guess a number b/w 1 & 10!");
		let secret_number = rand::thread_rng().gen_range(1..=10);

		println!("Please in put your guess");
		let mut guess = String::new();
		io::stdin().read_line(&mut guess).expect("Failed to read line");

		let guess : i32 = guess.trim().parse().expect("Enter valid integer");
		if secret_number != guess {
			println!("you guessed:{guess}");
			println!("The secret number is:{secret_number}");
			println!("try again!");
			// retry();

		} else {
			println!("good guess!");
			// status = true;
			exit(0);
		} 
	looping();
}
// }



