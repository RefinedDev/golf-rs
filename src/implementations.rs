use super::misc::mathfuncs::check_collision_for_quads;
use ggez::{Context, GameResult, graphics::{self, Image}, input, audio};
use glam::Vec2;

pub struct MainState {
    pub player: Player,
    pub hole: Hole,
    pub assets: Assets,
    pub holding_lmb: bool,
    
    pub mousevel_buildup: f32,
    pub level: i32,
    pub obstacle_cords: Vec<Vec2>,
    pub obstacle_size: Vec<i32>,
    pub strokes: i32,
    pub completed: bool,
    pub hit_hole: bool
}

pub struct Player { 
    pub position: Vec2,
    pub velocity: f32,
    pub rotation: f32,
    pub anglevec: Vec2,
    pub mouse_pos_when_shot: Vec2,
    pub pos_before_shot: Vec2,
    pub size: f32,
    pub shakevec: Vec2
}

pub struct Hole {
    pub position: Vec2,
}

pub struct Assets{
    pub player_img: Image,
    pub arrow: Image,
    pub hole: Image, 
    pub velobar: Image,
    pub velobar2: Image,
    pub bg: Image,
    pub obstacle: Image,
    pub small_obstacle: Image,

    pub shoot_sound: audio::Source,
    pub charge_sound: audio::Source,
    pub hole_sound: audio::Source,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        let (w, h) = graphics::drawable_size(ctx);

        let plr_pos = Vec2::new(w*0.5,h*0.5+150.0);
        let hole_pos = Vec2::new(w*0.5,h*0.5-150.0);
        let obstacle_cords = Vec::new();

        Ok(MainState {
            player: Player { 
                position: plr_pos, 
                velocity: 0.0,
                rotation: 0.0,
                anglevec: Vec2::ZERO,
                mouse_pos_when_shot: Vec2::ZERO,
                pos_before_shot: Vec2::ZERO,
                size: 1.0,
                shakevec: Vec2::ZERO,
            },
            hole: Hole {
                position: hole_pos,
            },
            assets: Assets {
                player_img: graphics::Image::new(ctx, "/ball.png")?,
                arrow: graphics::Image::new(ctx, "/arrow.png")?,
                hole: graphics::Image::new(ctx, "/hole.png")?,
                velobar: graphics::Image::new(ctx, "/velocity_bar.png")?,
                velobar2: graphics::Image::new(ctx, "/velocity_bar2.png")?,
                bg: graphics::Image::new(ctx, "/bg.png")?,
                obstacle: graphics::Image::new(ctx, "/obstacle_64x67.png")?,
                small_obstacle: graphics::Image::new(ctx, "/obstacle_32x34.png")?,
                shoot_sound: audio::Source::new(ctx, "/shoot.mp3")?,
                hole_sound: audio::Source::new(ctx, "/hole.mp3")?,
                charge_sound: audio::Source::new(ctx, "/charge.mp3")?,
            },
            holding_lmb: false,
            mousevel_buildup: 0.0,
            level: 0,
            obstacle_cords,
            obstacle_size: Vec::new(),
            strokes: 0,
            completed: false,
            hit_hole: false
        })
    }

    pub fn change_player_rotation(&mut self, ctx: &mut Context) {
        let mouse_coords = input::mouse::position(ctx);
        let dx = self.player.position.x - mouse_coords.x;
        let dy = self.player.position.y - mouse_coords.y;
        let angle = dx.atan2(dy);

        self.player.rotation = -angle
    }

    pub fn check_collision_with_border(&mut self, ctx: &mut Context) {
        let (scr_width, scr_height) = graphics::drawable_size(ctx);

        if self.player.position.y < 16.0 {
            self.player.anglevec.y = self.player.anglevec.y.abs()
        } else if self.player.position.y > scr_height - 16.0 { 
            self.player.anglevec.y = -self.player.anglevec.y.abs()
        } else if self.player.position.x < 16.0 {
             self.player.anglevec.x = self.player.anglevec.x.abs()
        } else if self.player.position.x > scr_width - 16.0 { 
             self.player.anglevec.x = -self.player.anglevec.x.abs()
        }
    }

    pub fn check_collision_with_obstacles(&mut self, ctx: &mut Context) {
        let dt = ggez::timer::delta(ctx).as_secs_f32();

        for i in &self.obstacle_cords {
            let player_x = self.player.position.x + (self.player.velocity * self.player.anglevec).x * dt;
            let player_y = self.player.position.y + (self.player.velocity * self.player.anglevec).y * dt;

            if  check_collision_for_quads(player_x, self.player.position.y, 32.0,32.0,i.x.to_owned(), i.y.to_owned(), 32.0,34.0) {
                self.player.anglevec.x = -self.player.anglevec.x;
            }

            if check_collision_for_quads(self.player.position.x, player_y, 32.0,32.0,i.x.to_owned(), i.y.to_owned(), 32.0,34.0) {
                self.player.anglevec.y = -self.player.anglevec.y;
            }
        }
    }

    pub fn check_collision_with_hole(&mut self) -> bool {
        let hole_radius = 16.0;
        let player_radius = 16.0;

        let dx = (self.player.position.x + player_radius) - (self.hole.position.x + hole_radius);
        let dy = (self.player.position.y + player_radius) - (self.hole.position.y + hole_radius);
        let distance = (dx * dx + dy * dy).sqrt();

        if distance < player_radius + hole_radius {
            true
        } else {
            false
        }
    }

    pub fn get_level_obstacles(&mut self, ctx: &mut Context) {
        self.obstacle_cords.clear();
        self.obstacle_size.clear();

        let (w, h) = graphics::drawable_size(ctx);
        match self.level {
            1 => {
                self.player.position = Vec2::new(w*0.5,h*0.5+150.0);
                self.hole.position = Vec2::new(w*0.5,h*0.5-150.0);
              
                self.obstacle_cords.push(Vec2::new(360.0,159.0));
                self.obstacle_size.push(32);

                self.obstacle_cords.push(Vec2::new(442.0,159.0));
                self.obstacle_size.push(32);

                self.obstacle_cords.push(Vec2::new(400.0,193.0));
                self.obstacle_size.push(32);

            }

            2 => {
                self.player.position = Vec2::new(141.49, 100.0);
                self.hole.position = Vec2::new(623.0, 380.0);
               
                // 32 x 34
                self.obstacle_cords.push(Vec2::new(583.0,338.0));
                self.obstacle_size.push(32);

                self.obstacle_cords.push(Vec2::new(583.0,381.0));
                self.obstacle_size.push(32);

                self.obstacle_cords.push(Vec2::new(583.0,424.0));
                self.obstacle_size.push(32);

                // 64 x 67
                self.obstacle_cords.push(Vec2::new(399.0,133.0));
                self.obstacle_size.push(64);

                self.obstacle_cords.push(Vec2::new(335.0,199.0));
                self.obstacle_size.push(64);

                self.obstacle_cords.push(Vec2::new(271.0,266.0));
                self.obstacle_size.push(64);

                self.obstacle_cords.push(Vec2::new(141.0,388.0));
                self.obstacle_size.push(64);

                self.obstacle_cords.push(Vec2::new(207.0,330.0));
                self.obstacle_size.push(64);
            }   

            3 => {
                self.player.position = Vec2::new(384.49, 58.0);
                self.hole.position = Vec2::new(384.0, 457.0);
               
                // 32 x 34
                self.obstacle_cords.push(Vec2::new(502.0,217.0));
                self.obstacle_size.push(32);

                self.obstacle_cords.push(Vec2::new(268.0,340.0));
                self.obstacle_size.push(32);

                // 64 x 67
                self.obstacle_cords.push(Vec2::new(534.0,123.0));
                self.obstacle_size.push(64);

                self.obstacle_cords.push(Vec2::new(236.0,190.0));
                self.obstacle_size.push(64);


                self.obstacle_cords.push(Vec2::new(384.0,347.0));
                self.obstacle_size.push(64);

                self.obstacle_cords.push(Vec2::new(534.0,414.0));
                self.obstacle_size.push(64);
            }

            4 => {
                self.player.position = Vec2::new(98.49, 300.0);
                self.hole.position = Vec2::new(705.0, 300.0);
               
                // 32 x 34
                self.obstacle_cords.push(Vec2::new(400.0,133.0));
                self.obstacle_size.push(32);

                self.obstacle_cords.push(Vec2::new(400.0,502.0));
                self.obstacle_size.push(32);

                // 64 x 67
                self.obstacle_cords.push(Vec2::new(400.0,250.0));
                self.obstacle_size.push(64);

                self.obstacle_cords.push(Vec2::new(400.0,317.0));
                self.obstacle_size.push(64);

                self.obstacle_cords.push(Vec2::new(400.0,384.0));
                self.obstacle_size.push(64);
            }

            5 => {
                self.player.position = Vec2::new(384.25, 462.0);
                self.hole.position = Vec2::new(384.0, 97.0);

                self.obstacle_cords.push(Vec2::new(96.0,218.0));
                self.obstacle_cords.push(Vec2::new(144.0,218.0));
                self.obstacle_cords.push(Vec2::new(192.0,218.0));
                self.obstacle_cords.push(Vec2::new(240.0,218.0));
                self.obstacle_cords.push(Vec2::new(288.0,218.0));
                self.obstacle_cords.push(Vec2::new(336.0,218.0));
                self.obstacle_cords.push(Vec2::new(384.0,218.0));
                self.obstacle_cords.push(Vec2::new(432.0,218.0));
                self.obstacle_cords.push(Vec2::new(480.0,218.0));
                self.obstacle_cords.push(Vec2::new(528.0,218.0));
                self.obstacle_cords.push(Vec2::new(576.0,218.0));
                self.obstacle_cords.push(Vec2::new(624.0,218.0));
                self.obstacle_cords.push(Vec2::new(672.0,218.0));
                self.obstacle_cords.push(Vec2::new(720.0,218.0));

                for _ in 0..=self.obstacle_cords.len() {
                    self.obstacle_size.push(32);
                }
            }

            6 => {
                self.player.position = Vec2::new(183.25, 460.0);
                self.hole.position = Vec2::new(183.0, 99.0);
               
                // 32 x 34
                self.obstacle_cords.push(Vec2::new(138.0,155.0));
                self.obstacle_size.push(32);

                self.obstacle_cords.push(Vec2::new(283.0,132.0));
                self.obstacle_size.push(32);

                self.obstacle_cords.push(Vec2::new(183.0,224.0));
                self.obstacle_size.push(32);

                self.obstacle_cords.push(Vec2::new(344.0,172.0));
                self.obstacle_size.push(32);

                self.obstacle_cords.push(Vec2::new(183.0,377.0));
                self.obstacle_size.push(32);

                self.obstacle_cords.push(Vec2::new(400.0,133.0));
                self.obstacle_size.push(32);

                self.obstacle_cords.push(Vec2::new(384.0,343.0));
                self.obstacle_size.push(32);

                self.obstacle_cords.push(Vec2::new(267.0,419.0));
                self.obstacle_size.push(32);

                // 64 x 67
                self.obstacle_cords.push(Vec2::new(235.0,292.0));
                self.obstacle_size.push(64);

                self.obstacle_cords.push(Vec2::new(392.0,240.0));
                self.obstacle_size.push(64);
            }
            
            _ => {}
        }

        self.player.size = 1.0;
    }
}