use crate::deck::{Deck, Card, print_side_by_side};
use super::super::players::WarPlayer;

pub struct HumanWarPlayer {
    name: String, 
    pub hand: Deck,
}

impl HumanWarPlayer {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            hand: Deck::empty(),
        }
    }
}

impl WarPlayer for HumanWarPlayer {
    fn name(&self) -> &str { &self.name } 
    fn hand(&self) -> &Deck { &self.hand }
    fn hand_mut(&mut self) -> &mut Deck { &mut self.hand }
    fn draw(&mut self) -> Option<Card> {
        let only_show = 5;
        println!("Your cards: ");
        print!("{}", self.hand().as_printable().hide_faces().only_show(only_show));

        let hand_length = self.hand().len();
        if hand_length > only_show {
            println!("(Only showing {} out of {} cards in hand)", only_show, hand_length);
        }
        println!();

        self.wait("Press ENTER to draw a card.");

        let drawn_card = if let Some(card) = self.hand_mut().draw() {
            card 
        } else { 
            return None; 
        };

        let only_show_after_draw = only_show - 1 /* for drawn card */;

        let deck_minus_drawn_card_string = self.hand()
            .as_printable()
            .hide_faces()
            .only_show(only_show_after_draw)
            .to_string();

        let drawn_card_string = drawn_card.as_printable().to_string();

        println!("You drew a: ");
        if deck_minus_drawn_card_string.len() == 0 {
            print_side_by_side(&[drawn_card_string], "");
        } else {
            print_side_by_side(&[deck_minus_drawn_card_string, drawn_card_string], " ");
        }

        let hand_length = self.hand().len();
        if hand_length > only_show_after_draw {
            println!("(Only showing {} out of {} cards in hand)", only_show_after_draw, hand_length);
        }
        println!();

        Some(drawn_card)
    }

    fn war_draw(&mut self, amount_of_cards: usize) -> Deck {
        let mut drawn_cards = Deck::empty();

        if self.hand().len() == 0 {
            println!("You do not have enough cards to wage war!");
            println!();
            return drawn_cards;
        }

        let only_show = amount_of_cards + 5;

        // println!("You are now waging war!");
        println!("Your cards: ");
        print!("{}", self.hand().as_printable().hide_faces().only_show(only_show));

        let hand_length = self.hand().len();
        if hand_length > only_show {
            println!("(Only showing {} out of {} cards in hand)", only_show, hand_length);
        }
        println!();

        for _ in 0..amount_of_cards {
            let drawn_card = if let Some(card) = self.hand_mut().draw() {
                card 
            } else { 
                break; 
            };

            self.wait("Press ENTER to draw a card.");

            let only_show_after_draw = only_show - (drawn_cards.len() + 1);

            let deck_minus_drawn_cards_string = self.hand()
                .as_printable()
                .hide_faces()
                .only_show(only_show_after_draw)
                .to_string();

            // If you either run of out of cards or draw as many as specified, 
            // print the last drawn card next to the deck and previously drawn cards. 
            if self.hand().len() == 0 || drawn_cards.len() == amount_of_cards - 1 {
                println!("You are waging: ");
                let drawn_card_string = drawn_card.as_printable().to_string();
                let drawn_cards_before_adding_drawn_card_string = drawn_cards.as_printable().hide_faces().to_string();
                print_side_by_side(&[deck_minus_drawn_cards_string, drawn_cards_before_adding_drawn_card_string, drawn_card_string], " ");
                drawn_cards.stack(drawn_card);
            // Otherwise print the drawn cards next to the deck 
            } else {
                drawn_cards.stack(drawn_card);
                let drawn_cards_string = drawn_cards.as_printable().hide_faces().to_string();
                print_side_by_side(&[deck_minus_drawn_cards_string, drawn_cards_string], " ");
            }

            let hand_length = self.hand().len();
            if hand_length > only_show_after_draw {
                println!("(Only showing {} out of {} cards in hand)", only_show_after_draw, hand_length);
            }
            println!();
        }

        drawn_cards
    }

    fn inform(&self) -> bool { true }
}

use crate::{BuildInto, Input};

impl BuildInto for HumanWarPlayer {
    type Target = dyn WarPlayer;

    fn build() -> Box<Self::Target> {
        println!("What do you want to name your player?");
        
        let name = Input::ask_for_name();

        Box::new(Self::new(&name))
    }
}