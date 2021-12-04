// Just a struct to hold player info
pub struct Player {
	name: String,
	score: i32
}


// Gets the name from terminal, and validates it with regex and returns a player
pub fn auth(count: u8) -> Player{
	let mut line = String::new();
	
	println!("Enter Player {} name:", count);
	std::io::stdin().read_line(&mut line).unwrap().to_string();
    
	let player = Player {
	    name: line.to_owned().replace("\n", ""),
	    score: 0
	};
	return player;
}