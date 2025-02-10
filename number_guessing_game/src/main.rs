use rand::Rng;

/// Guess a secret number between 1 and 10.
///
/// The program will print either "Too low! Try again." or "Too high! Try again."
/// depending on the entered number. If the entered number is correct, the program
/// will print "Correct! The number was <secret_number>" and terminate.
fn main() {
    let secret_number = rand::rng().random_range(1..=10);
    println!("Guess the number between 1 and 10");
    loop {
        let mut guess = String::new();
        println!("Enter your guess:");
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };
        if guess < secret_number {
            println!("Too low! Try again.");
        } else if guess > secret_number {
            println!("Too high! Try again.");
        } else {
            println!("🎉 Correct! The number was {}", secret_number);
            break;
        }
    }
}
