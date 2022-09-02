use macroquad::prelude::*;
use crate::tetris::utils::*;

pub const BOARD_SIZE: (i32, i32) = (10, 24);
pub const TILE_SIZE: (i32, i32) = (30, 30);

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ShapeVariant {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

#[derive(Clone)]
pub struct Shape {
    pub tiles: Vec<Pos>,
    anchor_index: usize,
    pub variant: ShapeVariant,
    locked: bool,
    // tiles begin as the shapes normalized positions
}

impl Shape {
    pub fn center_x(&mut self) {
        for position in self.tiles.iter_mut() {
            position.x += BOARD_SIZE.0 / 2;
        }
    }

    pub fn draw_somewhere(&self, x: f32, y: f32) {
        for tile in &self.tiles {
            draw_rectangle(
                x + (tile.x * TILE_SIZE.0) as f32,
                y + (tile.y * TILE_SIZE.1) as f32,
                TILE_SIZE.0 as f32,
                TILE_SIZE.1 as f32,
                colour(self.variant),
            );
            draw_rectangle_lines(
                x + (tile.x * TILE_SIZE.0) as f32,
                y + (tile.y * TILE_SIZE.1) as f32,
                TILE_SIZE.0 as f32,
                TILE_SIZE.1 as f32,
                4.0,
                darken_colour(colour(self.variant)),
            );
        }
    }

    pub fn draw(&self) {
        for pos in self.tiles.iter() {
            draw_rectangle(
                (pos.x * TILE_SIZE.0) as f32,
                (pos.y * TILE_SIZE.1) as f32,
                TILE_SIZE.0 as f32,
                TILE_SIZE.1 as f32,
                colour(self.variant),
            );
            draw_rectangle_lines(
                (pos.x * TILE_SIZE.0) as f32,
                (pos.y * TILE_SIZE.1) as f32,
                TILE_SIZE.0 as f32,
                TILE_SIZE.1 as f32,
                4.0,
                darken_colour(colour(self.variant)),
            );
        }
    }

    pub fn gravity(&mut self) {
        if !self.locked {
            for tile in self.tiles.iter_mut() {
                tile.y += 1;
            }
        }
    }

    pub fn update_locked_status(&mut self, locked_shapes: &Vec<Shape>) {
        for tile in self.tiles.iter() {
            if tile.y >= BOARD_SIZE.1 - 1 {
                self.locked = true;
            }
        }
        // if still not locked check against other shapes
        if !self.locked {
            let mut all_tiles: Vec<Pos> = vec![];
            for shape in locked_shapes.iter() {
                for tile in shape.tiles.iter() {
                    all_tiles.push(tile.clone());
                }
            }

            for tile in self.tiles.iter() {
                if all_tiles.contains(&pos(tile.x, tile.y + 1)) {
                    self.locked = true;
                }
            }
        }
    }

    pub fn locked(&self) -> bool {
        self.locked
    }

    pub fn left_most(&self) -> Pos {
        let mut left_most = &self.tiles[0];

        for tile in &self.tiles {
            if tile.x < left_most.x {
                left_most = tile;
            }
        }

        *left_most
    }

    pub fn right_most(&self) -> Pos {
        let mut right_most = &self.tiles[0];

        for tile in &self.tiles {
            if tile.x > right_most.x {
                right_most = tile;
            }
        }

        *right_most
    }

    pub fn can_move_right(&self, locked_shapes: &Vec<Shape>) -> bool {
        let mut all_locked_tiles = vec![];
        for shape in locked_shapes.iter() {
            for tile in shape.tiles.iter() {
                all_locked_tiles.push(tile.clone());
            }
        }

        for tile in self.tiles.iter() {
            let moved_tile = Pos::new(tile.x + 1, tile.y);
            if all_locked_tiles.contains(&moved_tile) || moved_tile.x > BOARD_SIZE.0 - 1 {
                return false;
            }
        }

        return true;
    }

    pub fn can_move_left(&self, locked_shapes: &Vec<Shape>) -> bool {
        let mut all_locked_tiles = vec![];
        for shape in locked_shapes.iter() {
            for tile in shape.tiles.iter() {
                all_locked_tiles.push(tile.clone());
            }
        }

        for tile in self.tiles.iter() {
            let moved_tile = Pos::new(tile.x - 1, tile.y);
            if all_locked_tiles.contains(&moved_tile) || moved_tile.x < 0 {
                return false;
            }
        }

        return true;
    }

    pub fn can_rotate(&self, locked_shapes: &Vec<Shape>) -> bool {
        let mut rotated = vec![];
        for tile in self.tiles.iter() {
            rotated.push(rotate_around(tile.clone(), self.tiles[self.anchor_index].clone()));
        }

        let mut all_locked_tiles = vec![];
        for shape in locked_shapes.iter() {
            for tile in shape.tiles.iter() {
                all_locked_tiles.push(tile.clone());
            }
        }

        for tile in rotated.iter() {
            if all_locked_tiles.contains(&tile) || (tile.x < 0 || tile.x > BOARD_SIZE.0 - 1) {
                return false;
            }
        }

        return true;
    }

    pub fn remove_if_below_screen(&mut self) {
        self.tiles.retain(|tile| !(tile.y > BOARD_SIZE.1 - 1));
    }

    pub fn control(&mut self, move_count: i32, locked_shapes: &Vec<Shape>) {
        // move left and right a bit slower 
        if move_count % 5 == 0 {
            if is_key_down(KeyCode::Left) {
                // check if it is possible
                println!("try to move left");
                if self.can_move_left(&locked_shapes) {
                    for tile in self.tiles.iter_mut() {
                        tile.x -= 1;
                    }  
                }
            } else if is_key_down(KeyCode::Right) {
                // check if possible
                if self.can_move_right(&locked_shapes) {
                    for tile in self.tiles.iter_mut() {
                        tile.x += 1;
                    } 
                }
            }
        }

        if is_key_pressed(KeyCode::Up) {
            if self.can_rotate(&locked_shapes) {
                self.rotate(self.tiles[self.anchor_index].clone());
            }
        }
    }

    pub fn rotate(&mut self, anchor: Pos) {
        if self.variant == ShapeVariant::O {
            // if shape is O shape dont bother rotating
            return;
        }

        for point in self.tiles.iter_mut() {
            // rotate point
            *point = rotate_around(*point, anchor);
            println!("{:?}", point);
        }
    }
}

// I O T S Z J L
pub fn new_i() -> Shape {
    // ####

    Shape {
        tiles: vec![
            pos(-2, 0),
            pos(-1, 0),
            pos(0, 0),
            pos(1, 0),
        ],
        variant: ShapeVariant::I,
        locked: false,
        anchor_index: 1,
    }
}

pub fn new_o() -> Shape {
    // ##
    // ##

    Shape {
        tiles: vec![
            pos(-1, 0),
            pos(0, 0),
            pos(-1, 1),
            pos(0, 1),
        ],
        variant: ShapeVariant::O,
        locked: false,
        anchor_index: 3,
    }
}

pub fn new_t() -> Shape {
    // #
    //###

    Shape {
        tiles: vec![
            pos(-1, 1),
            pos(0, 1),
            pos(0, 0),
            pos(1, 1),
        ],
        variant: ShapeVariant::T,
        locked: false,
        anchor_index: 1,
    }
}

pub fn new_s() -> Shape {
    //  ##
    // ##

    Shape {
        tiles: vec![
            pos(-1, 1),
            pos(0, 1),
            pos(0, 0),
            pos(1, 0),
        ],
        variant: ShapeVariant::S,
        locked: false,
        anchor_index: 1,
    }
}

pub fn new_z() -> Shape {
    // ##
    //  ##

    Shape {
        tiles: vec![
            pos(-1, 0),
            pos(0, 0),
            pos(0, 1),
            pos(1, 1),
        ],
        variant: ShapeVariant::Z,
        locked: false,
        anchor_index: 2,
    }
}

pub fn new_j() -> Shape {
    // #
    // ###

    Shape {
        tiles: vec![
            pos(-1, 0),
            pos(-1, 1),
            pos(0, 1),
            pos(1, 1),
        ],
        variant: ShapeVariant::J,
        locked: false,
        anchor_index: 2,
    }
}

pub fn new_l() -> Shape {
    //   #
    // ###

    Shape {
        tiles: vec![
            pos(-1, 1),
            pos(0, 1),
            pos(1, 1),
            pos(1, 0),
        ],
        variant: ShapeVariant::L,
        locked: false,
        anchor_index:1,
    }
}

pub fn colour(variant: ShapeVariant) -> Color {
    let colour = match variant {
        ShapeVariant::I => SKYBLUE,
        ShapeVariant::O => YELLOW,
        ShapeVariant::T => PURPLE,
        ShapeVariant::S => GREEN,
        ShapeVariant::Z => RED,
        ShapeVariant::J => BLUE,
        ShapeVariant::L => ORANGE,
    };

    colour
}