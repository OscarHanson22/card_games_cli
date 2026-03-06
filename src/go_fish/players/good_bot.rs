use crate::deck::Deck;
use crate::card::Card;
use crate::player::Player;

pub struct Bot {
	name: String, 
	hand: Deck, 
	books: Vec<Deck>, 
}

impl Bot {
	pub fn new(name: &str) -> Self {
		Self {
			name: name.to_string(), 
			hand: Deck::empty(), 
			books: Vec::new(),
		}
	}
}

impl Player for Bot {
	fn name(&self) -> &str { &self.name }
    fn hand(&self) -> &Deck { &self.hand }
    fn hand_mut(&mut self) -> &mut Deck { &mut self.hand }
    fn books(&self) -> &Vec<Deck> { &self.books }
    fn books_mut(&mut self) -> &mut Vec<Deck> { &mut self.books }

    fn pick(&self) -> Option<Card> {
        todo!()
    }

    fn ask(&self, other_players: Vec<(usize, &str)>) -> usize {
        todo!()
    }
}

/**
 * Needs to remember the other player's cards and potential cards  
 * 
 * for each rank, keep track of the probability of where the cards are
 * 
 * Unknown Rank: Ace -> vec![
 * 		PossiblePlacement::InPlayerHand { // if owned
 * 			name: self.name,
 * 			rank_info: RankInfo {
 * 				guaranteed: however many i have, 
 * 				lower_bound: however many i have, 
 * 				upper_bound: however many i have (hmih),
 * 			}
 * 		}, 
 * 
 * 		in this case, all the other information would be adjusted accordingly
 * 		
 * 		PossiblePlacement::InPlayerHand { // example other player
 * 			name: Player1,
 * 			rank_info: RankInfo {
 * 				guaranteed: 0, 
 * 				lower_bound: 0, 
 * 				upper_bound: 3,
 * 			}
 * 		}, 
 * 
 * 		PossiblePlacement::InDeck { // example deck
 * 			rank_info: RankInfo {
 * 				guaranteed: 0, 
 * 				lower_bound: 0, 
 * 				upper_bound: 4 - hmih,
 * 			}
 * 		}, 
 * 
 * 		------------------------------------------------
 * 
 * 		PossiblePlacement::InPlayerHand { // for every player, even self
 * 			name: Player1,
 * 			rank_info: RankInfo {
 * 				guaranteed: 0, 
 * 				lower_bound: 0, 
 * 				upper_bound: 3,
 * 			}
 * 		}, 
 * 
 * 		PossiblePlacement::InPlayerHand { // like so
 * 			name: Player2,
 * 			rank_info: RankInfo {
 * 				guaranteed: 0, 
 * 				lower_bound: 0, 
 * 				upper_bound: 3,
 * 			}
 * 		}, 
 * 
 * 		PossiblePlacement::InDeck {
 * 			rank_info: RankInfo {
 * 				guaranteed: 0, 
 * 				lower_bound: 0, 
 * 				upper_bound: 4,
 * 			}
 * 		}, 
 * ]
 */

struct GameInfo {

}

struct RankInfo {
	guaranteed: usize, 
	lower_bound: usize, 
	upper_bound: usize, 
} 

enum PossiblePlacement {
	InPlayerHand {
		name: String, 
		rank_info: RankInfo, 
	},

	InDeck {
		rank_info: RankInfo, 
	},
}

// unknown card -> RankInfo { 

}

Vec<PossiblePlacement>;


trait Player {
	fn name(&self) -> &str { &self.name }
    fn hand(&self) -> &Deck { &self.hand }
    fn hand_mut(&mut self) -> &mut Deck { &mut self.hand }
    fn books(&self) -> &Vec<Deck> { &self.books }
    fn books_mut(&mut self) -> &mut Vec<Deck> { &mut self.books }

    fn pick(&self) -> Option<Card> {
        todo!()
    }

    fn ask(&self, other_players: Vec<(usize, &str)>) -> usize {
        todo!()
    }
}