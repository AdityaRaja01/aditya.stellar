#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, log};

#[contracttype]
#[derive(Clone)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

#[contracttype]
#[derive(Clone)]
pub struct Game {
    pub player1: Address,
    pub player2: Address,
    pub move1: Option<Move>,
    pub move2: Option<Move>,
    pub winner: Option<Address>,
}

#[contract]
pub struct RockPaperScissorsContract;

#[contractimpl]
impl RockPaperScissorsContract {
    pub fn create_game(env: Env, id: u64, player1: Address, player2: Address) {
        let game = Game {
            player1,
            player2,
            move1: None,
            move2: None,
            winner: None,
        };
        env.storage().instance().set(&id, &game);
        log!(&env, "Game {} created!", id);
    }

pub fn play_move(env: Env, id: u64, player: Address, player_move: Move) {
    let mut game: Game = env.storage().instance().get(&id).unwrap();  // Explicit type annotation

    if player == game.player1 {
        game.move1 = Some(player_move);
    } else if player == game.player2 {
        game.move2 = Some(player_move);
    } else {
        panic!("Invalid player!");
    }

    if game.move1.is_some() && game.move2.is_some() {
        game.winner = determine_winner(&game);
        log!(&env, "Game {} winner: {:?}", id, game.winner);
    }

    env.storage().instance().set(&id, &game);
}


    pub fn get_game(env: Env, id: u64) -> Game {
        env.storage().instance().get(&id).unwrap()
    }
}

fn determine_winner(game: &Game) -> Option<Address> {
    let move1 = game.move1.clone().unwrap();
    let move2 = game.move2.clone().unwrap();

    use Move::*;
    match (move1, move2) {
        (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => Some(game.player1.clone()),
        (Scissors, Rock) | (Paper, Scissors) | (Rock, Paper) => Some(game.player2.clone()),
        _ => None, // Tie
    }
}
