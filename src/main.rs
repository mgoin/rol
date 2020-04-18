use std::fmt;

#[derive(Copy, Clone, PartialEq)]
enum CellState {
    Dead,
    Alive,
}

#[derive(Clone)]
struct Grid {
    width: usize,
    height: usize,
    step: usize,
    data: Vec<CellState>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        Grid {
            width,
            height,
            step: 0,
            data: vec![CellState::Dead; width * height],
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<CellState> {
        if x < self.width && y < self.height {
            Some(self.data[y * self.width + x])
        } else {
            None
        }
    }

    fn set(&mut self, x: usize, y: usize, state: CellState) {
        self.data[y * self.width + x] = state;
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<(CellState, usize, usize)> {
        let mut v: Vec<(CellState, usize, usize)> = Vec::new();
        for i in -1i32..=1 {
            for j in -1i32..=1 {
                if i != 0 || j != 0 {
                    let new_x = (x as i32 + j) as usize;
                    let new_y = (y as i32 + i) as usize;
                    if let Some(cs) = self.get(new_x, new_y) {
                        v.push((cs, new_x, new_y));
                    }
                }
            }
        }
        v
    }

    fn update(&mut self) {
        // Make a copy of current grid to use for rules
        let g = self.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let is_alive = match g.get(x, y) {
                    Some(cs) => cs == CellState::Alive,
                    None => false,
                };

                let alive_neighbors = g
                    .neighbors(x, y)
                    .iter()
                    .filter(|&n| n.0 == CellState::Alive)
                    .count();

                match (is_alive, alive_neighbors) {
                    // Birth: Any dead cell with three live neighbors becomes a live cell
                    (false, 3) => self.set(x, y, CellState::Alive),
                    // Survival: Any live cell with two or three live neighbors survives
                    (true, 2..=3) => self.set(x, y, CellState::Alive),
                    // All other live cells die, all other dead cells stay dead
                    (true, _) => self.set(x, y, CellState::Dead),
                    _ => {}
                }
            }
        }
        self.step += 1;
    }
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CellState::Dead => write!(f, "."),
            CellState::Alive => write!(f, "*"),
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get(x, y) {
                    Some(cs) => write!(f, "{} ", cs)?,
                    None => write!(f, "~ ")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let mut grid = Grid::new(5, 5);
    grid.set(0, 1, CellState::Alive);
    grid.set(1, 2, CellState::Alive);
    grid.set(2, 0, CellState::Alive);
    grid.set(2, 1, CellState::Alive);
    grid.set(2, 2, CellState::Alive);
    println!("{}", grid);

    for _ in 0..6 {
        grid.update();
        println!("{}", grid);
    }
}
