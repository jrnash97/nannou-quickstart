use nannou::Draw;

// A custom trait to define behavior of drawable features of our app
pub trait Drawable {
    fn update(&mut self);
    fn draw(&self, draw: &Draw);
}
