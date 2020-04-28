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

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();

    let window = video_subsystem
        .window("2048", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    let _res = canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    let width = 64;
    let height = 64;
    let green = Color::RGB(0, 255, 0);
    let red = Color::RGB(255, 0, 0);

    let font_path: &Path = Path::new("assets/RobotoMono-Regular.ttf");

    let font: Font = ttf_context.load_font(font_path, 256).unwrap();

    let green_square:Texture = create_rectangle_texture(&texture_creator, width, height, green).unwrap();
    let _red_square:Texture = create_rectangle_texture(&texture_creator, width, height, red).unwrap();

    let mut i = 0;

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

        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        for i in 0..4 {
            for j in 0..4 {
                canvas.copy(
                    &green_square,
                    None,
                    Rect::new(50 + i * 100, 50 + j * 100, 64, 64),
                )?;

                let texture = create_blended_text_texture(
                    &font,
                    &texture_creator,
                    grid[index(i as usize, j as usize, 4)].to_string().as_str(),
                    Color::RGBA(255, 0, 0, 255),
                )
                .unwrap();
                canvas.copy(
                    &texture,
                    None,
                    Rect::new(50 + i * 100, 50 + j * 100, 64, 64),
                )?;
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60))
    }

    Ok(())
}
