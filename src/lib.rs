#![feature(const_generics)]

use bitboards::Bitboard;

/// A bitboard initialized from a board size
pub type BitboardFromBoardSize<const S: usize> = Bitboard<{(S+2)*(S+2)}>;
/// A predefined type for 9x9 board
pub type Board9x9 = Board<9>;

/// A player type
pub type Player = bool;
/// Black player constant
pub const BLACK: Player = false;
/// White player constant
pub const WHITE: Player = true;

pub type Intersection = usize;

/// The main structure representing a board
pub struct Board<const S: usize> {
    /// The square size of the goban
    size: usize,

    // Bitboards for stone occupation
    /// stone occupation, indexed by player
    pub occupied_intersections: [BitboardFromBoardSize<S>; 2],
    /// empty intersections of the board
    pub empty_intersections: BitboardFromBoardSize<S>,
    /// sentinel intersections outside the board
    pub out_of_bound_intersections: BitboardFromBoardSize<S>,

    /// Player to move
    pub to_move: Player,
    /// Ko intersection if there is one
    pub ko: Option<Intersection>,
}

impl<const S: usize> Board<S> {
    pub fn new() -> Self {
        // Remember that the bitboards are of S+2 lines
        // We generate the empty intersections first so the junk of the bitboard is
        // set to out of bound
        let mut empty_intersections = Bitboard::new();
        for line in 1..S+1 {
            empty_intersections.set_whole_line(line, S+2);
            empty_intersections.unset(line*(S+2));
            empty_intersections.unset(line*(S+2) + S + 1);
        }

        let mut out_of_bound_intersections = empty_intersections.clone();
        out_of_bound_intersections.flip();
        Board {
            size: S,
            occupied_intersections: [BitboardFromBoardSize::<S>::new(), BitboardFromBoardSize::<S>::new()],
            empty_intersections,
            out_of_bound_intersections,

            to_move: BLACK,
            ko: None,
        }
    }
}
