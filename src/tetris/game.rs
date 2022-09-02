use macroquad::prelude::*;
use crate::tetris::utils::*;
use crate::tetris::shapes::*;

pub const BOARD_SIZE: (i32, i32) = (10, 24);
pub const TILE_SIZE: (i32, i32) = (30, 30);

pub struct Game {
    locked_shapes: Vec<Shape>,
    dynamic_shapes: Vec<Shape>,
    score: usize,
    pub next_shape: Shape,
}

impl Game {
    pub fn new() -> Self {
        Self {
            locked_shapes: vec![],
            dynamic_shapes: vec![],
            score: 0,
            next_shape: Self::spawn_shape(),
        }
    }

    pub fn spawn_shape() -> Shape {
        let new_shape = match rand::gen_range::<i32>(0, 6) {
            0 => new_i(),
            1 => new_o(),
            2 => new_t(),
            3 => new_s(),
            4 => new_z(),
            5 => new_j(),
            6 => new_l(),
            num => panic!("num generated: {}", num),
        };
        new_shape
    }

    pub fn reset(&mut self) {
        self.locked_shapes.clear();
        self.dynamic_shapes.clear();
        self.score = 0;
    }

    pub fn show(&self) {
        const size: usize = TILE_SIZE.0 as usize;
        for x in (0..300).step_by(size) {
            for y in (0..720).step_by(size) {
                draw_rectangle_lines(
                    x as f32,
                    y as f32,
                    TILE_SIZE.0 as f32,
                    TILE_SIZE.1 as f32,
                    1.0,
                    color_u8!(0, 0, 0, 100),
                );
            }
        }

        for shape in &self.locked_shapes {
            shape.draw();
        }
        for shape in &self.dynamic_shapes {
            shape.draw();
        }
    }

    pub fn update(&mut self, ticks: &i32, interval: &i32, move_count: i32) {

        if ticks % interval == 0 {
            // update that happens each game tick
            if self.dynamic_shapes.is_empty() {
                // spawn a new shape
                // let mut shape = match rand::gen_range::<i32>(0, 6) {
                let mut new_shape = self.next_shape.clone();
                // move shape to center when spawning in
                new_shape.center_x();

                self.dynamic_shapes.push(new_shape.clone());
                self.next_shape = Self::spawn_shape();

                
                let mut all_locked_tiles = vec![];
                for shape in self.locked_shapes.iter() {
                    for tile in shape.tiles.iter() {
                        all_locked_tiles.push(tile.clone());
                    }
                }

                for tile in new_shape.tiles.iter() {
                    if all_locked_tiles.contains(&tile) {
                        self.reset();
                        return;
                    }
                }
            }

            for shape in self.dynamic_shapes.iter_mut() {
                shape.gravity();
                self.score += 10;
            }
        }

        let mut clear = false;
        for shape in self.dynamic_shapes.iter_mut() {
            shape.update_locked_status(&self.locked_shapes);
            if shape.locked() {
                clear = true;
            }
        }
        if clear {
            let new_shape: Shape = self.dynamic_shapes.pop().unwrap();
            self.locked_shapes.push(new_shape.clone());
        }

        for shape in self.dynamic_shapes.iter_mut() {
            shape.control(move_count, &self.locked_shapes);
        }

        if self.bottom_row_filled() {
            self.score += 1000;
            for shape in self.locked_shapes.iter_mut() {
                for tile in shape.tiles.iter_mut() {
                    tile.y += 1;
                }
            }
        }

        for shape in self.locked_shapes.iter_mut() {
            shape.remove_if_below_screen();
        }
    }

    pub fn score(&self) -> usize {
        self.score
    }

    pub fn draw_landing_prediction(&self) {
        let mut all_locked_tiles: Vec<Pos> = vec![];
        for shape in self.locked_shapes.iter() {
            for tile in shape.tiles.iter() {
                all_locked_tiles.push(tile.clone());
            }
        }

        for shape in self.dynamic_shapes.iter() {
            for tile in &shape.tiles {
                let mut pos = tile.clone();
                while (!(all_locked_tiles.contains(&Pos::new(pos.x, pos.y + 1)))) && pos.y < BOARD_SIZE.1 - 1  {
                    pos.y += 1;
                }
                // draw an indication tile
                draw_rectangle_lines(
                    (pos.x as f32) * (TILE_SIZE.0 as f32),
                    (pos.y as f32) * (TILE_SIZE.1 as f32),
                    TILE_SIZE.0 as f32,
                    TILE_SIZE.1 as f32,
                    1.0,
                    colour(shape.variant)
                );
            }        
        }
    }

    pub fn bottom_row_filled(&self) -> bool {
        let mut tiles_left = BOARD_SIZE.0;

        for shape in self.locked_shapes.iter() {
            for tile in shape.tiles.iter() {
                if tile.y == BOARD_SIZE.1 - 1 {
                    tiles_left -= 1;
                }
            }
        }

        return tiles_left == 0;

    }
}