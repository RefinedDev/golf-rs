use glam::Vec2;

pub fn get_distance(a: Vec2, b: Vec2,t :f32) -> f32 {
    let vel =  ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt() / t;
    vel
}

pub fn vec_from_angle(angle: f32) -> Vec2 {
    let vx = angle.sin();
    let vy = angle.cos();
    Vec2::new(vx, vy)
}

#[allow(non_snake_case)]
pub fn check_collision_for_quads(plrX: f32, plrY: f32, plrW: f32, plrH: f32, objX: f32, objY: f32, objW: f32, objH: f32) -> bool {
    return plrX < objX + objW && plrX + plrW > objX && plrY < objY + objH && plrY + plrH > objY
}