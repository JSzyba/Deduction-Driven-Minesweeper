use macroquad::prelude::*;

const GRID_SIZE: usize = 10;
const TILE_SIZE: f32 = 40.0;

#[derive(Clone, Copy, PartialEq)]
enum TileState {
    Hidden,
    Revealed,
    Flagged,
}

struct Tile {
    is_mine: bool,
    neighbor_mines: u8,
    state: TileState,
}

struct Game {
    grid: Vec<Vec<Tile>>,
}

impl Game {
    fn new() -> Self {
        let mut grid = vec![
            vec![
                Tile {
                    is_mine: false,
                    neighbor_mines: 0,
                    state: TileState::Hidden,
                };
                GRID_SIZE
            ];
            GRID_SIZE
        ];

        // TODO: Place mines and calculate neighbor_mines

        Game { grid }
    }

    fn draw(&self) {
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                let tile = &self.grid[y][x];
                let pos_x = x as f32 * TILE_SIZE;
                let pos_y = y as f32 * TILE_SIZE;

                let color = match tile.state {
                    TileState::Hidden => GRAY,
                    TileState::Revealed => WHITE,
                    TileState::Flagged => RED,
                };

                draw_rectangle(pos_x, pos_y, TILE_SIZE - 2.0, TILE_SIZE - 2.0, color);

                if tile.state == TileState::Revealed {
                    if tile.is_mine {
                        draw_text("*", pos_x + 12.0, pos_y + 28.0, 24.0, BLACK);
                    } else if tile.neighbor_mines > 0 {
                        draw_text(
                            &tile.neighbor_mines.to_string(),
                            pos_x + 12.0,
                            pos_y + 28.0,
                            24.0,
                            BLACK,
                        );
                    }
                }
            }
        }
    }

    fn handle_click(&mut self, mouse_pos: Vec2, right_click: bool) {
        let x = (mouse_pos.x / TILE_SIZE) as usize;
        let y = (mouse_pos.y / TILE_SIZE) as usize;

        if x >= GRID_SIZE || y >= GRID_SIZE {
            return;
        }

        let tile = &mut self.grid[y][x];

        if right_click {
            if tile.state == TileState::Hidden {
                tile.state = TileState::Flagged;
            } else if tile.state == TileState::Flagged {
                tile.state = TileState::Hidden;
            }
        } else {
            if tile.state == TileState::Hidden {
                tile.state = TileState::Revealed;
                // TODO: Add explosions and empty fields uncovering
            }
        }
    }
}

#[macroquad::main("Saper")]
async fn main() {
    let mut game = Game::new();

    loop {
        clear_background(DARKGREEN);

        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position().into();
            game.handle_click(mouse_pos, false);
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            let mouse_pos = mouse_position().into();
            game.handle_click(mouse_pos, true);
        }

        game.draw();

        next_frame().await;
    }
}
