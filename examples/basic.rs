use extended_fizzbuzz::{fizzbuzz, Matcher};

fn main() {
    let matchers = vec![
        Matcher::new(3, "Fizz").expect("Failed to create `3=Fizz` matcher"),
        Matcher::new(5, "Buzz").expect("Failed to create `5=Buzz` matcher"),
    ];

    fizzbuzz(1, 100, &matchers).expect("FizzBuzzing failed");
}
