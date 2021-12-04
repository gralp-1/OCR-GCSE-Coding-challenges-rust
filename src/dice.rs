use rand::Rng;
use prettytable::{Table, row, cell};
/*Katarina is developing a two-player dice game.
The  players  roll  two  6-sided  dice  each  and  get  points  depending  on  what  they  
roll. There are 5 rounds in a game. In each round, each player rolls the two dice.
The rules are:
• The points rolled on each player’s dice are added to their score.
• If the total is an even number, an additional 10 points are added to their score.
• If the total is an odd number, 5 points are subtracted from their score.
• If they roll a double, they get to roll one extra die and get the number of points rolled added to 
their score.
• The score of a player cannot go below 0 at any point.
• The person with the highest score at the end of the 5 rounds wins.
• If  both  players  have  the  same  score  at  the  end  of  the  5  rounds,  they  each  roll  1  die  and  

whoever gets the highest score wins (this repeats until someone wins).
Only authorised players are allowed to play the game.
Where appropriate, input from the user should be validated.
Design, develop, test and evaluate a program that:
1. Allows  two  players  to  enter  their  details,  which  are  then  authenticated  to  ensure  that  they  are authorised players.
2. Allows each player to roll two 6-sided dice.
3. Calculates and outputs the points for each round and each player’s total score.
4. Allows the players to play 5 rounds.
5. If  both  players  have  the  same  score  after  5  rounds,  allows  each  player  to  roll  1  die  each  until  
someone wins.
6. Outputs who has won at the end of the 5 rounds.
7. Stores the winner’s score, and their name, in an external file.
8. Displays the score and player name of the top 5 winning scores from the external file. */

pub fn run() {
    let mut player_1 = auth(1);
    let mut player_2 = auth(2);
    let mut player_1_scores: Vec<String> = Vec::new();
    let mut player_2_scores: Vec<String> = Vec::new();


    println!("poggers");
    // Runs 5 rounds and outputs the scores for each round
    for _ in 0..5 {
        let scores: [i32; 2] = round(&player_1, &player_2);
        player_1.score += scores[0];
        player_2.score += scores[1];

        // Get the score and conver it to an &str
        let score_1 = player_1.score.to_string();
        let score_2 = player_2.score.to_string();

        // Add the score to the vector
        player_1_scores.push(score_1);
        player_2_scores.push(score_2);
    }

    let mut table = Table::new();
    table.add_row(row!["Round", player_1.name, player_2.name]);
    table.add_row(row!["1", player_1_scores[0], player_2_scores[0]]);
    table.add_row(row!["2", player_1_scores[1], player_2_scores[1]]);
    table.add_row(row!["3", player_1_scores[2], player_2_scores[2]]);
    table.add_row(row!["4", player_1_scores[3], player_2_scores[3]]);
    table.add_row(row!["5", player_1_scores[4], player_2_scores[4]]);


    table.printstd();


}

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

// runs one round of the game
fn round(player_1: &Player, player_2: &Player) -> [i32; 2]{
    let mut rng = rand::thread_rng();
    
    let p1_roll_1: i32 = rng.gen_range(1..6);
    let p1_roll_2: i32 = rng.gen_range(1..6);
    println!("{} rolled {} and {}", player_1.name, p1_roll_1, p1_roll_2);
    let p1_score = calc_score(p1_roll_1, p1_roll_2);


    let p2_roll_1: i32 = rng.gen_range(1..6);
    let p2_roll_2: i32 = rng.gen_range(1..6);
    println!("{} rolled {} and {}", player_2.name, p2_roll_1, p2_roll_2);
    let p2_score = calc_score(p2_roll_1, p2_roll_2);

    // returns it in an array for the main function to use
    return [p1_score, p2_score];
}

// Calculate the score for a round using the rules
fn calc_score(roll_1: i32, roll_2: i32) -> i32 {
    let mut total = roll_1 + roll_2;
    if total % 2 == 0 {
        total += 10;
    } else {
        total -= 5;
    }


    // Make sure total is not negative
    if total < 0 {
        total = 0;
    }
    return total;
}



