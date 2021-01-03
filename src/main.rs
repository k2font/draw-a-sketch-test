// これがアプリケーションを動作させる最小構成
use nannou::prelude::*;

fn main() {
    // 単一のウインドウを用意する命令
    nannou::sketch(view).run();
}

// 実行中はループする
fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLUE);

    draw.to_frame(app, &frame).unwrap();
}