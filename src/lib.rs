pub mod go_fish;
pub mod war;
// pub mod blackjack;

pub mod card;
pub mod deck;

#[allow(non_snake_case)]
/// A module for user gathering user input easily. 
pub mod Input {
	/// Halts thread execution until the user presses enter. 
	pub fn wait() {
		println!("Press ENTER to continue.");
		std::io::stdin().read_line(&mut String::new()).expect("Something went wrong while prompting for input.");
	}

	/// Returns user's response (trimmed of whitespace) to a prompt.
	/// User should be prompted before. 
	pub fn response() -> String {
		let mut response = String::new();
		std::io::stdin().read_line(&mut response).expect("Something went wrong while prompting for input.");
		response.trim().to_string()
	}

	/// Returns if the user's response was positive (`y` or `yes`)(true) or negative (anything else)(false). 
	/// Works the same as `Input::response` but for yes/no questions. 
	pub fn yes_or_no() -> bool {
		let response = response().to_lowercase();

		println!();

		match response.as_str() {
			"y" | "yes" => true,
			_ => false,
		}
	}

	/// Asks the user for a usize until it passes `check`.
	/// `default` defines what the default value should be (should the user just press ENTER). 
	/// 
	/// User should be prompted before (indicating both the terms of `check` and what the default value is). 
	///
	pub fn ask_until<C>(check: C, default: Option<usize>) -> usize
	where 
		C: Fn(usize) -> bool
	{
		loop {
		    let response = response();

		    if default.is_some() && &response == "" {
		        return default.unwrap();
		    }

		    match &response.parse::<usize>() {
		        Ok(amount) if check(*amount) => return *amount, 
		        _ => println!("\n\"{}\" is not a valid response.\n", response),
		    }
		}
	}

	/// Asks the user for a name until it has at least one character.
	/// User should be prompted before. 
	pub fn ask_for_name() -> String {
		loop {
            let response = response();

            if response.len() == 0 {
                println!("Please enter a name.");
                println!();
            } else {
                return response;
            }
        }
	}

	/// Asks until the user picks a valid option from `options`, the names of which are created from the `name` function.
	///
	/// Panics if no options are provided. Will also return the only option if there is only one. 
	///
	/// User should be prompted before. 
	///
	pub fn pick_from_options<T, N>(options: &[T], name: N) -> usize 
	where 
		N: Fn(&T) -> &str,
	{
		let amount_of_options = options.len();

		if amount_of_options == 0 {
			panic!("No options provided.");
		}

		if amount_of_options == 1 {
			return 0;
		}

		println!("Options: ");
		for (index, option) in options.iter().enumerate() {
			let name = name(option);
			println!("({}) - {}", index + 1, name);
		}
		println!();

		println!("Pick an option (1-{}): ", amount_of_options);

		let index = ask_until(|input| input > 0 && input <= amount_of_options, None) - 1;
		println!();

		return index;
	}
}

/// A trait that defines a card game. 
pub trait CardGame {
	/// Should play a card game. 
	fn play(&mut self);
}

/// A trait that defines a `build` method for use in `builders` (defined below).
/// 
/// Note that `build` function types are `fn() -> Box<T>` where `T` is the `Target` type. 
pub trait BuildInto {
	/// The type the struct should be coerced into (a trait the struct implements).
    type Target: ?Sized;
    
    /// Should return a properly created and formatted object (through user input). 
    fn build() -> Box<Self::Target> where Self: Sized;
}

/// A trait that defines a builder, something that creates certain structs bound by certain traits.
pub trait Builder<T>
where 
	T: ?Sized
{
	/// A builder is defined as a `(&'static str, fn() -> Box<T>)`, a tuple of a static str (the name of what is being built) 
	/// and a function that creates a boxed item of what is being built from user input.
	/// 
	/// Build functions can be implemented using BuildInto<Target = _> or some other function that behaves the same.  
	///
	/// Should return the builders held by the builder.
	fn builders(&self) -> &[(&'static str, fn() -> Box<T>)];

	/// `Starts` the builder, begins prompting the user to create objects.
	fn start(&self) -> Box<T> {  
		let index = Input::pick_from_options(self.builders(), |builder| builder.0);

		(self.builders()[index].1)()
	}
}

/// A builder for card game players. 
pub struct CardGamePlayerBuilder<T>
where
	T: ?Sized 
{
    builders: Box<[(&'static str, fn() -> Box<T>)]>,
}

impl<T> CardGamePlayerBuilder<T>
where
	T: ?Sized
{
    /// Creates a new CardGamePlayerBuilder with the given names and builders. 
    pub fn new(builders: Box<[(&'static str, fn() -> Box<T>)]>) -> Self {
        Self {
            builders, 
        }
    }

    /// Returns a Vec of card game players generalized to their trait (for use in card games). 
    pub fn build_players(&self) -> Vec<Box<T>> {
    	let mut players: Vec<Box<T>> = Vec::new();

        loop {
            if players.len() < 1 {
                println!("Add a player.");
            } else if players.len() < 2 {
                println!("Add another player.");
            } else {
                println!("Do you want to add another player (y/n)?");

                if !Input::yes_or_no() {
                	println!();
                    break;
                }
            }
            println!();

            let player = self.start();
            println!();

            println!("Player created successfully.");
            println!();

            players.push(player);
        }

        players
    }
}

impl<T> Builder<T> for CardGamePlayerBuilder<T>
where
	T: ?Sized
{
    fn builders(&self) -> &[(&'static str, fn() -> Box<T>)] {
        &self.builders
    }
}

/// A builder for card games. 
pub struct CardGameBuilder {
    builders: Box<[(&'static str, fn() -> Box<dyn CardGame>)]>, 
}

impl CardGameBuilder {
	/// Creates a new CardGameBuilder with the given names and builders.
    pub fn new(builders: Box<[(&'static str, fn() -> Box<dyn CardGame>)]>) -> Self {
        Self {
            builders, 
        }
    }
}

impl Builder<dyn CardGame> for CardGameBuilder {
    fn builders(&self) -> &[(&'static str, fn() -> Box<dyn CardGame>)] {
        &self.builders
    }
}

/// The response a struct that implements MenuItem should return in its `run(&self)` method.
pub enum MenuItemResponse {
	/// If returned, menu execution continues.
	Continue, 
	/// If returned, menu execution stops. 
	Quit, 
}

/// A trait that defines items in a menu. 
pub trait MenuItem {
	/// Should run the menu item.
    fn run(&self) -> MenuItemResponse;
}

/// A menu (main menu right now).
pub struct Menu {
    items: Box<[(&'static str, Box<dyn MenuItem>)]>,
}

impl Menu {
	/// Creates a new menu with the given items `items`.
    pub fn new(items: Box<[(&'static str, Box<dyn MenuItem>)]>) -> Self {
        Self {
            items,
        }
    }

    /// Starts the menu. 
    pub fn begin(&self) {
        loop {
	        println!("Main Menu");
    		println!();

            let item_index = Input::pick_from_options(&self.items, |option| &option.0);

            let response = self.items[item_index].1.run();

            match response {
            	MenuItemResponse::Continue => (), 
            	MenuItemResponse::Quit => break, 
            }
        }

        println!("Thank you for playing!");
    }
}

impl MenuItem for CardGameBuilder {
	fn run(&self) -> MenuItemResponse {
		loop {
	        println!("Which game do you want to play?");
	        println!();

	        let mut game = self.start();
	        game.play();

	        println!("Do you want to play another game (y/n)?");

	        if !Input::yes_or_no() {
	            break;
	        }
    	}	

    	MenuItemResponse::Continue
	} 
}

/// A struct that can use user input to change the global card set in the `card` module.
pub struct CardSetChanger {
    card_set_filenames: Vec<String>,
}

impl CardSetChanger {
	/// Returns a new CardSetChanger.
    pub fn new() -> Self {
        use std::fs;

        Self {
            card_set_filenames: fs::read_dir("card_sets")
                .expect("`card_sets` directory should exist.")
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.file_name().into_string().expect(&format!("{:?} must be Unicode encoded.", entry)))
                .collect(),
        }
    }

    /// Changes the global card set in module `card`.
    fn change_to(&self, card_set_at_index: usize) {
        let card_set_filename = &self.card_set_filenames[card_set_at_index];
        
        crate::card::set_card_set(card_set_filename);
    } 

    /// Has the player choose which card set to use. 
    pub fn choose(&self) {
        let card_set_index = Input::pick_from_options(&self.card_set_filenames, |option| &option.strip_suffix(".txt").expect("Should never fail."));
        self.change_to(card_set_index);

        println!("Do you want a preview (y/n)?");

        let preview = Input::yes_or_no();

        if preview {
        	use crate::deck::Deck;
        	println!();
        	println!("{}", Deck::shuffled_52_card().card_at_index(0).as_printable());
        }

        println!();
    }
}

impl MenuItem for CardSetChanger {
	fn run(&self) -> MenuItemResponse {
        println!("Which card set do you want to use?");
        println!();

        self.choose();	

        MenuItemResponse::Continue
	} 
}

/// A struct that lets users quit from menus.
pub struct Quit;

impl MenuItem for Quit {
	fn run(&self) -> MenuItemResponse {
		println!("Do you want to quit (y/n)?");

		let quit = Input::yes_or_no();

		println!();

		if quit {
			MenuItemResponse::Quit
		} else {
			MenuItemResponse::Continue
		}
	}
}