use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, TextureValueError};
use sdl2::surface::Surface;
use sdl2::ttf::Font;

use std::path::Path;

use std::time::Duration;

use rand::prelude::*;

fn create_rectangle_texture<T>(
    texture_creator: &TextureCreator<T>,
    width: u32,
    height: u32,
    color: Color,
) -> Result<Texture, TextureValueError> {
    let mut surface = Surface::new(width, height, PixelFormatEnum::RGB24).unwrap();
    let _res = surface.fill_rect(Rect::new(0, 0, width, height), color);
    texture_creator.create_texture_from_surface(surface)
}

fn create_blended_text_texture<'a, T: 'a>(
    font: &Font,
    texture_creator: &'a TextureCreator<T>,
    text: &str,
    color: Color,
) -> Result<Texture<'a>, TextureValueError> {
    let surface = font.render(text).blended(color).unwrap();
    texture_creator.create_texture_from_surface(surface)
}

fn index(i: usize, j: usize, width: usize) -> usize {
    j * width + i
}

enum MoveDirection {
    Left,
    Right,
    Up,
    Down
}

fn move_grid(dir: MoveDirection, mut game_grid: &mut Grid) {
    let mut has_moved = false;

    let grid = &mut game_grid.grid;

    let _res = match dir {
        MoveDirection::Left => {
            for i in 1..4 {
                for j in 0..4 {
                    let target_index = index(i - 1, j, game_grid.width);
                    let value_index = index(i, j, game_grid.width);

                    let target = grid[target_index];
                    let value = grid[value_index];

                    if value == 0 {
                        continue;
                    }

                    if target == value || target == 0 {
                        grid[target_index] = target + value;
                        grid[value_index] = 0;
                        has_moved = true;
                    }
                }
            }
        }
        MoveDirection::Right => {
            for i in 1..4 {
                for j in 0..4 {
                    let target_index = index(3 - i + 1, j, game_grid.width);
                    let value_index = index(3 - i, j, 4);

                    let target = grid[target_index];
                    let value = grid[value_index];

                    if value == 0 {
                        continue;
                    }

                    if target == value || target == 0 {
                        grid[target_index] = target + value;
                        grid[value_index] = 0;
                        has_moved = true;
                    }
                }
            }
        }
        MoveDirection::Up => {
            for i in 0..4 {
                for j in 1..4 {
                    let target_index = index(i, j - 1, game_grid.width);
                    let value_index = index(i, j, game_grid.width);

                    let target = grid[target_index];
                    let value = grid[value_index];

                    if value == 0 {
                        continue;
                    }

                    if target == value || target == 0 {
                        grid[target_index] = target + value;
                        grid[value_index] = 0;
                        has_moved = true;
                    }
                }
            }
        }
        MoveDirection::Down => {
            for i in 0..4 {
                for j in 1..4 {
                    let target_index = index(i, 3 - j + 1, game_grid.width);
                    let value_index = index(i, 3 - j, game_grid.width);

                    let target = grid[target_index];
                    let value = grid[value_index];

                    if value == 0 {
                        continue;
                    }

                    if target == value || target == 0 {
                        grid[target_index] = target + value;
                        grid[value_index] = 0;
                        has_moved = true;
                    }
                }
            }
        }
    };

    if has_moved {
        move_grid(dir, &mut game_grid);
    }
}


fn generate_either_2_or_4(rng: &mut ThreadRng) -> u32 {
    if rng.gen() { 2 } else { 4 }
}

fn hex_string_to_color(color_string: &str) -> Color {
    Color::RGB(
        u8::from_str_radix(&color_string[0..2], 16).unwrap(),
        u8::from_str_radix(&color_string[2..4], 16).unwrap(),
        u8::from_str_radix(&color_string[4..6], 16).unwrap(),
    )
}

fn power2_to_index(power2: usize) -> usize {
    match power2 {
        0 => 0,
        2 => 1,
        4 => 2,
        8 => 3,
        16 => 4,
        32 => 5,
        64 => 6,
        128 => 7,
        256 => 8,
        512 => 9,
        1024 => 10,
        2048 => 11,
        _ => 1,
    }
}


struct Grid {
    grid: Vec<u32>,
    width: usize,
    height: usize
}

impl Grid {

    fn fill(&mut self, val: u32) {
        self.grid = (0..(self.width * self.height)).map(|_| val).collect()
    }

    fn new(num_cell_width: usize, num_cell_height: usize) -> Self {
        Grid { width: num_cell_width, height: num_cell_height, grid: (0..(num_cell_width * num_cell_height)).map(|_| 0).collect() }
    }
}

fn add_to_empty_cell(rng: &mut ThreadRng, grid: &mut Grid, val: u32) -> bool {

    let length = grid.grid.len();

    let mut has_oppening = false;
    for target in &grid.grid {
        if *target == 0 {
            has_oppening = true;
        }
    }

    if !has_oppening {
        return false;
    }

    loop {
        let index = rng.gen_range(0, length);
        if grid.grid[index] == 0 {
            grid.grid[index] = val;
            return true;
        }
    }
}

fn reset_grid(mut rng: &mut ThreadRng, mut grid: &mut Grid) {
    grid.fill(0);

    let val = generate_either_2_or_4(&mut rng);
    let _ignore = add_to_empty_cell(&mut rng, &mut grid, val);
    let val = generate_either_2_or_4(&mut rng);
    let _ignore = add_to_empty_cell(&mut rng, &mut grid, val);
    let val = generate_either_2_or_4(&mut rng);
    let _ignore = add_to_empty_cell(&mut rng, &mut grid, val);
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let window_width = 800;
    let window_height = 600;

    let grid_cell_width = 4;
    let grid_cell_height = 4;

    let window = video_subsystem
        .window("2048", window_width, window_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    let _res = canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    let width = 128;
    let height = 128;
    let board_background = Color::RGBA(119, 110, 101, 255);

    let black_text = hex_string_to_color("776e65");
    let white_text = hex_string_to_color("f9f6f2");

    let number_colors = [
        "eee4da", "ede0c8", "f2b179", "f59563", "f67c5f", "f65e3b", "edcf72", "edcc61", "edc850",
        "edc53f", "edc22e ",
    ]   
    .iter()
    .map(|color_string| hex_string_to_color(color_string))
    .collect::<Vec<Color>>();

    let padding_x: i32 = 20;
    let padding_y: i32 = 20;

    let margin = 10;

    let font_path: &Path = Path::new("assets/RobotoMono-Regular.ttf");

    let font: Font = ttf_context.load_font(font_path, 256).unwrap();

    let mut rng = thread_rng();

    let mut grid= Grid::new(grid_cell_width, grid_cell_height);


    reset_grid(&mut rng, &mut grid);

    let mut game_running = true;

    'running: loop {

        let mut moved = false;
        let mut dir = None;

        // INPUT
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    dir = Some(MoveDirection::Left);
                    move_grid(MoveDirection::Left, &mut grid);
                    let val = generate_either_2_or_4(&mut rng);

                    let res = add_to_empty_cell(&mut rng, &mut grid, val);

                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    move_grid(MoveDirection::Right, &mut grid);
                    let val = generate_either_2_or_4(&mut rng);

                    let res = add_to_empty_cell(&mut rng, &mut grid, val);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    move_grid(MoveDirection::Up, &mut grid);
                    let val = generate_either_2_or_4(&mut rng);

                    let res = add_to_empty_cell(&mut rng, &mut grid, val);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    move_grid(MoveDirection::Down, &mut grid);

                }
                _ => {}
            }
        }

        // UPDATE
        if moved {
            match dir {
                Some(movement) => {
                    move_grid(movement, &mut grid);
                    let val = generate_either_2_or_4(&mut rng);
                    let res = add_to_empty_cell(&mut rng, &mut grid, val);
                    if !res {
                        println!("You lose!");
                    }
                },
                None => ()
            }
        }


        // RENDERING
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();


        let center_x:i32 = (window_width / 2 - grid_cell_width as u32 * width / 2 - (padding_x as u32)) as i32;
        let center_y:i32 = (window_height / 2 - grid_cell_height as u32 * height / 2 - (padding_y as u32)) as i32;
        let center_board_x = center_x - padding_x / 2;
        let center_board_y = center_y - padding_y / 2;

        let board = create_rectangle_texture(
            &texture_creator,
            (grid_cell_width as i32 * (width as i32) + margin + padding_x) as u32,
            (grid_cell_height as i32 * (height as i32) + margin + padding_y) as u32,
            board_background,
        )
        .unwrap();

        canvas.copy(
            &board,
            None,
            Rect::new(
                center_board_x as i32,
                center_board_y as i32,
                grid_cell_height as u32 * (width + (padding_x as u32)),
                grid_cell_height as u32 * (height + (padding_y as u32)),
            ),
        )?;

        for i in 0..grid_cell_width {
            for j in 0..grid_cell_height {
                let rect = Rect::new(
                    center_x + i as i32 * ((width as i32) + padding_x),
                    center_y + j as i32 * ((height as i32) + padding_y),
                    width,
                    height,
                );

                let power2 = grid.grid[index(i as usize, j as usize, grid.width)];

                let tex: Texture = create_rectangle_texture(
                    &texture_creator,
                    width - 10,
                    height - 10,
                    number_colors[power2_to_index(power2 as usize)],
                )
                .unwrap();

                canvas.copy(&tex, None, rect)?;

                if power2 == 0 {
                    continue;
                }

                let number_texture = create_blended_text_texture(
                    &font,
                    &texture_creator,
                    power2.to_string().as_str(),
                    if power2 < 8 { black_text } else { white_text },
                )
                .unwrap();
                canvas.copy(&number_texture, None, rect)?;
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60))
    }

    Ok(())
}
