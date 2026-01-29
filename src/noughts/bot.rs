use crate::noughts::game::{NoughtsNCrosses, Tile, play};

pub struct MoveTree {
    game: NoughtsNCrosses,
    last_move: (u8, u8),
    score: Option<u8>,
    moves: Option<Vec<MoveTree>>,
}

pub fn get_moves(game: NoughtsNCrosses) -> Vec<(u8, u8)> {
    let mut moves = Vec::new();
    if game.won() {
        return moves;
    }
    for x in 0..3 {
        for y in 0..3 {
            if game.board[x as usize][y as usize] == Tile::Empty {
                moves.push((x, y));
            }
        }
    }
    moves
}

pub fn alpha_beta_prune(game: NoughtsNCrosses) -> (u8, u8) {
    let mut current = MoveTree {
        game,
        last_move: (255, 255),
        score: None,
        moves: None,
    };
    prune(&mut current, 0, 2);

    // use invalid states to force it to chose a real move even if non are perfect
    let mut best_move = (255u8, 255u8);
    let mut best_value: i32 = if game.cross_move { -1 } else { 255 };

    for mv in current.moves.unwrap().iter() {
        if let Some(score) = mv.score {
            let score = score as i32;
            if (game.cross_move && score > best_value) || (!game.cross_move && score < best_value) {
                best_value = score as i32;
                best_move = mv.last_move;
            }
        }
    }

    best_move
}
fn prune(parent: &mut MoveTree, mut alpha: u8, mut beta: u8) -> u8 {
    if parent.game.won() || parent.game.draw() {
        parent.moves = Some(Vec::new());
        let score = if parent.game.cross_won() {
            2 // Cross win
        } else if parent.game.won() {
            0 // Nought win
        } else {
            1 // Draw
        };
        parent.score = Some(score);
        return score;
    }

    let mov = get_moves(parent.game)
        .iter()
        .map(|(x, y)| MoveTree {
            game: play(parent.game, *x, *y),
            last_move: (*x, *y),
            score: None,
            moves: None,
        })
        .collect();
    parent.moves = Some(mov);

    let best_value = if parent.game.cross_move {
        // cross -> maxi
        let mut best_value = 0;
        for child in parent.moves.as_mut().unwrap().iter_mut() {
            let value = prune(child, alpha, beta);
            if value > best_value {
                best_value = value;
            }
            if best_value > alpha {
                alpha = best_value;
            }
            if beta <= alpha {
                break;
            }
        }
        best_value
    } else {
        // nought -> mini
        let mut best_value = 2;
        for child in parent.moves.as_mut().unwrap().iter_mut() {
            let value = prune(child, alpha, beta);
            if value < best_value {
                best_value = value;
            }
            if best_value < beta {
                beta = best_value;
            }
            if beta <= alpha {
                break;
            }
        }
        best_value
    };

    parent.score = Some(best_value);
    best_value
}
