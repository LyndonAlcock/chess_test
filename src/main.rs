use board::Board;
use ggez::{
    event::{self, MouseButton, EventHandler},
    glam::*,
    graphics::{self, Color},
    Context, GameResult, mint::Point2, GameError,
};
mod piece_methods;
mod board;

const STARTING_FEN : &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
const TILE_SIZE : f32 = 100.0;

struct BoardState{
    circle: graphics::Mesh,
    board: Board,
    held_index: IVec2,
    draw_pos: Vec2,
    handle: Vec2
}
struct Piece {
    held : bool,
    pos : Point2<f32>,

}

impl BoardState {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            vec2(0., 0.),
            50.0,
            2.0,
            Color::WHITE,
        )?;

        Ok( BoardState{
            circle,
            board: Board::parse_fen(STARTING_FEN),
            held_index: ivec2(-1, -1),
            draw_pos : vec2(0.0, 0.0),
            handle: vec2(0.0, 0.0)
        } )
            
    }
}


impl EventHandler<GameError> for BoardState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.draw_pos.x = ctx.mouse.position().x + self.handle.x;
        self.draw_pos.y = ctx.mouse.position().y + self.handle.y;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        for (i, p) in self.board.pieces.iter().enumerate(){
            let piece_index = ivec2((i%8) as i32, (i/8) as i32);
            let piece_pos = piece_index.as_vec2() * TILE_SIZE;

            if piece_index==self.held_index && !p.is_empty(){
                canvas.draw(&self.circle, self.draw_pos);
            } else {
                canvas.draw(&self.circle, piece_pos);
            }
        }

        canvas.finish(ctx)?;
        Ok(())
    }
    

    fn mouse_button_down_event(
            &mut self,
            _ctx: &mut Context,
            button: MouseButton,
            x: f32,
            y: f32,
        ) -> GameResult {
        
        for (i, p) in self.board.pieces.iter_mut().enumerate(){
            let piece_index = ivec2((i%8) as i32, (i/8) as i32);
            let piece_pos = piece_index.as_vec2() * TILE_SIZE;
            let dist = ((x-piece_pos.x).powf(2.) + (y-piece_pos.y).powf(2.)).sqrt();
        
            if (dist < 100.) & (button == MouseButton::Left){
                self.held_index = piece_index;
                self.handle = vec2(piece_pos.x - x, piece_pos.y - y);
            }
            
        }
        Ok(())
    }

    fn mouse_button_up_event(
            &mut self,
            _ctx: &mut Context,
            button: MouseButton,
            x: f32,
            y: f32,
        ) -> Result<(), GameError> {
        
        let drop_index = (vec2(x, y)/TILE_SIZE).floor().as_ivec2();
        if button == MouseButton::Left{   
            self.board.move_piece(self.held_index, drop_index);
        }
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (mut ctx, event_loop) = cb.build()?;
    let state = BoardState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}