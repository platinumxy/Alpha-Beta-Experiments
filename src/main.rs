use std::io::{self, Write};

mod noughts;

type Board = noughts::game::NoughtsNCrosses;

fn main() {
    let mut game = noughts::game::new_game();
    let player_1_is_bot = is_player_bot(1);
    let player_2_is_bot = is_player_bot(2);

    let play = |game: Board, is_bot: bool, n: u8| -> Board {
        let ngame;
        let x;
        let y;
        if is_bot {
            (ngame, x, y) = bot_play(game);
            println!("\nBot {} plays at: {} {}", n, x, y);
        } else {
            (ngame, x, y) = human_play(game);
            println!("\nPlayer {} plays at: {} {}", n, x, y);
        }
        ngame
    };

    while !game.won() && !game.draw() {
        game = play(game, player_1_is_bot, 1);
        noughts::game::pretty_print(game);
        if game.won() || game.draw() {
            break;
        }

        game = play(game, player_2_is_bot, 2);
        noughts::game::pretty_print(game);
    }
    println!("\n\n\n=======GAME OVER=======\n");
    noughts::game::pretty_print(game);
    println!(
        "{}",
        if game.cross_won() {
            "Bot wins!"
        } else if game.won() {
            "You win!"
        } else {
            "It's a draw!"
        }
    );
}

fn readline() -> String {
    let mut l = String::new();
    io::stdin().read_line(&mut l).unwrap();
    l
}

fn is_player_bot(n: u8) -> bool {
    println!("Is player {} a bot? (y/n): ", n);
    io::stdout().flush().unwrap();
    let line = readline();
    match line.trim().to_lowercase().as_str() {
        "y" | "yes" => true,
        "n" | "no" => false,
        _ => is_player_bot(n),
    }
}

fn bot_play(game: Board) -> (Board, u8, u8) {
    let (x, y) = noughts::bot::alpha_beta_prune(game);
    (noughts::game::play(game, x, y), x, y)
}

fn human_play(game: Board) -> (Board, u8, u8) {
    println!("Enter your move as 'x y': ");
    io::stdout().flush().unwrap();

    let line = readline();
    let parts: Vec<&str> = line.trim().split_whitespace().collect();
    if parts.len() != 2 {
        println!("Invalid input. Please enter two numbers separated by a space.");
        return human_play(game);
    }

    let (x, y) = match (parts[0].parse::<u8>(), parts[1].parse::<u8>()) {
        (Ok(x), Ok(y)) if x < 3 && y < 3 => (x, y),
        _ => {
            println!("Invalid input. Please enter numbers between 0 and 2.");
            return human_play(game);
        }
    };

    if game.board[x as usize][y as usize] != noughts::game::Tile::Empty {
        println!("Tile already occupied. Choose another move.");
        return human_play(game);
    }

    (noughts::game::play(game, x, y), x, y)
}
