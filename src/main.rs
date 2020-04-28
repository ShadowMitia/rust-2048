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
    Down,
}

fn move_grid(dir: MoveDirection, grid: &mut Vec<u32>) {
    let mut has_moved = false;

    let _res = match dir {
        MoveDirection::Left => {
            for i in 1..4 {
                for j in 0..4 {
                    let target_index = index(i - 1, j, 4);
                    let value_index = index(i, j, 4);

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
                    let target_index = index(3 - i + 1, j, 4);
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
                    let target_index = index(i, j - 1, 4);
                    let value_index = index(i, j, 4);

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
                    let target_index = index(i, 3 - j + 1, 4);
                    let value_index = index(i, 3 - j, 4);

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
        move_grid(dir, grid);
    }
}

fn add_random_starter_number(rng: &mut ThreadRng, grid: &Vec<u32>) -> Vec<u32> {
    let val = if rng.gen() { 2 } else { 4 };

    let mut new_grid = grid.clone();

    let length = grid.len();

    let mut has_oppening = false;
    for target in grid {
        if *target == 0 {
            has_oppening = true;
        }
    }

    if !has_oppening {
        return new_grid;
    }

    loop {
        let index = rng.gen_range(0, length);
        if new_grid[index] == 0 {
            new_grid[index] = val;
            return new_grid;
        }
    }
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

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let window_width = 800;
    let window_height = 600;

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

    let font_path: &Path = Path::new("assets/RobotoMono-Regular.ttf");

    let font: Font = ttf_context.load_font(font_path, 256).unwrap();

    let mut rng = thread_rng();

    let mut grid: Vec<u32> = (0..4 * 4).map(|_| 0).collect();

    grid = add_random_starter_number(&mut rng, &mut grid);

    'running: loop {
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
                    move_grid(MoveDirection::Left, &mut grid);
                    grid = add_random_starter_number(&mut rng, &mut grid);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    move_grid(MoveDirection::Right, &mut grid);
                    grid = add_random_starter_number(&mut rng, &mut grid);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    move_grid(MoveDirection::Up, &mut grid);
                    grid = add_random_starter_number(&mut rng, &mut grid);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    move_grid(MoveDirection::Down, &mut grid);
                    grid = add_random_starter_number(&mut rng, &mut grid);
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::WHITE);
        canvas.clear();

        let padding_x: i32 = 20;
        let padding_y: i32 = 20;

        let margin = 10;
        let center_x:i32 = (window_width / 2 - 4 * width / 2 - (padding_x as u32)) as i32;
        let center_y:i32 = (window_height / 2 - 4 * height / 2 - (padding_y as u32)) as i32;
        let center_board_x = center_x - padding_x / 2;
        let center_board_y = center_y - padding_y / 2;

        let board = create_rectangle_texture(
            &texture_creator,
            (4 * (width as i32) + margin + padding_x) as u32,
            (4 * (height as i32) + margin + padding_y) as u32,
            board_background,
        )
        .unwrap();

        canvas.copy(
            &board,
            None,
            Rect::new(
                center_board_x as i32,
                center_board_y as i32,
                4 * (width + (padding_x as u32)),
                4 * (height + (padding_y as u32)),
            ),
        )?;

        for i in 0..4 {
            for j in 0..4 {
                let rect = Rect::new(
                    center_x + i * ((width as i32) + padding_x),
                    center_y + j * ((height as i32) + padding_y),
                    width,
                    height,
                );

                let power2 = grid[index(i as usize, j as usize, 4)];

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
