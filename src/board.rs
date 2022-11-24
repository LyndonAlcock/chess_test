use ggez::glam::*;
use std::{ops::{Index}};

use crate::piece::Piece;
use crate::piece_methods::PieceMethods;

const WIDTH:i32 = 8;
#[derive(Copy,Clone)]
pub struct Board{pub pieces: [Piece;64]}


impl Index<IVec2> for Board {
    type Output = Piece;
    fn index(&self, index : IVec2) -> &Self::Output{
        if (index.abs() != index) || (index.max_element() > WIDTH-1) {
            &Piece('\0')
        } else {
            let i : usize = (index.x + WIDTH* index.y).try_into().unwrap();
            &self.pieces[i]
        }
    }
}

impl Board { 
    fn _get(&self, index: IVec2) -> Option<&Piece> {
        if (index.abs() != index) || (index.max_element() > WIDTH-1) {
            None
        } else {
            let i: usize = (index.x + WIDTH* index.y).try_into().ok()?;
            Some(&self.pieces[i])
        }
    }
    fn get_mut(&mut self, index: IVec2) -> Option<&mut Piece> {
        if (index.abs() != index) || (index.max_element() > WIDTH-1) {
            None
        } else {
            let i: usize = (index.x + WIDTH * index.y).try_into().ok()?;
            Some(&mut self.pieces[i])
        }
    }

    pub fn get_moves(self, pos:IVec2)-> Vec<IVec2>{
        let piece = self[pos];
        piece
            .get_directions()
            .into_iter()
            .flat_map(|dir| {
                match *piece {
                    'K' | 'k' | 'N' | 'n' => piece.short_move(pos, dir, &self),
                    'Q' | 'q' | 'B' | 'b' | 'R' | 'r' => piece.long_move(pos, dir, &self),
                    'P' | 'p' => piece.pawn_move(pos, dir, &self),
                    _ => vec![]
                }
            })
            .collect()
    }

    pub fn move_piece(&mut self, from: IVec2, to: IVec2){
        if !self.get_moves(from).contains(&to){
            return;
        }
        let dir = from - to;
        println!("{}",dir);

        *self.get_mut(to).unwrap() = self[from];

        if self[from].is_pawn() & (dir.abs().y == 2) {
            let ep_loc = from - dir/2;
            println!("x:{} y:{}",ep_loc.x, ep_loc.y);
            **self.get_mut(ep_loc).unwrap() = '-';
            println!("{}",*self[ep_loc])
        }

        **self.get_mut(from).unwrap() = ' ';
    }

    pub fn parse_fen(s: &str)-> Board{
        let mut ch_v = vec![];
        s.replace("/","")
            .chars()
            .for_each(|c| 
                if c.is_numeric(){
                    for _ in 0..c.to_digit(10).unwrap()
                        {ch_v.push(Piece(' '));}}
                else {ch_v.push(Piece(c));});
        Board {pieces: ch_v.try_into().unwrap() }
    }
}