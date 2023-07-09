use std::collections::HashMap;

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

impl Cell {
    fn empty() -> Self {
        Self {
            class: CellClass::Empty,
            properties: CellProperties::NONE,
        }
    }

    fn sand() -> Self {
        Self {
            class: CellClass::Sand,
            properties: CellProperties::MOVEDOWN | CellProperties::MOVEDOWNSIDE,
        }
    }

    fn water() -> Self {
        Self {
            class: CellClass::Water,
            properties: CellProperties::MOVEDOWN
                | CellProperties::MOVESIDE
                | CellProperties::MOVEDOWNSIDE,
        }
    }
}

pub struct SandWorld {
    brush: Cell,
    width: usize,
    height: usize,
    scale: usize,
    cells: Vec<Cell>,
    changes: HashMap<usize, Vec<usize>>,
}

impl SandWorld {
    pub fn new(width: usize, height: usize, scale: usize) -> Self {
        let mut cells = vec![Cell::empty(); width * height];

        cells[0] = Cell {
            class: CellClass::Sand,
            properties: CellProperties::MOVEDOWN,
        };
        let changes = HashMap::new();
        let brush = Cell::sand();
        Self {
            brush,
            width,
            height,
            scale,
            cells,
            changes,
        }
    }

    fn get_index_by_pos(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn get_cell_by_index(&self, index: usize) -> &Cell {
        &self.cells[index]
    }

    fn get_cell_by_pos(&self, x: usize, y: usize) -> &Cell {
        &self.cells[y * self.width + x]
    }

    fn set_sell_by_pos(&mut self, x: usize, y: usize, cell: Cell) {
        let index = self.get_index_by_pos(x, y);
        self.cells[index] = cell;
    }

    fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    fn is_empty(&self, x: usize, y: usize) -> bool {
        if !self.in_bounds(x, y) {
            return false;
        }
        let cell = self.get_cell_by_pos(x, y);
        cell.class == CellClass::Empty
    }

    // add a move to the changes hashmap
    fn move_cell(&mut self, x: usize, y: usize, xto: usize, yto: usize) {
        let index = self.get_index_by_pos(x, y);
        let index_to = self.get_index_by_pos(xto, yto);
        let possible_sources = self.changes.entry(index_to).or_insert(Vec::new());
        possible_sources.push(index);
    }

    fn move_down(&mut self, x: usize, y: usize) -> bool {
        let down = self.is_empty(x, y + 1);

        if down {
            self.move_cell(x, y, x, y + 1);
        }

        down
    }

    fn move_side(&mut self, x: usize, y: usize) -> bool {
        let mut left = x > 0 && self.is_empty(x - 1, y);
        let mut right = self.is_empty(x + 1, y);

        if left && right {
            left = rand::gen_range(0, 2) == 0;
            right = !left;
        }

        if left {
            self.move_cell(x, y, x - 1, y);
        } else if right {
            self.move_cell(x, y, x + 1, y);
        }

        left || right
    }

    fn move_down_side(&mut self, x: usize, y: usize) -> bool {
        let mut down_left = x > 0 && self.is_empty(x - 1, y + 1);
        let mut down_right = self.is_empty(x + 1, y + 1);

        if down_left && down_right {
            down_left = rand::gen_range(0, 2) == 0;
            down_right = !down_left;
        }

        if down_left {
            self.move_cell(x, y, x - 1, y + 1);
        } else if down_right {
            self.move_cell(x, y, x + 1, y + 1);
        }

        down_left || down_right
    }

    pub fn commit_cells(&mut self) {
        for (destination, possible_sources) in self.changes.iter() {
            // pick one of the possible sources
            let source = possible_sources[rand::gen_range(0, possible_sources.len())];
            self.cells[*destination] = self.cells[source];
            self.cells[source] = Cell::empty();
        }
        self.changes.clear();
    }

    pub fn update(&mut self) {
        if is_key_down(KeyCode::W) {
            self.brush = Cell::water();
        } else if is_key_down(KeyCode::S) {
            self.brush = Cell::sand();
        } else if is_key_down(KeyCode::E) {
            self.brush = Cell::empty();
        }

        if is_mouse_button_down(MouseButton::Left) {
            match self.brush.class {
                CellClass::Sand => {
                    let coords = mouse_position();
                    let x = (coords.0 / self.scale as f32) as usize;
                    let y = (coords.1 / self.scale as f32) as usize;

                    if self.in_bounds(x, y) {
                        self.set_sell_by_pos(x, y, Cell::sand());
                    }
                }
                CellClass::Water => {
                    let coords = mouse_position();
                    let x = (coords.0 / self.scale as f32) as usize;
                    let y = (coords.1 / self.scale as f32) as usize;

                    if self.in_bounds(x, y) {
                        self.set_sell_by_pos(x, y, Cell::water());
                    }
                }
                CellClass::Empty => {
                    let coords = mouse_position();
                    let x = (coords.0 / self.scale as f32) as usize;
                    let y = (coords.1 / self.scale as f32) as usize;

                    if self.in_bounds(x, y) {
                        self.set_sell_by_pos(x, y, Cell::empty());
                    }
                }

                _ => {}
            }
        }

        for x in 0..self.width {
            for y in 0..self.height {
                let cell = self.get_cell_by_pos(x, y);
                let properties = cell.properties;

                if (properties & CellProperties::MOVEDOWN) != CellProperties::NONE
                    && self.move_down(x, y)
                {
                } else if (properties & CellProperties::MOVEDOWNSIDE) != CellProperties::NONE
                    && self.move_down_side(x, y)
                {
                } else if (properties & CellProperties::MOVESIDE) != CellProperties::NONE
                    && self.move_side(x, y)
                {
                }
            }
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

    pub fn brush(&self) -> &str {
        match self.brush.class {
            CellClass::Empty => "Empty",
            CellClass::Sand => "Sand",
            CellClass::Water => "Water",
            CellClass::Rock => "Rock",
        }
    }
}
