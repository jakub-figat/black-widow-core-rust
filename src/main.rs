use game::{Game, GameSettings, GameState};

fn main() {
    let players = vec![
        String::from("player1"),
        String::from("player2"),
        String::from("player3")
    ];
    let game = Game::new(players, GameSettings {max_score: 100});
    match &game.game_state {
        GameState::CardExchange(step) => println!("{:#?}", step.state.cards_to_exchange),
        _ => panic!("invalid step")
    }
}
