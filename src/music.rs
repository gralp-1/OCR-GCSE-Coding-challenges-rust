use rand::*;
use regex::Regex;
use serde_json::*;
use std::fs::File;
use std::io::Read;
use prettytable::*;
mod utils
/*
Noel is creating a music quiz game.
The  game  stores  a  list  of  song  names  and  their  artist
(e.g. the band or solo artist name). The player needs to
try and guess the song name.
The game is played as follows:
• A random song name and artist are chosen.
• The artist and the first letter of each word in the song title are displayed.
• The user has two chances to guess the name of the song.
• If the user guesses the answer correctly the first time, they score 3 points. If the user guesses
the answer correctly the second time they score 1 point. The game repeats.
• The game ends when a player guesses the song name incorrectly the second time.
Only authorised players are allowed to play the game.
Where appropriate, input from the user should be validated.
Design, develop, test and evaluate a system that:
1.  Allows  a  player  to  enter  their  details,  which  are  then  authenticated  to  ensure  that  they  are  an
authorised player.
2. Stores a list of song names and artists in an external file.
3. Selects a song from the file, displaying the artist and the first letter of each word of the song title.
4. Allows the user up to two chances to guess the name of the song, stopping the game if they guess
a song incorrectly on the second chance.
5. If the guess is correct, add the points to the player’s score depending on the number of guesses.
6. Displays the number of points the player has when the game ends.
7. Stores the name of the player and their score in an external file.
8. Displays the score and player name of the top 5 winning scores from the external file.
*/

// TODO: write playerdata (steps 7 & 8)
pub fn run() {
    let mut player = utils::auth(1);
    player.score += round();
    println!("Final Score: {}", player.score);
}

fn round() -> i32 {
    let mut rng = rand::thread_rng();
    let songs = load_songs();
    let song_index = rng.gen_range(0..62);
    // A regex to make sure the song name is in the right format
    let root = &songs[8]["tracks"][song_index]["track"];
    let name: String = root["name"].to_string();
    let artist: String = root["album"]["artists"][0]["name"].to_string();

    let mut guess = String::new();
    let mut guess_count = 0;
    let mut score = 0;


    table!(
        ["Artist", "Hidden Song Name"],
        [artist, song_matching(name.to_owned())]
    ).printstd();

    loop {
        if guess_count == 2 {
            break;
        }
        println!("Guess: ");
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        guess = guess.trim().to_string();
        if guess.replace("\"", "") == name.replace("\"", "") {
            score += 3;
            break;
        } else {
            guess_count += 1;
	        println!("{}", guess);
        }
    }
    return score;
}

fn song_matching(name: String) -> String {
    // Print the artist and the first letter of each word in the song title
    let re = Regex::new(r"\b\w").unwrap();
    let mut name_array: Vec<String> = Vec::new();
    for word in re.find_iter(&name) {
        name_array.push(word.as_str().to_string());
    }
    let mut name_string: String = String::new();
    for word in name_array {
        name_string.push_str(&word);
        name_string.push_str(" ");
    }
    return name_string;
}

fn load_songs() -> Value {
    // Basically gets the song name and artist from the json file and stuff
    let mut data = String::new();
    let mut file = File::open("src/data/playlists.json").unwrap();
    file.read_to_string(&mut data).unwrap();
    let json: Value = serde_json::from_str(&data).unwrap();

    // Get the array of songs
    return json.to_owned();
}


#[derive(Debug)]
struct Song {
    name: String,
    artist: String,
}

// Gets the name from terminal, and validates it with regex and returns a player