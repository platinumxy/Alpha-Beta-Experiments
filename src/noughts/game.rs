#[derive(Debug, Copy, Clone)]
pub struct NoughtsNCrosses {
    pub board: [[Tile; 3]; 3],
    pub cross_move: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Nought,
    Cross,
}

impl NoughtsNCrosses {
    pub fn won(self) -> bool {
        let b = &self.board;
        let line_matches = (0..3).into_iter().any(|i| {
            (b[i][0] != Tile::Empty && b[i][0] == b[i][1] && b[i][1] == b[i][2])
                || (b[0][i] != Tile::Empty && b[0][i] == b[1][i] && b[1][i] == b[2][i])
        });
        let diag = b[1][1] != Tile::Empty
            && (b[0][0] == b[1][1] && b[1][1] == b[2][2]
                || b[2][0] == b[1][1] && b[1][1] == b[0][2]);

        diag || line_matches
    }
    pub fn cross_won(self) -> bool {
        self.won() && !self.cross_move
    }

    pub fn draw(self) -> bool {
        self.board
            .iter()
            .all(|l| l.iter().all(|t| *t != Tile::Empty))
    }
}

pub fn new_game() -> NoughtsNCrosses {
    use Tile::*;
    NoughtsNCrosses {
        board: [
            [Empty, Empty, Empty],
            [Empty, Empty, Empty],
            [Empty, Empty, Empty],
        ],
        cross_move: true,
    }
}

pub fn play(mut game: NoughtsNCrosses, x: u8, y: u8) -> NoughtsNCrosses {
    assert!(x < 3 && y < 3);
    assert!(game.board[x as usize][y as usize] == Tile::Empty);
    assert!(!game.won());
    game.board[x as usize][y as usize] = if game.cross_move {
        Tile::Cross
    } else {
        Tile::Nought
    };
    game.cross_move = !game.cross_move;

    game
}

pub fn nice_tile(tile: &Tile) -> char {
    match tile {
        Tile::Empty => ' ',
        Tile::Nought => 'O',
        Tile::Cross => 'X',
    }
}

pub fn pretty_print(game: NoughtsNCrosses) {
    for i in 0..3 {
        let l = game.board[i];
        println!(
            "{}|{}|{}",
            nice_tile(&l[0]),
            nice_tile(&l[1]),
            nice_tile(&l[2]),
        );
        if i != 2 {
            println!("-----")
        }
    }
}
