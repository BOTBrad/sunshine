use std::io;

use sdl2;

use board;
use hero;

pub struct State {
  board: board::Board,
  hero: hero::Hero,
  canvas: sdl2::render::WindowCanvas,
}

impl State {
  pub fn new(width: u32, height: u32) -> State {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window(
      "rust-sdl2 demo: Video",
      (width + 2) * 10,
      (height + 2) * 10,
      ).build()
      .unwrap();

    State {
      board: board::Board {
        width: width,
        height: height,
      },
      hero: hero::Hero {
        pos: ((width as i32)/2, (height as i32)/2),
      },
      canvas: window.into_canvas().build().unwrap(),
    }
  }

  pub fn draw(&mut self) {
    let board = &self.board;
    let hero = &self.hero;

    // console output

    let mut wall = String::new();
    for _ in 0..(board.width + 2) {
      wall.push('#');
    }

    println!("{}", wall);
    for y in 0..board.height {
      print!("#");
      for x in 0..board.width {
        if (x as i32, y as i32) == hero.pos {
          print!("@");
        } else {
          print!(" ");
        }
      }
      print!("#\n");
    }
    println!("{}", wall);

    // sdl2 output

    self.canvas.clear();

    self.canvas.fill_rect(sdl2::rect::Rect::new(0, 0, (board.width+2)*10, 10)).unwrap();

    for y in 0..(board.height as i32) {
      self.canvas.fill_rect(sdl2::rect::Rect::new(0, (y+1)*10, 10, 10)).unwrap();
      for x in 0..(board.width as i32) {
        if (x, y) == hero.pos {
          self.canvas.fill_rect(sdl2::rect::Rect::new((x+1)*10, (y+1)*10, 10, 10)).unwrap();
        }
      }
      self.canvas.fill_rect(sdl2::rect::Rect::new(((board.width as i32)+1)*10, (y+1)*10, 10, 10)).unwrap();
    }

    self.canvas.fill_rect(sdl2::rect::Rect::new(0, ((board.height as i32)+1)*10, (board.width+2)*10, 10)).unwrap();

    self.canvas.present();
  }

  pub fn update(&mut self) -> bool {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("stdin failed");
    input.pop(); // remove the '\n' from the end

    if input == "exit" {
      return true;
    } else {
      let dir = hero::to_dir("d", "w", "a", "s", input.as_str());
      if let Some(d) = dir {
        self.hero.walk(d);
      }
    }
    false
  }
}
