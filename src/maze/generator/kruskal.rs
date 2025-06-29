use disjoint::DisjointSet;
use rand::prelude::*;

use crate::maze::{Board, Generator, State};

#[derive(Debug, Eq, PartialEq)]
enum Direction {
    North,
    West,
}

#[derive(Debug)]
struct Edge {
    x: i32,
    y: i32,
    direction: Direction,
}

pub struct Kruskal {
    edges: Vec<Edge>,
    cells: Vec<(i32, i32, i32)>,
    merged: DisjointSet,
    visited_edges: Vec<Edge>,
    step: i32,
}

impl Kruskal {
    pub fn new(board: &Board) -> Self {
        let mut rng = rand::rng();
        // pupulate the edges
        let mut edges: Vec<Edge> = vec![];
        for y in 0..board.board_size {
            for x in 0..board.board_size {
                if y > 0 {
                    edges.push(Edge {
                        x,
                        y,
                        direction: Direction::North,
                    })
                }
                if x > 0 {
                    edges.push(Edge {
                        x,
                        y,
                        direction: Direction::West,
                    })
                }
            }
        }
        edges.shuffle(&mut rng);

        Self {
            edges,
            cells: Vec::new(),
            merged: DisjointSet::with_len(board.cells.len()),
            visited_edges: Vec::new(),
            step: 1,
        }
    }
}

impl Generator for Kruskal {
    fn step(&mut self, board: &mut Board) -> State {
        let edge: Option<Edge> = self.edges.pop();
        if let Some(edge) = edge {
            let index_cell = board.get_index(edge.x, edge.y);
            let index_neighbour = if edge.direction == Direction::North {
                board.get_index(edge.x, edge.y - 1)
            } else {
                board.get_index(edge.x - 1, edge.y)
            };

            if !self
                .merged
                .is_joined(index_cell as usize, index_neighbour as usize)
            {
                self.merged
                    .join(index_cell as usize, index_neighbour as usize);
                self.cells.push((self.step, index_cell, index_neighbour));

                //remove walls
                match edge.direction {
                    Direction::North => {
                        board.cells[index_cell as usize].walls.top = false;
                        board.cells[index_neighbour as usize].walls.bottom = false;
                    }
                    Direction::West => {
                        board.cells[index_cell as usize].walls.left = false;
                        board.cells[index_neighbour as usize].walls.right = false;
                    }
                }
            }
            self.visited_edges.push(edge);
        } else {
            return State::GenerationDone;
        }

        self.step += 1;
        State::Generate
    }
}
