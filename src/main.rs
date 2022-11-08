use glam::*;
use std::{ops::Index, char};

const WIDTH:u32 = 8;
const DIAGONALS : [IVec2; 4] = [
    ivec2(-1, -1), ivec2(-1, 1),
    ivec2(-1, 1), ivec2(1, 1)
];
struct Board{pieces: [char;64]}

type PieceOption = Option<char>;

impl Index<IVec2> for Board {
    type Output = PieceOption;
    fn index(&self, v : IVec2) -> &Self::Output{
        if (v.abs() != v) || (v.max_element() > 8) {&None}
        else {
            let i : usize = (v.x + 8* v.y).try_into().unwrap();
            &Some(self.pieces[i])
        }
    }
}

trait PieceMethods {
    fn is_empty(self)-> bool;
    fn is_black(self)-> bool;
    fn is_white(self)-> bool;
    fn is_rival(self, other: PieceOption)-> bool;
    fn is_ally(self, other: PieceOption)-> bool;
}

impl PieceMethods for PieceOption {
    fn is_empty(self) -> bool{
        if self == Some(' ') {true} 
        else {false}
    }
    fn is_black(self) -> bool {
        match self {
            Some('k')|Some('q')|Some('b')|Some('n')|Some('r')|Some('p') => true,
            _=> false
        }
    }
    fn is_white(self) -> bool {
        match self {
            Some('K')|Some('Q')|Some('B')|Some('N')|Some('R')|Some('P') => true,
            _=> false
        }
    }

    fn is_ally(self, other: PieceOption) -> bool{
        if self.is_black() && other.is_black() {true} 
        else if self.is_white() && other.is_white() {true}
        else {false}
    }

    fn is_rival(self, other: PieceOption)-> bool {
        if self.is_white() && other.is_black() {true} 
        else if self.is_black() && other.is_white() {true}
        else {false}
    }
}


impl Board {
    fn moves(&self, piece:&PieceOption, pos:IVec2, dir: Vec<IVec2>)->Vec<IVec2>{
        dir.into_iter()
            .map(|d| d+pos)
            .filter(|p| (p.abs() == *p) && p.max_element() < 8)
            .flat_map(|p|
                match piece{
                    Some('B')|Some('b')=>{
                        if self[p].is_empty() {self.moves(piece, p, vec![p-pos]).push(p)}
                        else if self[p].is_rival(*piece) {p}
                        else {}
                        },
                    _ => ().into()
                }
            )
            .collect()
    }
}


fn main() {
    let board= Board{pieces:
        [' ',' ',' ',' ',' ',' ',' ',' ',
        ' ',' ',' ',' ',' ',' ',' ',' ',
        ' ',' ',' ',' ',' ',' ',' ',' ',
        ' ',' ',' ',' ','b',' ',' ',' ',
        ' ',' ',' ',' ',' ',' ',' ',' ',
        ' ',' ',' ',' ',' ',' ',' ',' ',
        ' ',' ',' ',' ',' ',' ',' ',' ',
        ' ',' ',' ',' ',' ',' ',' ',' ',]};

    board.moves('b', ivec2(4,3), DIAGONALS);
    
    
}