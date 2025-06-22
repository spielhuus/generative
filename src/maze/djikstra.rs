use super::Board;

#[derive(Default, Clone, Copy, Debug)]
pub struct Weight {
    pub x: usize,
    pub y: usize,
    pub weight: usize,
}

pub struct Solver {
    start: (usize, usize),
    end: (usize, usize),
    board_size: usize,
    positions: Vec<usize>,
    pub path: Vec<usize>,
    pub weights: Vec<Option<Weight>>,
    pub reached_end: bool,
    pub solved: bool,
}

impl Solver {
    pub fn new(start: (usize, usize), end: (usize, usize), board_size: usize) -> Self {
        let mut weights = vec![None; board_size * board_size];
        weights[0] = Some(Weight {
            x: 1,
            y: 1,
            weight: 1,
        });
        Self {
            start,
            end,
            board_size,
            positions: vec![0],
            path: vec![],
            weights,
            reached_end: false,
            solved: false,
        }
    }

    pub fn neighbours(&self, board: &Board, index: usize) -> Vec<Option<usize>> {
        let mut res = Vec::<Option<usize>>::new();
        if board.cells[index].y > 1 {
            res.push(Some(index - 1));
        } else {
            res.push(None);
        }
        if board.cells[index].y < self.board_size as i32 - 1 {
            res.push(Some(index + 1));
        } else {
            res.push(None);
        }
        if board.cells[index].x > 1 {
            res.push(Some(index - self.board_size + 1));
        } else {
            res.push(None);
        }
        if board.cells[index].x < self.board_size as i32 - 1 {
            res.push(Some(index + self.board_size - 1));
        } else {
            res.push(None);
        }
        res
    }

    pub fn step(&mut self, board: &Board) {
        let mut next_cells: Vec<usize> = vec![];
        for index in &self.positions {
            let weight = self.weights[*index].unwrap();
            let neighbours = self.neighbours(board, *index);
            let free: Vec<(usize, &Option<usize>)> = neighbours
                .iter()
                .enumerate()
                .filter(|&(d, i)| {
                    if (i.is_some() && self.weights[i.unwrap()].is_none())
                        && ((d == 0 && !board.cells[*index].walls.top)
                            || (d == 1 && !board.cells[*index].walls.bottom)
                            || (d == 2 && !board.cells[*index].walls.left)
                            || (d == 3 && !board.cells[*index].walls.right))
                    {
                        return true;
                    }
                    false
                })
                .collect();

            if !free.is_empty() {
                for (_, j) in free {
                    self.weights[j.unwrap()] = Some(Weight {
                        x: board.cells[j.unwrap()].x as usize,
                        y: board.cells[j.unwrap()].y as usize,
                        weight: weight.weight + 1,
                    });
                    next_cells.push(j.unwrap());
                    if self.weights[j.unwrap()].unwrap().x == self.end.0
                        && self.weights[j.unwrap()].unwrap().y == self.end.1
                    {
                        self.reached_end = true;
                        self.path.push(j.unwrap())
                    }
                }
            }
        }
        self.positions = next_cells;
    }

    pub fn path(&mut self, board: &Board) {
        let index: usize = *self.path.last().unwrap();
        let neighbours = self.neighbours(board, index);
        let mut free: Vec<(usize, &Option<usize>)> = neighbours
            .iter()
            .enumerate()
            .filter(|&(d, i)| {
                if (i.is_some()
                    && !self.path.contains(&i.unwrap())
                    && self.weights[i.unwrap()].is_some())
                    && ((d == 0 && !board.cells[index].walls.top)
                        || (d == 1 && !board.cells[index].walls.bottom)
                        || (d == 2 && !board.cells[index].walls.left)
                        || (d == 3 && !board.cells[index].walls.right))
                {
                    return true;
                }
                false
            })
            .collect();
        free.sort_by(|a, b| {
            self.weights[a.1.unwrap()]
                .unwrap()
                .weight
                .cmp(&self.weights[b.1.unwrap()].unwrap().weight)
        });
        let next = free.first().unwrap();
        self.path.push(next.1.unwrap());

        if self.weights[next.1.unwrap()].unwrap().x == self.start.0
            && self.weights[next.1.unwrap()].unwrap().y == self.start.1
        {
            self.solved = true;
        }
    }
}
