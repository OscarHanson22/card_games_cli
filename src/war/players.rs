pub mod bot;
pub mod human;

use crate::deck::{Deck, Card, print_side_by_side};

pub trait WarPlayer {
    fn name(&self) -> &str;
    fn hand(&self) -> &Deck;
    fn hand_mut(&mut self) -> &mut Deck;
    fn draw(&mut self) -> Option<Card> { self.hand_mut().draw() }
    fn war_draw(&mut self, amount_of_cards: usize) -> Deck {
        let mut drawn_cards = Deck::empty();

        if self.hand().len() == 0 {
            println!("{} does not have enough cards to wage war!", self.name());
            println!();
            return drawn_cards;
        }

        for _ in 0..amount_of_cards {
            if let Some(drawn_card) = self.hand_mut().draw() {
                drawn_cards.stack(drawn_card);
            } else {
                break;
            }
        }

        if drawn_cards.len() == 0 {
            return drawn_cards;
        }

        let last_card = drawn_cards.draw().expect("Should have drawn at least one card.");
        let last_card_string = last_card.as_printable().to_string();
        let remaining_cards_string = drawn_cards.as_printable().hide_faces().to_string();

        println!("{} wages: ", self.name());
        print_side_by_side(&[remaining_cards_string, last_card_string], " ");
        println!();
                
        drawn_cards.stack(last_card);

        drawn_cards
    }

    fn inform(&self) -> bool { false }
    fn wait(&self, message: &str) { 
        if self.inform() {
            println!("{}", message);
            std::io::stdin().read_line(&mut String::new()).expect("Something went wrong.");
        }
    }
}