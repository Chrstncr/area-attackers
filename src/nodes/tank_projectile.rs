use macroquad::prelude::*;

/// # Tank projectile
/// this is the projectile that the tank will fire
pub struct TankProjectile {
    pub size: Vec2,
    pub position: Vec2,
    speed: u16,
    texture: Texture2D,
    should_render: bool,
}

impl TankProjectile {
    /// create a new projectile with its texture as a parameter
    pub fn new(texture: Texture2D) -> Self {
        Self {
            size: vec2(6.9, 20.),
            speed: 1000,
            position: vec2(0., 0.),
            should_render: false,
            texture,
        }
    }
    // function to shoot
    pub fn shoot(&mut self, position: Vec2) {
        //make sure its not already shot
        if !self.should_render {
            self.should_render = true;
            //set its position to the one of the tank
            self.position = vec2(position.x - self.size.x, position.y);
        }
    }

    pub fn hit_an_enemy(&mut self) {
        self.should_render = false;
        self.position = vec2(0., 0.);
    }

    pub fn render(&mut self) {
        //should render is only true if the y position of the projectile is > 0
        self.should_render = self.position.y > 0.;
        if self.should_render {
            // move the projectile up (counterintuitively decrementing the value)
            self.position.y -= self.speed as f32 * get_frame_time();
            // render the texture for the projectile
            draw_texture_ex(
                self.texture,
                self.position.x,
                self.position.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Option::from(self.size),
                    ..Default::default()
                },
            )
        }
    }
}
