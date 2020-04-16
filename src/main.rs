use std::fmt;

#[derive(Copy, Clone)]
enum CellState {
    Alive,
    Dead,
}

struct Grid {
    width: usize,
    height: usize,
    data: Vec<CellState>,
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CellState::Alive => write!(f, "1"),
            CellState::Dead => write!(f, "0"),
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{} ", self.get(x, y))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        Grid {
            width,
            height,
            data: vec![CellState::Dead; width * height],
        }
    }

    fn get(&self, x: usize, y: usize) -> CellState {
        self.data[y * self.width + x]
    }

    fn set(&mut self, x: usize, y: usize, state: CellState) {
        self.data[y * self.width + x] = state;
    }

    // fn update(&mut self, rules: impl Fn())
}

fn main() {
    let grid = Grid::new(10, 10);
    println!("{}", grid);
}
