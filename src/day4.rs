use std::collections::HashSet;

const SEPARATOR: &str = "|";

fn get_score(card_text: &str) -> u32 {
    let mut tokens: Vec<&str> = card_text.split(' ').collect();

    // Considering it says: "large pile of colorful cards", its best to use a data set with constant lookup time
    let mut winning_numbers = HashSet::new();

    let mut winning_number_stage = true;

    let mut score: u32 = 0;

    for token in tokens {

        if winning_number_stage {

            if token.eq(SEPARATOR) {
                winning_number_stage = false;
            }
            else {
                let val = token.parse::<u32>();
                if val.is_ok() {
                    winning_numbers.insert(val.unwrap());
                }
            }
        }
        else {
            let val = token.parse::<u32>();
            if val.is_ok() && winning_numbers.contains(&val.unwrap()) {
                score = if score == 0 {1} else {score * 2}
            }
        }
    }

    return score;
}


pub fn day4(){
    let cards = [
        String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
        String::from("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
        String::from("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
        String::from("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
        String::from("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
        String::from("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11")
    ];
    let mut total_score = 0;
    for card in cards {
        total_score += get_score(card.as_str());
    }
    println!("Total score: {}", total_score)
}