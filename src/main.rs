use nannou::prelude::*;

struct Model {
    rotation: f32,
}

fn model(app: &App) -> Model {
    app.new_window().view(view).build().unwrap();
    Model { rotation: 0.0 }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.rotation += 0.01;
}

// Put your drawing code, called once per frame, per window.
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(DIMGRAY);
    draw.rect()
        .rotate(model.rotation)
        .w(100.0)
        .h(100.0)
        .color(RED);
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).run();
}
