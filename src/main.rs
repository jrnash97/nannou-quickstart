use nannou::prelude::*;
use nannou_quickstart::{model::*, traits::Drawable};

// Entry point of the Nannou app
fn main() {
    nannou::app(init).update(update).event(event).run();
}

//  windows and app state
fn init(app: &App) -> Model {
    // Create a new window
    app.new_window()
        .event(window_event)
        .view(view)
        .build()
        .unwrap();

    // Build and return a new model with the built-in Nannou HONEYDEW background color
    ModelBuilder::new()
        .bg_color(HONEYDEW.into_format::<f32>())
        .build()
}

// window_event() is called on any window event (e.g. KeyPressed)
fn window_event(_app: &App, _model: &mut Model, _event: WindowEvent) {}

// event() is called on any event (including updates and window events)
fn event(_app: &App, _model: &mut Model, _event: Event) {}

// update() is called every frame
fn update(_app: &App, model: &mut Model, _update: Update) {
    model.update();
}

// Draw the app state to the current frame
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(model.bg_color);
    model.draw(&draw);

    draw.to_frame(app, &frame).unwrap();
}
