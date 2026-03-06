use crate::deck::Deck;

use super::game::Action;

pub trait BlackjackPlayer {
	/// Should return the name of the player.
	fn name(&self) -> &str;

	/// Note: money is stored as a usize, representing the amount of cents.

	/// Should return how much money the player has.  
	fn bankroll(&self) -> usize;

	/// Should remove and return an amount of money from the player. 
	fn bet(&mut self, betting_minimum: usize, betting_maximum: usize) -> usize;

	/// Should add the amount `amount` to the player's money. 
	fn pay(&mut self, amount: usize);

	/// Should return an action from `actions`. 
	fn choose_action(&self, hand: &Deck, actions: &[Action]) -> Action; 
}
