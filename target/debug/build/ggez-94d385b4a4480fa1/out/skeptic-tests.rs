extern crate skeptic;
#[test] fn readme_sect_what_is_this_line_120() {
    let s = &format!(r####"
{}"####, r####"use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color};
use ggez::event::{self, EventHandler};

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game);
}

struct MyGame {
    // Your state here...
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            // ...
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        // Draw code here...
        canvas.finish(ctx)
    }
}
"####);
    skeptic::rt::compile_test(r#"/Users/johnasencio/.cargo/registry/src/github.com-1ecc6299db9ec823/ggez-0.9.0-rc0"#, r#"/Users/johnasencio/Desktop/endless spire/target/debug/build/ggez-94d385b4a4480fa1/out"#, r#"aarch64-apple-darwin"#, s);
}

