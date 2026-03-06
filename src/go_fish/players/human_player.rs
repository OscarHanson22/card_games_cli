use crate::deck::{Deck, Rank, print_side_by_side};

use super::super::players::GoFishPlayer;
use super::super::game::PublicInformation;

pub struct HumanPlayer {
    name: String, 
    hand: Deck, 
    books: Vec<Deck>, 
}

impl HumanPlayer {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(), 
            hand: Deck::empty(),
            books: Vec::new(),
        }
    }
}

impl GoFishPlayer for HumanPlayer {
    fn name(&self) -> &str { &self.name }
    fn hand(&self) -> &Deck { &self.hand }
    fn hand_mut(&mut self) -> &mut Deck { &mut self.hand }
    fn books(&self) -> &Vec<Deck> { &self.books }
    fn books_mut(&mut self) -> &mut Vec<Deck> { &mut self.books }

    fn handle_public_information(&mut self, public_information: PublicInformation) {
        println!();
        println!("You look around: ");

        for player_info in &public_information.player_info {
            if player_info.name == self.name() {
                continue;
            }

            println!("    {} has {} cards and {} books.", player_info.name, player_info.amount_of_cards, player_info.amount_of_books);
        }

        println!("    There are {} cards left in the deck.", public_information.deck_length);
    }

    fn pick(&self) -> Option<Rank> {
        use std::io;

        let amount_of_cards = self.hand().len();

        if amount_of_cards == 0 {
            return None; 
        }

        println!("Your hand:");
        println!();
        print!("{}", self.hand().as_printable().peek(3));
        println!();
        println!("Pick a rank from your cards to ask for: ");

        let rank = loop {
            let mut input = String::new();

            io::stdin().read_line(&mut input).expect("Something went wrong.");

            let rank_string = input.trim();
            let cards_with_rank = self.hand().cards(|card| card.rank.to_string().to_lowercase() == rank_string.to_lowercase());

            if cards_with_rank.len() > 0 {
                break cards_with_rank.card_at_index(0).rank;
            } else {
                println!("\"{}\" is not a rank you can choose.", rank_string);
            }
        };

        Some(rank)
    }

    fn ask(&self, other_players: Vec<(usize, &str)>) -> usize {
        let amount_of_other_players = other_players.len();

        if amount_of_other_players == 1 {
            println!("Asking {}.", other_players[0].1);
            return other_players[0].0;
        }

        let other_player_names: Vec<&str> = other_players.iter()
            .map(|(_, name)| *name)
            .collect();

        println!("Pick a player:");
        println!();

        let other_player_names_separator = " | ";

        print_side_by_side(&other_player_names, other_player_names_separator);

        for i in 1..=amount_of_other_players {
            let other_player_name_length = other_player_names[i - 1].len();

            let left_padding = " ".repeat(((other_player_name_length - 3) as f64 / 2.0).floor() as usize);
            let right_padding = " ".repeat(((other_player_name_length - 3) as f64 / 2.0).ceil() as usize);

            print!("{}({}){}{}", left_padding, i, right_padding, " ".repeat(other_player_names_separator.len()));
        }
        println!("\n");

        println!("Ask player 1-{}: ", amount_of_other_players);

        let index = loop {
            let mut input = String::new();

            std::io::stdin().read_line(&mut input).expect("Something went wrong.");

            match input.trim().parse::<usize>() {
                Ok(i) if i <= amount_of_other_players => {
                    break i - 1;
                },

                Ok(i) => {
                    println!("{i} is not a player you can pick.")
                }

                Err(_) => continue,
            }
        };

        other_players[index].0
    }

    fn inform(&self) -> bool { true }
}

use crate::{BuildInto, Input};

impl BuildInto for HumanPlayer {
    type Target = dyn GoFishPlayer;

    fn build() -> Box<Self::Target> {
        println!("What do you want to name your player?");
        
        let name = Input::ask_for_name();

        Box::new(Self::new(&name))
    }
}