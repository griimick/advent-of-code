pub fn main() {
    println!(
        "{:?}",
        include_str!("../input.txt")
        .split("\n")
        .into_iter()
        .filter(|s| !s.is_empty())
        .map(|round_str| play_round(round_str))
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>(),
    );
}

pub fn get_card_score(card: &str) -> u32 {
    match card {
        _ if card == "X" => return 1,
        _ if card == "Y" => return 2,
        _ if card == "Z" => return 3,
        _ => return 0,
    }
}

pub fn get_outcome_score(opp: &str, you: &str) -> u32 {
    if you == "X" && opp == "A" {
        return 3;
    }

    if you == "Y" && opp == "B" {
        return 3;
    }

    if you == "Z" && opp == "C" {
        return 3;
    }

    if you == "X" && opp == "C" {
        return 6;
    }

    if you == "Y" && opp == "A" {
        return 6;
    }

    if you == "Z" && opp == "B" {
        return 6;
    }

    return 0;
}

pub fn play_round(round_str: &str) -> u32 {
    let choices = round_str
        .split(" ")
        .collect::<Vec<&str>>();

    if choices.len() < 2 {
        return 0;
    }

    return get_card_score(choices[1]) + get_outcome_score(choices[0], choices[1]);
}
