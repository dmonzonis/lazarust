use lazarust::random;

fn main() {
    // range
    println!(
        "A random integer between 1 and 10: {}",
        random::range(1, 11)
    );
    println!(
        "A random float between -1 and 1: {}",
        random::range(-1., 1.)
    );

    // roll
    println!("Rolling a 3d6: {}", random::roll(6, 3));

    // one_in
    let mut tries = [false; 10];
    for t in &mut tries {
        *t = random::one_in(10);
    }
    println!(
        "Trying to succeed 10 times with a 1/10 probability of success: {:?}",
        tries
    );

    // normal
    println!(
        "Sample of a normal (gaussian) distribution with mean 0 and stdev 1: {}",
        random::normal(0., 1.)
    );

    // choice
    let samples = vec!["first", "second", "third"];
    println!(
        "Selecting a random item from a slice: {}",
        random::choice(&samples).unwrap()
    );
}
