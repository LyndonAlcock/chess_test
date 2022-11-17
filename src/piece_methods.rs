use std::ops::{Deref, Index};
use ggez::glam::*;

pub trait PieceMethods <B> where 
    Self: Copy + Sized + Deref<Target = char>,
    B: Index<IVec2, Output = Self>{
    fn is_empty(self)-> bool {
        if *self == ' ' {
            true
        } else {
            false
        }
    }
    fn is_black(self)-> bool {
        match *self {
            'k' | 'q' | 'b' | 'n' | 'r' | 'p' => true,
            _=> false
        }
    }
    fn is_white(self)-> bool {
        match *self {
            'K' | 'Q' | 'B' | 'N' | 'R' | 'P' => true,
            _=> false
        }
    }
    fn is_rival(self, other: Self)-> bool {
        if self.is_black() && other.is_white() {true} 
        else if self.is_white() && other.is_black() {true}
        else {false}
    }
    fn is_ally(&self, other: Self)-> bool {
        if self.is_black() && other.is_black() {true} 
        else if self.is_white() && other.is_white() {true}
        else {false}
    }
    fn get_directions(self) -> Vec<IVec2> {
        match *self{
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
    fn long_move(self, pos: IVec2, dir: IVec2, board: &B) -> Vec<IVec2> {
        let next = pos + dir;
        if board[next].is_empty() {
            let mut moves = self.long_move(next, dir, board);
            moves.push(next);
            moves
        } else if board[next].is_rival(self) { 
            vec![next]
        } else {
            vec![]
        }
    }
    fn short_move(self, pos: IVec2, dir: IVec2, board: &B) -> Vec<IVec2> {
        let next = pos + dir;
        if board[next].is_empty() | board[next].is_rival(self) { 
            vec![next] 
        } else {
            vec![]
        }
    }
    fn pawn_move(self, pos: IVec2, dir: IVec2, board: &B) -> Vec<IVec2> {
        match dir.abs().x{
            0 => { self.short_move(pos, dir, board) },
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