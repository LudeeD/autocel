use macroquad::prelude::*;

mod world;

fn window_conf() -> Conf {
    Conf {
        window_title: "AutoCell".to_owned(),
        window_width: 800,
        window_height: 800,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = world::SandWorld::new(80, 80, 10);

    loop {
        clear_background(WHITE);

        draw_text(format!("FPS: {}", get_fps()).as_str(), 0., 16., 32., BLUE);

        world.update();

        world.commit_cells();

        world.draw();

        next_frame().await
    }
}
