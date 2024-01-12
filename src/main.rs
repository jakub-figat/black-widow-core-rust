use game::game::{Game, GameSettings};

fn main() {
    let players = vec![
        String::from("player1"),
        String::from("player2"),
        String::from("player3")
    ];
    let game = Game::new(&players, GameSettings {max_score: 100});
}
