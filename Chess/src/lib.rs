use std::fmt;
use skip_error;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver
}

pub struct Game {
    /* save board, active colour, ... */
    state: GameState,
    board: Board,
    active: Side,

}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        Game {
            board: Board::new(),/* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            active: Side::White,
            //...
        }
    }

    /// If the current game state is `InProgress` and the move is legal, 
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: &str, _to: &str) -> Option<GameState> {
        None
    }

    /// Set the piece type that a pawn becames following a promotion.
    pub fn set_promotion(&mut self, _piece: &str) -> () {
        ()
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }
    
    /// If a piece is standing on the given tile, return all possible 
    /// new positions of that piece. Don't forget to the rules for check. 
    /// 
    /// (optional) Don't forget to include en passent and castling.
    pub fn get_possible_moves(&self, _postion: &str) -> Option<Vec<String>> {
        None
    }
}

/// Implement print routine for Game.
/// 
/// Output example:
/// |:----------------------:|
/// | R  Kn B  K  Q  B  Kn R |
/// | P  P  P  P  P  P  P  P |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | P  P  P  P  P  P  P  P |
/// | R  Kn B  K  Q  B  Kn R |
/// |:----------------------:|
impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */
        
        write!(f, "")
    }
}
/// Represents the two different teams in a game of chess.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Side {
    White,
    Black,
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Side::White => write!(f, "w"),
            Side::Black => write!(f, "b"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl Piece {
    pub fn get_possible_moves(board: Board, _position: &str) -> Vec<String> {
        
        let (f, r) = _position.split_at(1);
        let p = board.squares[Board::file_index(f)][Board::rank_index(r)].piece.unwrap();

        match p {
            Piece::King => return Piece::legal_moves(p, board,_position),
            Piece::Queen => return Piece::legal_moves(p, board, _position),
            Piece::Bishop => return Piece::legal_moves(p, board, _position),
            Piece::Knight => return Piece::legal_moves(p, board, _position),
            Piece::Rook => return Piece::legal_moves(p, board, _position),
            Piece::Pawn => return Piece::legal_moves(p, board, _position),
            _ => unreachable!()
        }
    }

    
    pub fn legal_moves(self, b: Board, _position: &str) -> Vec<String> {

        let (f, r) = _position.split_at(1);

            match self {
                Piece::Pawn => return Piece::pawn_valid(b, f, r),
                Piece::Rook => return Piece::pawn_valid(b, f, r),
                Piece::Bishop => return Piece::pawn_valid(b, f, r),
                Piece::Knight => return Piece::knight_valid(b, f, r),
                Piece::Queen => return Piece::pawn_valid(b, f, r),
                Piece::King => return Piece::pawn_valid(b, f, r),
                _ => unreachable!()
            }
    }

    fn pawn_valid(board: Board, f: &str, r: &str) -> Vec<String> {
        let mut legals: Vec<String> = vec![];
        if board.squares[Board::file_index(f)][Board::rank_index(r)].side == Some(Side::White) {
        
            if board.squares[Board::file_index(f)][Board::rank_index(r)-1].side != Some(Side::White) || board.squares[Board::file_index(f)][Board::rank_index(r)-1].side != Some(Side::Black) {
                let mut rank = RANKS[Board::rank_index(r)-1].to_string();
                let file = f.to_string();
                legals.push(file+&rank);
            }

            
            if Board::rank_index(r) == 6 && board.squares[Board::file_index(f)][Board::rank_index(r)-2].side != Some(Side::White) || Board::rank_index(r) == 6 && board.squares[Board::file_index(f)][Board::rank_index(r)-2].side != Some(Side::Black) {
               let rank = RANKS[Board::rank_index(r)-2].to_string();
               let file = f.to_string();
               legals.push(file+&rank);
               
            }

            if board.squares[Board::file_index(f)+1][Board::rank_index(r)-1].side == Some(Side::Black) {
                let rank= RANKS[Board::rank_index(r)-1].to_string();
                let file = FILES[Board::file_index(f)+1].to_string();
                legals.push(file+&rank);
            }
        }
        if board.squares[Board::file_index(f)-1][Board::rank_index(r)-1].side == Some(Side::Black) {
        
            if board.squares[Board::file_index(f)][Board::rank_index(r)+1].side != Some(Side::White) || board.squares[Board::file_index(f)][Board::rank_index(r)+1].side != Some(Side::Black) {
                let rank = RANKS[Board::rank_index(r)+1].to_string();
                let file = f.to_string();
                legals.push(file+&rank);
            }

            if Board::rank_index(r) == 1 && board.squares[Board::file_index(f)][Board::rank_index(r)].side != Some(Side::White) || Board::rank_index(r) == 1 && board.squares[Board::file_index(f)][Board::rank_index(r)].side != Some(Side::Black) {
               let rank = RANKS[Board::rank_index(r)+2].to_string();
               let file = f.to_string();
               legals.push(file+&rank);
            }

            if board.squares[Board::file_index(f)+1][Board::rank_index(r)+1].side == Some(Side::White) {
                let rank = RANKS[Board::rank_index(r)+1].to_string();
                let file = FILES[Board::file_index(f)+1].to_string();
                legals.push(file+&rank);
            }
            if board.squares[Board::file_index(f)-1][Board::rank_index(r)+1].side == Some(Side::White) {
                let rank = RANKS[Board::rank_index(r)+1].to_string();
                let file = FILES[Board::file_index(f)-1].to_string();
                legals.push(file+&rank);
            }
        }
    return legals;

    }


    fn knight_valid(board: Board, f: &str, r: &str) -> Vec<String> {
        let mut legals: Vec<String> = vec![];
        if board.squares[Board::file_index(f)][Board::rank_index(r)].side == Some(Side::White) {

            if board.squares[Board::file_index(f)+2][Board::rank_index(r)-1].side != Some(Side::White) {
                let rank = RANKS[Board::rank_index(r)-1].to_string();
                let file = FILES[Board::file_index(f)+2].to_string();
                legals.push(file+&rank);
            }

            if board.squares[Board::file_index(f)-2][Board::rank_index(r)-1].side != Some(Side::White) {
                let rank = RANKS[Board::rank_index(r)-1].to_string();
                let file = FILES[Board::file_index(f)-2].to_string();
                legals.push(file+&rank);
            }
            if board.squares[Board::file_index(f)+1][Board::rank_index(r)-2].side != Some(Side::White) {
                let rank = RANKS[Board::rank_index(r)-2].to_string();
                let file = FILES[Board::file_index(f)+1].to_string();
                legals.push(file+&rank);
            }
            if board.squares[Board::file_index(f)-1][Board::rank_index(r)-2].side != Some(Side::White) {
                let rank = RANKS[Board::rank_index(r)-2].to_string();
                let file = FILES[Board::file_index(f)-1].to_string();
                legals.push(file+&rank);
            }
        }
        if board.squares[Board::file_index(f)][Board::rank_index(r)].side == Some(Side::Black) {
        
            if board.squares[Board::file_index(f)+2][Board::rank_index(r)+1].side != Some(Side::White) {
                let rank = RANKS[Board::rank_index(r)-1].to_string();
                let file = FILES[Board::file_index(f)+2].to_string();
                legals.push(file+&rank);
            }
            if board.squares[Board::file_index(f)-2][Board::rank_index(r)+1].side != Some(Side::White) {
                let rank = RANKS[Board::rank_index(r)-1].to_string();
                let file = FILES[Board::file_index(f)+2].to_string();
                legals.push(file+&rank);
            }
            if board.squares[Board::file_index(f)+1][Board::rank_index(r)+2].side != Some(Side::White) {
                let rank = RANKS[Board::rank_index(r)+1].to_string();
                let file = FILES[Board::file_index(f)+2].to_string();
                legals.push(file+&rank);
            }
            if board.squares[Board::file_index(f)-1][Board::rank_index(r)+2].side != Some(Side::White) {
                let rank = RANKS[Board::rank_index(r)-1].to_string();
                let file = FILES[Board::file_index(f)+2].to_string();
                legals.push(file+&rank);
            }
        }
        return legals;
    }
}

    


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square {
    piece: Option<Piece>,
    side: Option<Side>,
}

impl Square {
    pub fn new() -> Square {
        Square {piece: None, side: None}
    }

    pub fn symbol(&self) -> &str {
        match self.side {
            Some(Side::White) =>
                match self.piece {
                    Some(Piece::King)   => "♚ ",
                    Some(Piece::Queen)  => "♛ ",
                    Some(Piece::Rook)   => "♜ ",
                    Some(Piece::Bishop) => "♝ ",
                    Some(Piece::Knight) => "♞ ",
                    Some(Piece::Pawn)   => "♟ ",
                    None => ". "
            }

            Some(Side::Black) =>
                match self.piece {
                Some(Piece::King)   => "♔ ",
                Some(Piece::Queen)  => "♕ ",
                Some(Piece::Rook)   => "♖ ",
                Some(Piece::Bishop) => "♗ ",
                Some(Piece::Knight) => "♘ ",
                Some(Piece::Pawn)   => "♙ ",
                None                => ". "
                }
            None => ". "
        }     
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board {
    squares: [[Square; 8]; 8]
}

impl Board {
 
    pub fn empty() -> Board {
        Board {
            squares: [[Square { piece: None, side: None }; 8]; 8]
        }
    }
    pub fn new() -> Board {
        let mut squares = [[Square { piece: None, side: None }; 8]; 8];
    
        for x in 0..8 {
            squares[x][6].piece = Some(Piece::Pawn);
            squares[x][6].side = Some(Side::White);
            squares[x][1].piece = Some(Piece::Pawn);
            squares[x][1].side = Some(Side::Black);

            let piece = match x {
                0 | 7 => Some(Piece::Rook),
                1 | 6 => Some(Piece::Knight),
                2 | 5 => Some(Piece::Bishop),
                3 => Some(Piece::King),
                4 => Some(Piece::Queen),
                _ => unreachable!()
            };
            squares[x][7].piece = piece;
            squares[x][7].side = Some(Side::White);
            squares[x][0].piece = piece;
            squares[x][0].side = Some(Side::Black);
        }
    
        Board {
            squares: squares
        }
    }

    pub fn get_board(&mut self) -> Board {
        Board { squares: self.squares }
    }
    pub fn file_index(file: &str) -> usize {
        return FILES.iter().position(|&rs| rs == file).unwrap();
    }

    pub fn rank_index(rank: &str) -> usize {
        return RANKS.iter().position(|&rs| rs == rank.parse::<usize>().unwrap()).unwrap();
    }
    
}
    

    


impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = String::new();
        let mut rank = [" 8", " 7", " 6", " 5", " 4", " 3", " 2", " 1"];
        let mut file = " A B C D E F G H";
        for i in 0..8 {
            for j in 0..8 {
                res.push_str(self.squares[j][i].symbol());
            }
            res.push_str(rank[i]);
            res.push_str("\n");

            
        }
        res.push_str(file);
        write!(f, "{}", res)
    }
}

const FILES: &'static [&'static str] = &["a", "b", "c", "d", "e", "f", "g", "h"];
const RANKS: [usize; 8] = [8,7,6,5,4,3,2,1];

pub const ALL_PIECES:[Piece; 6] = [Piece::King , Piece::Queen , Piece::Bishop , Piece::Knight, Piece::Rook, Piece::Pawn];

#[cfg(test)]
mod tests {
    use super::Game;
    use super::GameState;

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {

        let game = Game::new();

        println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }


    #[test]
    fn board_test() {
        use super::*;
        assert_eq!(Board::file_index("a"), 0);
        assert_eq!(Board::rank_index("8"), 0);
        assert_eq!(Board::new().squares[0][0].piece , Some(Piece::Rook));
        assert_eq!(Board::new().squares[0][0].side , Some(Side::Black))
        }
}

    