use std::fs;

#[derive(Debug)]
enum CardAction {
    Increment(i32),
    DealInto,
    Cut(i32),
    NotYet(String),
}

fn line_to_action(s: String) -> CardAction {
    use CardAction::*;
    let chunks = s.split(" ").collect::<Vec<_>>();
    let maybe_number = chunks[chunks.len() - 1].parse();
    match (chunks, maybe_number) {
        (my_chunks, Ok(n)) => match my_chunks[0] {
            "cut" => Cut(n),
            "deal" => Increment(n),
            _ => panic!("it broke {:?}", my_chunks),
        },
        (_, Err(_)) => DealInto,
    }
}

fn apply_action(deck: Vec<i32>, action: CardAction) -> Vec<i32> {
    match action {
        CardAction::DealInto => deck.into_iter().rev().collect(),
        CardAction::Cut(count) => {
            let c = if count >= 0 {
                count as usize
            } else {
                (deck.len() as i32 - (-1 * count)) as usize
            };
            let top_half = &deck[..c];
            let bottom_half = &deck[c..];
            [bottom_half, top_half].concat()
        }
        CardAction::Increment(n) => {
            let mut new_deck = vec![0; deck.len()];
            for i in 0..deck.len() {
                let new_index = (i * n as usize) % deck.len();
                new_deck[new_index] = deck[i];
            }
            new_deck
        }
        _ => panic!("at the goat"),
    }
}

pub fn main() -> std::io::Result<()> {
    let f = fs::read_to_string("input/day22.txt")?.trim().to_string();
    let actions: Vec<_> = f
        .split("\n")
        .map(|s| s.to_string())
        .map(line_to_action)
        .collect();
    let mut first_deck = vec![];
    for c in 0..=10006 {
        first_deck.push(c);
    }
    for action in actions {
        first_deck = apply_action(first_deck, action);
    }

    for (index, the_card) in first_deck.iter().enumerate() {
        if the_card == &2019 {
            dbg!(index);
        }
    }

    // now up to 11

    let mut big_deck = vec![];
    let this_many_cards = 119315717514047 as i64;
    for c in 0..=this_many_cards {
        big_deck.push(c);
    }

    //dbg!(part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deal_into() {
        let my_deck = vec![1, 2, 3];
        assert_eq!(apply_action(my_deck, CardAction::DealInto), vec![3, 2, 1]);
    }

    #[test]
    fn test_cut_three() {
        let my_deck = vec![10, 20, 30, 40, 50];
        assert_eq!(
            apply_action(my_deck, CardAction::Cut(3)),
            vec![40, 50, 10, 20, 30]
        );
    }

    #[test]
    fn test_cut_negative_three() {
        let my_deck = vec![10, 20, 30, 40, 50];
        assert_eq!(
            apply_action(my_deck, CardAction::Cut(-3)),
            vec![30, 40, 50, 10, 20]
        );
    }

    #[test]
    fn test_cut_increment_three() {
        let my_deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(
            apply_action(my_deck, CardAction::Increment(3)),
            vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3,]
        );
    }
}
