#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GamePiece {
    pub color: PieceColor,
    pub crowned: bool,
}

impl GamePiece {
    pub fn new(color: PieceColor) -> GamePiece {
        GamePiece {
            color,
            crowned: false,
        }
    }

    pub fn crowned(p: GamePiece) -> GamePiece {
        GamePiece {
            color: p.color,
            crowned: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinate(pub usize, pub usize);

// 0, 0 ->
// ---------------
// | | | | | | | |
// | | | | | | | |
// | | | | | | | |
// | | | | | | | |
// | | | | | | | |
// | | | | | | | |
// | | | | | | | |
// ---------------
impl Coordinate {
    // Check if coordinate is on the board since it's 7 x 7
    pub fn on_board(self) -> bool {
        let Coordinate(x, y) = self;
        x <= 7 && y <= 7
    }

    // List of potential jump targets
    pub fn jump_targets_from(&self) -> impl Iterator<Item = Coordinate> {
        let mut jumps = vec![];
        let Coordinate(x, y) = *self;

        if y >= 2 {
            jumps.push(Coordinate(x + 2, y - 2));
        }

        // NOTE: if x >= 5, this shouldn't work?
        jumps.push(Coordinate(x + 2, y + 2));

        if x >= 2 && y >= 2 {
            jumps.push(Coordinate(x - 2, y - 2));
        }
        if x >= 2 {
            jumps.push(Coordinate(x - 2, y + 2));
        }

        jumps.into_iter()
    }

    // List of potential move targets (adjacent spaces)
    pub fn move_targets_from(&self) -> impl Iterator<Item = Coordinate> {
        let mut moves = vec![];
        let Coordinate(x, y) = *self;

        if x >= 1 {
            moves.push(Coordinate(x - 1, y + 1));
        }

        // NOTE: shouldn't work if x >= 7
        moves.push(Coordinate(x + 1, y + 1));

        if y >= 1 {
            moves.push(Coordinate(x + 1, y - 1));
        }
        if x >= 1 && y >= 1 {
            moves.push(Coordinate(x - 1, y - 1));
        }

        moves.into_iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Move {
    pub from: Coordinate,
    pub to: Coordinate,
}

impl Move {
    pub fn new(from: (usize, usize), to: (usize, usize)) -> Move {
        let (fx, fy) = from;
        let (tx, ty) = to;

        Move {
            from: Coordinate(fx, fy),
            to: Coordinate(tx, ty),
        }
    }
}
