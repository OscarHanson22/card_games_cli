use crate::deck::{Deck, Card, Rank};
use super::players::BlackjackPlayer;

#[derive(Copy, Clone)]
pub enum Action {
	Hit, 
	Stand, 
	Split,
	DoubleDown,
	None,
}

pub struct Blackjack {
	deck: Deck, 
	dealer: Deck,
	hands: Vec<Deck>,
	splits: Vec<Vec<Deck>>,
	bets: Vec<usize>, 
	insurance_bets: Vec<usize>,
	double_down_cards: Vec<Card>, 


	players: Box<[Box<dyn BlackjackPlayer>]>,
}

impl Blackjack {
	pub fn new(players: Box<[Box<dyn BlackjackPlayer>]>) -> Self {
		Self {
			deck: Deck::shuffled_52_card(), 
			players, 
		}
	}

	fn hand_values(hand: &Deck) -> Vec<usize> {
		let mut hand_value = 0;
		let mut amount_of_aces = 0;

		for card in hand {
			if card.rank == Rank::Ace {
				amount_of_aces += 1;
			}

			hand_value += match card.rank {
				Rank::Two => 2,
	            Rank::Three => 3,
	            Rank::Four => 4, 
	            Rank::Five => 5, 
	            Rank::Six => 6, 
	            Rank::Seven => 7, 
	            Rank::Eight => 8, 
	            Rank::Nine => 9, 
	            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => 10,
	            Rank::Ace => 11, 
        	};
		}

		let mut values = Vec::new();

		for _ in 0..amount_of_aces + 1 {
			values.push(hand_value);
			hand_value -= 10;
		}

		return values;
	}

	fn is_ten_card(card: Card) -> bool {
		match card.rank {
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => true,
            _ => false,
    	}
	}

	fn player(&self, player_index: usize) -> &Box<dyn BlackjackPlayer> {
		&self.players[player_index]
	}

	fn player_mut(&mut self, player_index: usize) -> &mut Box<dyn BlackjackPlayer> {
		&mut self.players[player_index]
	}

	fn play_turn(&mut self, player: usize, hand: &mut Deck, bet: usize, double_down_hand: &mut Deck, has_split: bool) {
		let mut possible_actions = vec![Action::Hit, Action::Stand];

		if self.player(player).bankroll() > bet {
			possible_actions.push(Action::DoubleDown);
		}

		// two of a kind (split condition)
		if hand.card_at_index(0).rank == hand.card_at_index(1).rank {
			possible_actions.push(Action::Split);
		}

		// if 
		loop {
			let action = self.player(player).choose_action(&hand, &possible_actions);

			match action {
				Action::Hit => {
					let drawn_card = self.deck.draw().expect("Should never fail.");
					hand.stack(drawn_card);
				},

				Action::Stand => {
					break;
				},
				
				Action::Split => {
					let first_hand = hand.card_at_index(0);
					let second_hand = hand.card_at_index(1);
					self.play_turn(player, hand, bet, double_down_hand, true);
				},
				
				Action::DoubleDown => {
					let drawn_card = self.deck.draw().expect("Should never fail.");
					double_down_hand.stack(drawn_card);
				},

				_ => panic!("Should never occur."),
			}
		}
	}
}

fn format_money(amount: usize) -> String {
	let mut amount_string = amount.to_string();
	amount_string.insert(amount_string.len() - 3, '.');
	
	format!("${}", amount_string)
}

use crate::CardGame;

impl CardGame for Blackjack {
	fn play(&mut self) {
		let amount_of_players = self.players.len();

		let mut player_in: Vec<bool> = (0..amount_of_players).into_iter().map(|_| true).collect();

		// let betting = true; // might add later for casual play
		let betting_minimum = 200; // in cents
		let betting_maximum = 50000; // in cents
		let mut bets: Vec<usize> = Vec::new();

		// collect bets from each player before the dealing begins

		println!("Collecting bets...");
		println!("Bets are between {} and {}.", format_money(betting_minimum), format_money(betting_maximum));
		println!();

		for player in 0..amount_of_players {
			let bet = self.player_mut(player).bet(betting_minimum, betting_maximum);
			
			println!("{} bets {}!", self.player(player).name(), format_money(bet));
			println!();

			bets.push(bet);
		}

		let mut dealer: Deck = Deck::empty();
		let mut hands: Vec<Deck> = (0..amount_of_players).into_iter().map(|_| Deck::empty()).collect();

		// deal to all players one card, one to dealer face-up, deal to players, one face down
		// player cards are face up

		for player in 0..amount_of_players {
			let card = self.deck.draw().expect("Should never fail.");
			hands[player].stack(card);	
		}

		let card = self.deck.draw().expect("Should never fail.");
		dealer.stack(card);

		for player in 0..amount_of_players {
			let card = self.deck.draw().expect("Should never fail.");
			hands[player].stack(card);	
		}

		let card = self.deck.draw().expect("Should never fail.");
		dealer.stack(card);

		for player in 0..amount_of_players {
			self.player(player).choose_action(&hands[player], &vec![Action::None]);
		}

		// check for naturals (ace and ten value card)
		// if player has natural, they get their 1.5 * their bet, 
		// if the dealer has natural, they collect all the bets of those without naturals
		// if dealer and players have naturals, players recall their bet and a new round begins


		// if the dealer's face-up card is an ace, all players have the option of making a side-bet up to half their original bet
		// if the dealer's face-down card is a ten card (natural blackjack), it is turned up and players that made insurance bets get double their insurance bets
		// if the dealer's face-down card is not a ten card, then all players lose their insurance bets

		let mut insurance_bets: Vec<usize> = Vec::new();

		if dealer.card_at_index(0).rank == Rank::Ace {
			for player in 0..amount_of_players {
				let insurance_bet_maximum = bets[player] / 2;
				let insurance_bet = self.player_mut(player).bet(betting_minimum / 2, insurance_bet_maximum);
				insurance_bets.push(insurance_bet);
			}

			if Self::is_ten_card(dealer.card_at_index(1)) {
				for player in 0..amount_of_players {
					self.player_mut(player).pay(2 * insurance_bets[player]); // 2-to-1 insurance payout
					
					// if a player had a natural, then they 
					if Self::hand_values(&hands[player])[0] == 21 {
						self.player_mut(player).pay(2 * bets[player]);
					}	
				}

				// end round because dealer has natural
			} 
		} 

		// if the dealer has a natural with a ten card on top
		if Self::hand_values(&dealer)[0] == 21 {
			// payout even to all players that also have a natural
			for player in 0..amount_of_players {
				if Self::hand_values(&hands[player])[0] == 21 {
					self.player_mut(player).pay(bets[player]);
				}	
			}

			// end round because dealer has natural
		}

		

		// otherwise, if the dealer doesn't have a natural, payout if any players have a natural 
		for player in 0..amount_of_players {
			if Self::hand_values(&hands[player])[0] == 21 {
				player_in[player] = false;

				self.player_mut(player).pay(3 * bets[player] / 2 + bets[player]);
			}
		}

		// stored because they are face down until after settlement
		let mut double_down_hands: Vec<Deck> = (0..amount_of_players).into_iter().map(|_| Deck::empty()).collect();
		
		for player in 0..amount_of_players {
			if !player_in[player] {
				continue;
			} 

			let mut possible_actions = vec![Action::Hit, Action::Stand];

			if self.player(player).bankroll() > bets[player] {
				possible_actions.push(Action::DoubleDown);
			}

			// two of a kind (split condition)
			if hands[player].card_at_index(0).rank == hands[player].card_at_index(1).rank {
				possible_actions.push(Action::Split);
			}

			// if 
			loop {
				let action = self.player(player).choose_action(&hands[player], &possible_actions);

				match action {
					Action::Hit => {
						let drawn_card = self.deck.draw().expect("Should never fail.");
						hands[player].stack(drawn_card);
					},

					Action::Stand => {
						break;
					},
					
					Action::Split => {

					},
					
					Action::DoubleDown => {
						let drawn_card = self.deck.draw().expect("Should never fail.");
						hands[player]
					},

					_ => panic!("Should never occur."),
				}
			}
		}

		// The players then go through and either stand (do nothing) or hit (get a card from the dealer)
		// if players bust, they lose their bets
		// account for soft hand (ace can be 1 or 11)

		// if the player has two of a kind, they have the option to split, 
		// splitting the hand will require another equal bet
		// 		each game is played normally
		// if splitting a pair of aces
		// 		each ace is automatically given one card and cannot be hit on
		// 		if the given card is a ten-card then they get only the bet on that split (no 1.5 *)

		// if a player's hand totals to 9, 10, or 11, the player has the option to double down
		// player can place bet equal to original bet (doubling down)
		// and the dealer only gives the player just one card (face down) which is only turned up when settlement ends

		// for player in 0..amount_of_players {
		// 	if hands[player][0].card_at_index(0).rank == hands[player][0].card_at_index(1).rank {
		// 		// split condition
		// 	}

		// 	for pla
		// }

		// dealer's face down card is turned up 
		// if the total is 17 or more, dealer must stand
		// if the total is 16 or less, dealer must hit
		// if the dealer has an ace and counting it as 11 would be 17 or over, they must stand,
		// if the dealer has an ace and counting it as 11 would be 16 or under, they must hit, 
		// 		if they bust, the ace can be counted as 1

		// Once all players have gone
		// if the player achieves blackjack (and the dealer doesn't) they get 1.5 * their bet back
		// if the player's hand is better than the dealer, they get their bet back
		// if the player's hand is worse than the dealer's, they lose their hand
		// if the player's hand is the same as the dealer's, they get their bet back

		// 
	}
}