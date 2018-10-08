extern crate cfg_if;
extern crate js_sys;
extern crate wasm_bindgen;

mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(msg: &str);
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ($($t:tt)*) => (log(&format!($($t)*)))
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = performance)]
  fn now() -> f64;
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn time(name: &str);

  #[wasm_bindgen(js_namespace = console)]
  fn timeEnd(name: &str);
}

pub struct Timer<'a> {
  name: &'a str,
}

impl<'a> Timer<'a> {
  pub fn new(name: &'a str) -> Timer<'a> {
    time(name);
    Timer { name }
  }
}

impl<'a> Drop for Timer<'a> {
  fn drop(&mut self) {
    timeEnd(self.name);
  }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
  Dead = 0,
  Alive = 1,
}

impl Cell {
  fn toggle(&mut self) {
    *self = match *self {
      Cell::Dead => Cell::Alive,
      Cell::Alive => Cell::Dead,
    }
  }
}

#[wasm_bindgen]
pub struct Universe {
  width: u32,
  height: u32,
  cells: Vec<Cell>,
}

impl Universe {
  fn get_index(&self, row: u32, column: u32) -> usize {
    (row * self.width + column) as usize
  }

  fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
    let mut count = 0;

    let top_row = if row == 0 { self.height - 1 } else { row - 1 };

    let bottom_row = if row == self.height - 1 { 0 } else { row + 1 };

    let right_col = if column == self.width - 1 {
      0
    } else {
      column + 1
    };

    let left_col = if column == 0 {
      self.width - 1
    } else {
      column - 1
    };

    // top
    count += self.cells[self.get_index(top_row, column)] as u8;
    // top right
    count += self.cells[self.get_index(top_row, right_col)] as u8;
    // right
    count += self.cells[self.get_index(row, right_col)] as u8;
    // bottom right
    count += self.cells[self.get_index(bottom_row, right_col)] as u8;
    // bottom
    count += self.cells[self.get_index(bottom_row, column)] as u8;
    // bottom left
    count += self.cells[self.get_index(bottom_row, left_col)] as u8;
    // left
    count += self.cells[self.get_index(row, left_col)] as u8;
    // top left
    count += self.cells[self.get_index(top_row, left_col)] as u8;

    count
  }
}

impl fmt::Display for Universe {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for line in self.cells.as_slice().chunks(self.width as usize) {
      for &cell in line {
        let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
        write!(f, "{}", symbol)?;
      }
      write!(f, "\n")?;
    }

    Ok(())
  }
}

#[wasm_bindgen]
impl Universe {
  pub fn tick(&mut self) {
    // let _timer = Timer::new("Universe::tick");

    let mut next = self.cells.clone();

    for row in 0..self.height {
      for col in 0..self.width {
        let idx = self.get_index(row, col);
        let cell = self.cells[idx];
        let live_neighbors = self.live_neighbor_count(row, col);

        let next_cell = match (cell, live_neighbors) {
          (Cell::Alive, x) if x < 2 => Cell::Dead,
          (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
          (Cell::Alive, x) if x > 3 => Cell::Dead,
          (Cell::Dead, 3) => Cell::Alive,
          (otherwise, _) => otherwise,
        };

        next[idx] = next_cell;
      }
    }

    self.cells = next;
  }

  pub fn new() -> Universe {
    utils::set_panic_hook();

    let width = 128;
    let height = 128;

    log!("Initializing universe");

    let cells = (0..width * height)
      .map(|i| {
        if js_sys::Math::random() < 0.5 {
          Cell::Alive
        } else {
          Cell::Dead
        }
      }).collect();

    Universe {
      width,
      height,
      cells,
    }
  }

  pub fn render(&self) -> String {
    self.to_string()
  }

  pub fn width(&self) -> u32 {
    self.width
  }

  pub fn height(&self) -> u32 {
    self.height
  }

  pub fn cells(&self) -> *const Cell {
    self.cells.as_ptr()
  }

  pub fn toggle_cell(&mut self, row: u32, column: u32) {
    let idx = self.get_index(row, column);
    self.cells[idx].toggle();
  }
}
