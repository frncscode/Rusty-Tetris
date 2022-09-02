use macroquad::prelude::*;
use tetris::game::Game;

pub mod tetris;

pub const BOARD_SIZE: (i32, i32) = (10, 24);
pub const TILE_SIZE: (i32, i32) = (30, 30);

#[macroquad::main("Tetris")]
async fn main() {
    // load font
    let font = load_ttf_font("src/Poppins-Black.ttf").await.unwrap();

    let mut tetris = Game::new();

    // sub time manageemnet
    let mut move_count = 0;

    // main time management
    let mut ticks = 1;
    let mut update_interval = 30;

    loop {
        // update with other actions
        tetris.update(&ticks, &update_interval, move_count);

        // all key detection 
        if is_key_down(KeyCode::Down) {
            update_interval = 5;
        } else {
            update_interval = 30;
        }

        // render
        clear_background(WHITE);
        tetris.show();
        // draw the score
        // let dims = measure_text(&format!("Score: {}", tetris.score())[..], 30.0);
        let text_params = TextParams {
            font,
            font_size: 30,
            font_scale: 1.0,
            color: GREEN,
            ..Default::default()
        };

        draw_text_ex(&format!("Score: {}", tetris.score())[..], 350.0, 100.0, text_params);
        // draw next shape
        tetris.next_shape.draw_somewhere(400.0, 150.0);
        // tetris.draw_landing_prediction(); currently faulty so leave it for now
        draw_rectangle_lines(0.0, 0.0, (BOARD_SIZE.0 * TILE_SIZE.0) as f32, (BOARD_SIZE.1 * TILE_SIZE.1) as f32, 2.0, DARKGRAY);

        next_frame().await;
        ticks += 1;
        move_count += 1;
    }
}
