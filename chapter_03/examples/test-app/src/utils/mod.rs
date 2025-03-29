use rand::Rng;

pub fn tax_the_poor_in_math() {
    let mut rng = rand::thread_rng();

    // === Table 1: Lucky Draw ===
    println!("\n🎲 Welcome to Table 1: Lucky Draw!");
    let random_number = rng.gen_range(0..100);
    if random_number < 30 {
        println!("You drew {}! You win a small prize! 🎁", random_number);
    } else if random_number <= 70 {
        println!("You drew {}! You win a medium prize! 🎉", random_number);
    } else {
        println!("You drew {}! You win the jackpot! 🏆", random_number);
    }

    // === Table 2: Slot Machine ===
    println!("\n🎰 Welcome to Table 2: Slot Machine!");
    let symbols = ["Cherry", "Lemon", "Bell", "Star", "Diamond"];
    for i in 0..3 {
        let spin = symbols[rng.gen_range(0..symbols.len())];
        println!("Reel {}: {}", i + 1, spin);
    }
    println!("Spin complete! Did you win? Check for matching symbols!");

    // === Table 3: Roulette ===
    println!("\n🎡 Welcome to Table 3: Roulette!");
    let bet = rng.gen_range(0..37);
    let roulette_number = rng.gen_range(0..37);
    println!("You bet on {}. The wheel landed on {}.", bet, roulette_number);

    match bet {
        _ if bet == roulette_number => println!("🎉 Exact match! You win big!"),
        _ if bet % 2 == roulette_number % 2 => println!("😊 You win! The color matches (Red or Black)."),
        _ => println!("💔 You lose this round. Better luck next time!"),
    }

    // === High Rollers Lounge: All-In-One Game ===
    println!("\n🏛️ Welcome to the High Rollers Lounge: All-In-One Game!");
    let mut player_score = 0;

    for turn in 1..=5 {
        println!("\nTurn {}:", turn);

        // Dice roll
        let dice = rng.gen_range(1..=6);
        println!("You rolled a {}.", dice);

        // Decision-making with if-else
        if dice == 6 {
            println!("Lucky roll! You earn 2 points.");
            player_score += 2;
        } else if dice >= 4 {
            println!("Good roll! You earn 1 point.");
            player_score += 1;
        } else {
            println!("Unlucky roll! No points this time.");
        }

        // Bonus events using match
        match dice {
            1 => println!("Bonus Event: You found a treasure chest!"),
            5 => println!("Bonus Event: You encountered a friendly NPC!"),
            _ => (),
        }

        // Slot machine roll (nested loop)
        println!("Mini-game: Spin the slot machine for a bonus!");
        for i in 0..3 {
            let spin = symbols[rng.gen_range(0..symbols.len())];
            println!("Reel {}: {}", i + 1, spin);
        }
    }

    println!("\nGame Over! Your final score in the High Rollers Lounge is: {}", player_score);
}

