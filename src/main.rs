use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
extern crate rand;
mod game;
struct MainState {
    gm: game::Game,
    width: u32,
    height: u32,
    //entity pixel dimensions
    e_width: f32,
    e_height: f32
}

impl MainState {
    fn new(width: f32, height: f32) -> ggez::GameResult<MainState> {
        let mut s = MainState{
            gm: game::Game::new(
                100, //map width
                100, //map height
                60, //plant energy
                60, //energy needed to reproduce
                1, //plants per day
                1, //plants in the forest per day
                0.1), //the size of the forest as percentage of the whole map
            width: 100, 
            height: 100, 
            e_width: width/100.0, 
            e_height: height/100.0};
        println!("Size: {}x{}", s.gm.get_width(), s.gm.get_height());
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        self.gm.new_day();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [1.0, 1.0, 1.0, 1.0].into());
        for y in 0..(self.height as usize) {
            for x in 0..(self.width as usize) {
                //Go through every char in the map-array and draw rectangles if they are plants or animals
                let e: char = self.gm.get_single(x, y);

                //empty spaces get skipped as we cleared the screen white already
                if e == ' ' {
                    continue;
                }
                let entity: graphics::Mesh;
                let col: graphics::Color;

                //Just choose a color based on the type of entity
                if e == 'P' {
                    col = graphics::Color::new(0.0,1.0,0.0,1.0);
                }else {
                    col = graphics::Color::new(1.0,0.0,0.0,1.0);
                }
                entity = graphics::Mesh::new_rectangle(
                    ctx, 
                    graphics::DrawMode::fill(), 
                    graphics::Rect::new(
                        (x as f32)*self.e_width,
                        (y as f32)*self.e_height,
                        self.e_width,
                        self.e_height
                    ),
                    col)?;
                graphics::draw(ctx, &entity, (na::Point2::new(0.0, 0.0),))?;
            }
        }
        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> ggez::GameResult {
    println!("Welcome to 'Darwin Simulation'!");
    let mut cb = ggez::ContextBuilder::new("darwin", "overflwn");
    let mut conf = ggez::conf::Conf::new();
    conf.window_setup.title = String::from("Darwin Simulation");
    conf.window_mode.width = 800.0;
    conf.window_mode.height = 600.0;
    conf.window_setup.vsync = true;
    cb = cb.conf(conf);
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(800.0, 600.0)?;

    event::run(ctx, event_loop, state)
}
