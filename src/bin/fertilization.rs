use generative::raylib;

use rand::prelude::*;
use std::ffi::CString;

const ROCKETS: usize = 10000;
const MUTANTS: usize = 100;
const SCREEN_WIDTH: i32 = 1200;
const SCREEN_HEIGHT: i32 = 860;
const ROCKET_WIDTH: f32 = 2.0;
const ROCKET_HEIGHT: f32 = 20.0;
const TITLE: &str = "Fertilization";
const SPEED: f32 = 3.0;
const ROUNDS: usize = 700;

const WALL_A: raylib::Rectangle = raylib::Rectangle {
    x: (SCREEN_WIDTH / 3) as f32,
    y: (SCREEN_HEIGHT / 3) as f32,
    width: 20.0,
    height: (SCREEN_HEIGHT / 3 * 2) as f32,
};

const WALL_B: raylib::Rectangle = raylib::Rectangle {
    x: (SCREEN_WIDTH / 3 * 2) as f32,
    y: 0.0,
    width: 20.0,
    height: (SCREEN_HEIGHT / 3 * 2) as f32,
};

struct Gene {
    pos: raylib::Rectangle,
    genes: Vec<f32>,
    fitness: f32,
    winner: bool,
    steps: usize,
    dead: bool,
    selected: bool,
    cum_fitness: f32,
    id: usize,
}

impl Gene {
    pub fn new(id: usize) -> Self {
        let mut rng = rand::rng();
        let mut angle: i32 = rng.random_range(0..=360);
        let mut angles = Vec::new();
        angles.push(angle as f32);
        for _ in 0..ROUNDS {
            let delta_angle: i32 = rng.random_range(-10..=10);
            angle += delta_angle;
            angles.push(angle as f32);
        }
        unsafe {
            Self {
                pos: raylib::Rectangle::new(
                    20.0,
                    (raylib::GetScreenHeight() as f32) / 2.0 - 50.0,
                    ROCKET_WIDTH,
                    ROCKET_HEIGHT,
                ),
                genes: angles,
                fitness: 0.0,
                winner: false,
                steps: 0,
                dead: false,
                selected: false,
                cum_fitness: 0.0,
                id,
            }
        }
    }

    pub fn pair(id: usize, gene_a: &Gene, gene_b: &Gene) -> Self {
        let mut angles = Vec::new();
        for i in 0..ROUNDS {
            // if i < ROUNDS / 2 {
            if i % 2 == 0 {
                angles.push(gene_a.genes[i]);
            } else {
                angles.push(gene_b.genes[i]);
            }
        }
        unsafe {
            Self {
                pos: raylib::Rectangle::new(
                    20.0,
                    (raylib::GetScreenHeight() as f32) / 2.0 - 50.0,
                    ROCKET_WIDTH,
                    ROCKET_HEIGHT,
                ),
                genes: angles,
                fitness: 0.0,
                winner: false,
                steps: 0,
                dead: false,
                selected: false,
                cum_fitness: 0.0,
                id,
            }
        }
    }

    pub fn angle(&self, round: usize) -> f32 {
        self.genes[round]
    }

    pub fn update(&mut self, round: usize, target: &raylib::Vector2) {
        unsafe {
            if self.winner {
                return;
            }
            let angle = self.genes[round];
            let movement = raylib::Vector2::new(
                angle.to_radians().cos() * SPEED,
                angle.to_radians().sin() * SPEED,
            );
            self.pos.x += movement.x;
            self.pos.y += movement.y;

            if raylib::CheckCollisionRecs(WALL_A, self.pos)
                || raylib::CheckCollisionRecs(WALL_B, self.pos)
            {
                self.pos.x -= movement.x;
                self.pos.y -= movement.y;
            }
            if self.pos.x < 0.0 || self.pos.y < 0.0 || self.pos.y > (SCREEN_HEIGHT as f32) {
                self.dead = true;
            }
            let distance = 10.0;
            if self.pos.x > target.x - distance
                && self.pos.x < target.x + distance
                && self.pos.y > target.y - distance
                && self.pos.y < target.y + distance
            {
                self.winner = true;
                self.steps = round;
            }
        }
    }

    pub fn distance(&mut self, target: raylib::Vector2) {
        unsafe {
            let steps = if self.winner { self.steps } else { ROUNDS };
            self.fitness = 1.0
                / (raylib::Vector2Distance(target, raylib::Vector2::new(self.pos.x, self.pos.y))
                    / steps as f32)
                    .powf(2.0)
        }
    }
}

struct Population {
    pub genes: Vec<Gene>,
    pub target: raylib::Vector2,
    pub winners: i32,
}

impl Population {
    pub fn new(target: raylib::Vector2) -> Self {
        let mut genes = Vec::new();
        for i in 0..ROCKETS {
            genes.push(Gene::new(i))
        }
        Self {
            genes,
            target,
            winners: 0,
        }
    }

    pub fn draw(&self, round: usize) {
        unsafe {
            for gene in &self.genes {
                if !gene.dead {
                    raylib::DrawRectanglePro(
                        gene.pos,
                        raylib::Vector2::new(0.0, 0.0),
                        (gene.angle(round) + 90.0) % 360.0,
                        if gene.winner {
                            raylib::GREEN
                        } else if gene.selected {
                            raylib::BLACK
                        } else {
                            raylib::RED
                        },
                    );
                }
            }
        }
    }

    pub fn step(&mut self, round: usize) {
        self.winners = 0;
        for gene in &mut self.genes {
            if !gene.dead {
                gene.update(round, &self.target);
                if gene.winner {
                    self.winners += 1;
                }
            }
        }
    }

    pub fn fitness(&mut self) {
        for gene in &mut self.genes {
            gene.distance(self.target);
        }
    }

    pub fn mutate(&mut self) {
        let mut rng = rand::rng();
        for _ in 0..MUTANTS {
            let index: usize = rng.random_range(0..ROCKETS);
            let angle_index: usize = rng.random_range(0..ROUNDS - ROUNDS / 2);
            for i in 1..ROUNDS / 2 {
                let delta_angle: i32 = rng.random_range(-10..=10);
                self.genes[index].genes[angle_index + i] += delta_angle as f32;
            }
        }
    }

    pub fn generate(&mut self) {
        let mut genes: Vec<&mut Gene> = self.genes.iter_mut().filter(|item| !item.dead).collect();
        let sum_fitness: f32 = genes.iter().map(|item| item.fitness).sum();
        let mut cum_fitness = 0.0;
        for gene in &mut genes {
            cum_fitness += gene.fitness / sum_fitness;
            gene.cum_fitness = cum_fitness;
        }
        let mut new_genes = Vec::new();
        let mut rng = rand::rng();
        for i in 0..ROCKETS {
            let number_a = rng.random_range(0.0..=cum_fitness);
            let gene_a = genes
                .iter()
                .find(|item| item.cum_fitness > number_a)
                .unwrap();
            let id_a = gene_a.id;
            let number_b = rng.random_range(0.0..=cum_fitness);
            let gene_b = genes
                .iter()
                .find(|item| item.cum_fitness > number_b)
                .unwrap();
            let id_b = gene_b.id;
            new_genes.push(Gene::pair(i, gene_a, gene_b));
            for gene in &mut genes {
                if gene.id == id_a || gene.id == id_b {
                    gene.selected = true
                }
            }
        }
        self.genes = new_genes;
    }

    pub fn winners(&self) -> i32 {
        self.winners
    }
}

fn main() {
    unsafe {
        raylib::InitWindow(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            CString::new(TITLE).expect("cstr").as_ptr(),
        );

        let screen_width = raylib::GetScreenWidth() as f32;
        let screen_height = raylib::GetScreenHeight() as f32;

        let target = raylib::Vector2::new(screen_width - 10.0, screen_height / 2.0);
        let mut population = Population::new(target);

        raylib::SetTargetFPS(60);

        let mut loops = 0;
        let mut round = 0;

        // Main game loop
        while !raylib::WindowShouldClose() {
            raylib::BeginDrawing();
            raylib::ClearBackground(raylib::BLACK);

            let winners = population.winners();
            raylib::DrawText(
                CString::new(std::format!(
                    "Round: {}, Steps: {}, Winners: {}",
                    loops,
                    round,
                    winners
                ))
                .expect("cstr")
                .as_ptr(),
                10,
                10,
                20,
                raylib::WHITE,
            );
            raylib::DrawRectangleRec(WALL_A, raylib::WHITE);
            raylib::DrawRectangleRec(WALL_B, raylib::WHITE);
            raylib::DrawCircleV(target, 4.0, raylib::GREEN);
            population.draw(round);
            raylib::EndDrawing();

            if round < ROUNDS - 1 {
                population.step(round);
                round += 1;
            } else {
                population.fitness();
                population.generate();
                population.mutate();
                round = 0;
                loops += 1;
            }
        }
        raylib::CloseWindow();
    }
}
