mod implementations;
mod misc {
    pub mod mathfuncs;
}

use std::{path, env};
use ggez::{ContextBuilder, 
    event::{EventHandler, MouseButton},
    GameError, Context, GameResult, 
    graphics::{self, Color, DrawParam, Text, PxScale, Font}, 
    input, audio::SoundSource
};

use glam::Vec2;
use implementations::MainState;
use crate::misc::mathfuncs::{get_distance, vec_from_angle};

impl EventHandler<GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const FPS: u32 = 60;
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        self.hit_hole = self.check_collision_with_hole();

        // Golf ball movement
        while ggez::timer::check_update_time(ctx, FPS) {
            if self.player.velocity > 0.0 && !self.hit_hole {
                self.check_collision_with_border(ctx);
                self.check_collision_with_obstacles(ctx); 

                let v = self.player.velocity * self.player.anglevec;
                self.player.position += v * dt;
                self.player.velocity -= get_distance(self.player.mouse_pos_when_shot, self.player.position, 5.0) * dt;    
            } else if self.hit_hole {
                self.player.velocity = 0.0;
                self.player.position = self.hole.position;
    
                if self.player.shakevec == Vec2::ZERO {
                    self.assets.hole_sound.play(ctx)?;
                    self.player.shakevec = Vec2::new(2.0, 2.0);
                }
    
                if !(self.player.size <= 0.2) {
                    self.player.size -= 0.02;
                    self.player.shakevec.x = -self.player.shakevec.x;
                    self.player.shakevec.y = -self.player.shakevec.y;
                } else {
                    self.player.shakevec = Vec2::ZERO;
                    if self.level == 6 {
                        self.completed = true;
                    } else {
                        self.level += 1;
                        self.get_level_obstacles(ctx);
                    }
                    
                }
            }
    
            // Input 
            if self.player.velocity <= 0.0 && !self.hit_hole {
                if input::mouse::button_pressed(ctx, MouseButton::Left) {
                    self.holding_lmb = true;
                    self.mousevel_buildup = get_distance(input::mouse::position(ctx).into(), self.player.position,1.2);
                    self.change_player_rotation(ctx);
                } else { 
                    self.holding_lmb = false;
                    self.mousevel_buildup =  0.0
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::GREEN);
    
        // Background
        let dp = DrawParam::new();
        graphics::draw(ctx, &self.assets.bg, dp)?;
        
        // Load Obstacles
        if !self.completed {
            for (i,v) in self.obstacle_cords.iter().enumerate() {
                let dp = DrawParam::new().dest(v.to_owned()).offset(Vec2::new(0.5, 0.5));
                if self.obstacle_size[i] == 32 {
                    graphics::draw(ctx, &self.assets.small_obstacle, dp)?;
                } else {
                    graphics::draw(ctx, &self.assets.obstacle, dp)?;
                }
            }
        }
        
        // Draw Hole
        let dp = DrawParam::new().dest(self.hole.position).offset(Vec2::new(0.5, 0.5));
        graphics::draw(ctx, &self.assets.hole, dp)?;

        // Draw Player (and arrow if playing is holding lmb)
        let dp = DrawParam::new().dest(Vec2::new(self.player.position.x - self.player.shakevec.x, self.player.position.y - self.player.shakevec.y)).rotation(self.player.rotation).offset(Vec2::new(0.5, 0.5)).scale(Vec2::new(self.player.size, self.player.size));
        graphics::draw(ctx, &self.assets.player_img, dp)?;

        if self.holding_lmb {
            dp.dest(Vec2::new(self.player.position.x, self.player.position.y - 5.0));       
            graphics::draw(ctx, &self.assets.arrow, dp)?;
        }

        // Draw velocity bar
        if self.holding_lmb {
            let dp = DrawParam::new().dest(Vec2::new(self.player.position.x + 40.0, self.player.position.y)).offset(Vec2::new(0.5, 0.5)).scale(Vec2::new(1.0, (self.mousevel_buildup / 32.0).clamp(0.0, 1.0)));
            graphics::draw(ctx, &self.assets.velobar, dp)?;
        } else if self.player.velocity > 0.0 {
            let dp = DrawParam::new().dest(Vec2::new(self.player.position.x + 40.0, self.player.position.y)).offset(Vec2::new(0.5, 0.5)).scale(Vec2::new(1.0, (self.player.velocity / get_distance(self.player.mouse_pos_when_shot, self.player.pos_before_shot, 1.5)).clamp(0.0, 1.0)));
            graphics::draw(ctx, &self.assets.velobar2, dp)?;
        }

        // GUIs
        let mut text_stroke = Text::new(format!("Strokes: {}", self.strokes.to_string()));
        text_stroke.set_font(Font::new(ctx, "/font.ttf")?, PxScale::from(24.0));

        if self.completed {
            let (w, h) = graphics::drawable_size(ctx);
            text_stroke.set_font(Font::new(ctx, "/font.ttf")?, PxScale::from(40.0));

            let guiparams_strokes = DrawParam::new().dest(Vec2::new(w*0.5-100.0,h*0.5));
            graphics::draw(ctx, &text_stroke, guiparams_strokes)?;

            let complete = DrawParam::new().dest(Vec2::new(w*0.5-200.0,h*0.5-100.0));
            let mut com_text = Text::new("You completed the game!");
            com_text.set_font(Font::new(ctx, "/font.ttf")?, PxScale::from(40.0));
            graphics::draw(ctx, &com_text, complete)?;
        } else {
            let guiparams_strokes = DrawParam::new().dest(Vec2::new(0.0,25.0));

            let guiparams = DrawParam::new().dest(Vec2::new(0.0,0.0));   
            let mut text_level = Text::new(format!("Level: {}", self.level.to_string()));
            text_level.set_font(Font::new(ctx, "/font.ttf")?, PxScale::from(24.0));
            graphics::draw(ctx, &text_stroke, guiparams_strokes)?;
            graphics::draw(ctx, &text_level, guiparams)?;
        }
        
        graphics::present(ctx)?;
        Ok(())
    }
    
    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: MouseButton, _x: f32, _y: f32) {
        match button {
            MouseButton::Left => {
                if self.mousevel_buildup > 0.0 && self.player.velocity <= 0.0 && !self.hit_hole {
                    self.assets.shoot_sound.play(ctx).unwrap();
                    self.player.velocity = self.mousevel_buildup;
                    self.holding_lmb = false;
                    
                    let anglevec = vec_from_angle(-self.player.rotation);
                    self.player.anglevec = anglevec;
                    self.player.mouse_pos_when_shot = input::mouse::position(ctx).into();
                    self.player.pos_before_shot = self.player.position;
                    self.strokes += 1;
                }   
            }
            _ => {}
        }
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {
        if self.mousevel_buildup <= 0.0 {
            self.assets.charge_sound.play(ctx).unwrap();
        }
    }
}

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (mut ctx, event_loop) = ContextBuilder::new("Golf", "RefinedDev")
    .window_setup(ggez::conf::WindowSetup::default().title("Golf").vsync(true))
    .add_resource_path(resource_dir)
        .build()
        .expect("nah dawg this sh aint workin");
        
    let the_game = MainState::new(&mut ctx)?;

    ggez::event::run(ctx, event_loop, the_game);
}
