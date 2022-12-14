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

pub fn get_outcome_score(card: &str) -> u32 {
    match card {
        _ if card == "X" => return 0,
        _ if card == "Y" => return 3,
        _ if card == "Z" => return 6,
        _ => return 0,
    }
}

pub fn get_card_score_by_outcome(opp: &str, outcome: &str) -> u32 {
    if outcome == "X" {
        match opp  {
            _ if opp  == "A" => return 3,
            _ if opp  == "B" => return 1,
            _ if opp  == "C" => return 2,
            _ => return 0,
        }
    } else if outcome == "Y" {
        match opp  {
            _ if opp  == "A" => return 1,
            _ if opp  == "B" => return 2,
            _ if opp  == "C" => return 3,
            _ => return 0,
        }
    } else {
        match opp  {
            _ if opp  == "A" => return 2,
            _ if opp  == "B" => return 3,
            _ if opp  == "C" => return 1,
            _ => return 0,
        }
    }
}

pub fn play_round(round_str: &str) -> u32 {
    let choices = round_str
        .split(" ")
        .collect::<Vec<&str>>();

    if choices.len() < 2 {
        return 0;
    }

    return get_outcome_score(choices[1]) + get_card_score_by_outcome(choices[0], choices[1]);
}
