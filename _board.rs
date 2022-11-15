use glam::*;
use std::{ops::Index, char};
const WIDTH:i32 = 8;

pub struct Board{pub pieces: [Piece;64]}

type Piece = char;

impl Index<IVec2> for Board {
    type Output = Piece;
    fn index(&self, v : IVec2) -> &Self::Output{
        if (v.abs() != v) || (v.max_element() > WIDTH-1) {
            &'\0'
        } else {
            let i : usize = (v.x + WIDTH* v.y).try_into().unwrap();
            &self.pieces[i]
        }
    }
}

trait PieceMethods {
    fn is_empty(self)-> bool;
    fn is_black(self)-> bool;
    fn is_white(self)-> bool;
    fn is_rival(self, other: Self)-> bool;
    fn is_ally(self, other: Self)-> bool;
    fn get_directions(self) -> Vec<IVec2>;
    fn long_moves(self, pos: IVec2, dir: IVec2, board: &Board) -> Vec<IVec2>;
    fn short_moves(self, pos: IVec2, dir: IVec2, board: &Board) -> Vec<IVec2>;
    fn pawn_moves(self, pos: IVec2, dir: IVec2, board: &Board) -> Vec<IVec2>;
}

impl PieceMethods for Piece{
    fn is_empty(self) -> bool{
        if self == ' ' {
            true
        } else {
            false
        }
    }
    fn is_black(self) -> bool {
        match self {
            'k' | 'q' | 'b' | 'n' | 'r' | 'p' => true,
            _=> false
        }
    }
    fn is_white(self) -> bool {
        match self {
            'K' | 'Q' | 'B' | 'N' | 'R' | 'P' => true,
            _=> false
        }
    }

    fn is_ally(self, other: Self) -> bool{
        if self.is_black() && other.is_black() {true} 
        else if self.is_white() && other.is_white() {true}
        else {false}
    }

    fn is_rival(self, other: Self)-> bool {
        if self.is_white() && other.is_black() {true} 
        else if self.is_black() && other.is_white() {true}
        else {false}
    }

    fn get_directions(self)-> Vec<IVec2> {
        match self{
            'K'|'k'|'Q'|'q' => vec![
                ivec2(-1, -1), ivec2( 0, -1), ivec2( 1, -1), ivec2(-1,  0),
                ivec2( 1,  0), ivec2(-1,  1), ivec2( 0,  1), ivec2( 1,  1)],
            'N'|'n' => vec![
                ivec2(-1, -2), ivec2( 1, -2), ivec2(-2, -1), ivec2( 2, -1),
                ivec2(-2,  1), ivec2( 2,  1), ivec2(-1,  2), ivec2( 1,  2)
            ],
            'R'|'r' => vec![
                ivec2( 0, -1), ivec2(-1,  0), ivec2( 1,  0), ivec2( 0,  1)
            ],
            'B'|'b' => vec![
                ivec2(-1, -1), ivec2( 1, -1), ivec2(-1,  1), ivec2( 1,  1)
            ],
            'P' => vec![ivec2(-1, -1), ivec2( 0, -1), ivec2( 1, -1)],
            'p' => vec![ivec2(-1,  1), ivec2( 0,  1), ivec2( 1,  1)],
            _=> Vec::new()
        }
    }

    fn long_moves(self, pos: IVec2, dir: IVec2, board: &Board) -> Vec<IVec2>{
        let next = pos + dir;
        if board[next].is_empty() {
            let mut moves = self.long_moves(next, dir, board);
            moves.push(next);
            moves
        } else if board[next].is_rival(self) { 
            vec![next]
        } else {
            vec![]
        }
    }

    fn short_moves(self, pos: IVec2, dir: IVec2, board: &Board) -> Vec<IVec2> {
        let next = pos + dir;
        if board[next].is_empty() | board[next].is_rival(self) { 
            vec![next] 
        } else {
            vec![]
        }
    }

    fn pawn_moves(self, pos: IVec2, dir: IVec2, board: &Board) -> Vec<IVec2> {
        match dir.abs().x{
            0 => { self.short_moves(pos, dir, board) },
            1 => { 
                let next = pos + dir;
                    if board[next].is_empty() | board[next].is_rival(self) { 
                    vec![next] 
                } else {
                vec![]
                }
            },
            _ => vec![]
        }
    }
}


impl Board {
    pub fn get_moves(&self, pos:IVec2)-> Vec<IVec2>{
        let piece = self[pos];
        piece
            .get_directions()
            .into_iter()
            .flat_map(|dir| {
                match piece {
                    'K' | 'k' | 'N' | 'n' => piece.short_moves(pos, dir, self),
                    'Q' | 'q' | 'B' | 'b' | 'R' | 'r' => piece.long_moves(pos, dir, self),
                    'P' | 'p' => piece.pawn_moves(pos, dir, self),
                    _ => vec![]
                }
            })
            .collect()
    }
}