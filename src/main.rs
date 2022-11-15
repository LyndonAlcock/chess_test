use ggez::{
    event::{self, MouseButton, EventHandler},
    glam::*,
    graphics::{self, Color},
    Context, GameResult, mint::Point2, GameError,
};

struct Board{
    circle: graphics::Mesh,
    pieces: [Piece;3],
    draw_pos : Point2<f32>,
    handle: Point2<f32>
}
struct Piece {
    held : bool,
    pos : Point2<f32>,

}

impl Board {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            vec2(0., 0.),
            50.0,
            2.0,
            Color::WHITE,
        )?;

        let piece1 = Piece{held: false, pos: Point2 { x: 30.0, y: 40.0 }};
        let piece2 = Piece{held: false, pos: Point2 { x: 200.0, y: 200.0 }};
        let piece3 = Piece{held: false, pos: Point2 { x: 400.0, y: 400.0 }};

        Ok( Board{
            circle,
            pieces: [piece1,piece2,piece3],
            draw_pos : Point2 { x: 0.0, y: 0.0 },
            handle: Point2 { x: 0.0, y: 0.0 }
        } )
            
    }
}


impl EventHandler<GameError> for Board {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.draw_pos.x = ctx.mouse.position().x + self.handle.x;
        self.draw_pos.y = ctx.mouse.position().y + self.handle.y;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        for p in &self.pieces{
            if p.held{
                canvas.draw(&self.circle, self.draw_pos);
            } else {
                canvas.draw(&self.circle, p.pos);
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
        
        for piece in self.pieces.iter_mut(){
            let dist = ((x-piece.pos.x).powf(2.) + (y-piece.pos.y).powf(2.)).sqrt();
        
            if (dist < 100.) & (button == MouseButton::Left){
                piece.held = true;
                self.handle = Point2{x: piece.pos.x - x, y: piece.pos.y - y};
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
        
        for piece in self.pieces.iter_mut(){
            if (piece.held == true) & (button == MouseButton::Left){
                piece.held = false;
                piece.pos = Point2{x: self.handle.x + x, y: self.handle.y + y};
            }
        }

        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (mut ctx, event_loop) = cb.build()?;
    let state = Board::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}