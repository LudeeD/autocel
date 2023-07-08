use bitflags::bitflags;
use macroquad::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum CellClass {
    Empty,
    Sand,
    Water,
    Rock,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct CellProperties: u32 {
        const NONE          = 0b00000000;
        const MOVEDOWN      = 0b00000001;
        const MOVEDOWNSIDE  = 0b00000010;
        const MOVESIDE      = 0b00000100;
    }
}

#[derive(Clone, Copy)]
struct Cell {
    class: CellClass,
    properties: CellProperties,
}

pub struct SandWorld {
    width: usize,
    height: usize,
    scale: usize,
    cells: Vec<Cell>,
}

impl SandWorld {
    pub fn new(width: usize, height: usize, scale: usize) -> Self {
        let cells = vec![
            Cell {
                class: CellClass::Empty,
                properties: CellProperties::NONE,
            };
            width * height
        ];
        Self {
            width,
            height,
            scale,
            cells,
        }
    }

    pub fn update(&mut self) {
        // choose random cell to alive
        let random_index = rand::gen_range(0, self.cells.len());
        let random_type = rand::gen_range(0, 3);
        match random_type {
            0 => self.cells[random_index].class = CellClass::Rock,
            1 => self.cells[random_index].class = CellClass::Sand,
            2 => self.cells[random_index].class = CellClass::Water,
            _ => (),
        }
    }

    pub fn draw(&self) {
        for (i, cell) in self.cells.iter().enumerate() {
            let x = (i % self.width) as f32 * self.scale as f32;
            let y = (i / self.width) as f32 * self.scale as f32;
            match cell.class {
                CellClass::Empty => (),
                CellClass::Sand => {
                    draw_rectangle(x, y, self.scale as f32, self.scale as f32, YELLOW)
                }
                CellClass::Water => {
                    draw_rectangle(x, y, self.scale as f32, self.scale as f32, BLUE)
                }
                CellClass::Rock => draw_rectangle(x, y, self.scale as f32, self.scale as f32, GRAY),
            }
        }
    }
}
