use crypto_pals::sets;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("  cargo run -- <set> <challenge>");
        println!("  cargo run -- list");
        return;
    }

    let challenges = sets::get_challenges();

    match args[1].as_str() {
        "list" => {
            println!("Implemented challenges:");
            for (key, _) in challenges.iter() {
                println!("Set {} Challenge {}", key.0, key.1);
            }
        }
        set => {
            if args.len() != 3 {
                println!("Please specify a challenge number");
                return;
            }
            let challenge = &args[2];
            match challenges.get(&(set, challenge.as_str())) {
                Some(f) => println!("Result:\n{}", f()),
                None => println!("Challenge not implemented"),
            }
        }
    }
}
