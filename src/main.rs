use network::NeuralNetwork;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::any::Any;
mod network;
use std::rc::Rc;

#[derive(Hash)]
pub enum Ground {
    Soil,
    Toxic,
    Empty
}

#[derive(Hash)]
pub enum Block<E : Entity<E>> {
    Empty(Ground),
    Occupied(Ground,E )
}

impl<E: Entity<E>> Block<E> {
    fn to_f64(&self) -> f64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        let hash_value = hasher.finish();
        hash_value as f64
    }
}

pub trait Entity<E: Entity<E>>: Any + Hash {
    fn update(&self, soroundings: &[&[Block<E>]]) -> Action;
}
trait Reporducable {
    fn reproduce() -> Self;
}

struct Organism {
    network: Rc<NeuralNetwork<100>>,
    max_health: u16,
    health: u16,
    energy: u16
}

impl Hash for Organism {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.max_health.hash(state);
        self.health.hash(state);
        self.energy.hash(state);
    }
}

impl Organism {
    fn new(network: Rc<NeuralNetwork<100>>, max_health: u16, health: u16, energy: u16) -> Self {
         Self { network, max_health, health, energy } 
    }
}

#[derive(Copy, Clone)]
pub enum Direction {
    Forward,
    Backward,
    Left,
    Right,
    Rest
}

pub enum Action {
    Move(Direction),

}

const DIRECTIONS: [Direction; 5] = [
    Direction::Forward,
    Direction::Backward,
    Direction::Left,
    Direction::Right,
    Direction::Rest
];

fn encode_soroundings(soroundings: &[&[Block<Organism>]]) -> Vec<f64> {
    soroundings
    .iter()
    .map(|it| it
        .iter()
        .map(|inner| inner.to_f64())
        .collect::<Vec<f64>>()
    )
    .flatten()
    .collect()
}

fn find_largest_index(numbers: &[f64]) -> usize {
    numbers
        .iter()
        .enumerate()
        .max_by(|(_, &a), (_, &b)| a.partial_cmp(&b).unwrap())
        .map(|(index, _)| index)
        .unwrap_or(0)
}

fn parse_request(data: &[f64]) -> Action {
    
}

impl Entity<Organism> for Organism {

    fn update(&self, soroundings: &[&[Block<Organism>]]) -> Action {
        //convert suroundings to 1d array of floats
        let processed_data: Vec<f64> = encode_soroundings(soroundings);

        //neural network magic
        let output: [f64; 100] = self.network.forward(&processed_data);

        //convert action back
        let direction = DIRECTIONS[find_largest_index(&output)];
        
        Action::Move(direction)
    }
}

impl Reporducable for Organism {
    fn reproduce() -> Self {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}
