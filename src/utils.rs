pub fn auth(count: u8) -> Player {
    let mut line = String::new();

    println!("Enter Player {} name:", count);
    std::io::stdin().read_line(&mut line).unwrap().to_string();

    let player = Player {
        name: line.to_owned().replace("\n", ""),
        score: 0,
    };
    return player;
}


pub struct Player {
    name: String,
    score: i32,
}