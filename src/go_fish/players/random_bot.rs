use crate::deck::{Deck, Rank};
use super::super::players::GoFishPlayer;

pub struct RandomBot {
    name: String,
    hand: Deck,
    books: Vec<Deck>,
}

impl RandomBot {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(), 
            hand: Deck::empty(), 
            books: Vec::new(), 
        }
    }
}

impl GoFishPlayer for RandomBot {
    fn name(&self) -> &str { &self.name }
    fn hand(&self) -> &Deck { &self.hand }
    fn hand_mut(&mut self) -> &mut Deck { &mut self.hand }
    fn books(&self) -> &Vec<Deck> { &self.books }
    fn books_mut(&mut self) -> &mut Vec<Deck> { &mut self.books }

    fn pick(&self) -> Option<Rank> {
        use rand::Rng;

        let amount_of_cards = self.hand.len();

        if amount_of_cards == 0 {
            return None; 
        }

        let random_index = rand::thread_rng().gen_range(0..amount_of_cards);

        Some(self.hand().card_at_index(random_index).rank)
    }

    fn ask(&self, other_players: Vec<(usize, &str)>) -> usize {
        use rand::Rng;

        let random_player_index = rand::thread_rng().gen_range(0..other_players.len());

        other_players[random_player_index].0
    }
}

use crate::{BuildInto, Input};

impl BuildInto for RandomBot {
    type Target = dyn GoFishPlayer;

    fn build() -> Box<Self::Target> {
        println!("What do you want to name the bot?");
        
        let name = Input::ask_for_name();

        Box::new(Self::new(&name))
    }
}