use ndarray::{Array1, Array};


#[derive(Debug)]
struct Neuron {
    weights: Array1<f64>
}

impl Neuron {
    pub fn new(size: usize) -> Self {
        let weights = Array::from_iter(
            (0..size).map(|_| rand::random::<f64>())
        );
        Neuron {weights}
    }

    pub fn activate(&self, inputs: &[f64]) -> f64 {
        let weighted_sum: f64 = self.weights.iter().zip(inputs.iter()).map(|(&w, &i)| w * i).sum();
        1.0 / (1.0 + (-weighted_sum)).exp()
    }
}

pub struct NeuralNetwork<const SIZE: usize> {
    input: [Neuron; SIZE],
    hidden: [Neuron; SIZE],
    output: [Neuron; SIZE]
}

impl<const SIZE: usize> NeuralNetwork<SIZE> {

    pub fn new<const WEIGHTS: usize> () -> Self {
        NeuralNetwork {
            input: (0..SIZE).map(|_| Neuron::new(WEIGHTS)).collect::<Vec<Neuron>>().try_into().unwrap(),
            hidden: (0..SIZE).map(|_| Neuron::new(WEIGHTS)).collect::<Vec<Neuron>>().try_into().unwrap(),
            output: (0..SIZE).map(|_| Neuron::new(WEIGHTS)).collect::<Vec<Neuron>>().try_into().unwrap(),
        }
    }

    pub fn forward(&self, inputs: &[f64]) -> [f64; SIZE] {
        let mut processed = [0.0; SIZE];
        let mut processed_cpy = [0.0; SIZE];
    
        // Process input layer
        for (i, neuron) in self.input.iter().enumerate() {
            processed[i] = neuron.activate(inputs);
        }
    
        // Process hidden layer using input layer output
        for (i, neuron) in self.hidden.iter().enumerate() {
            processed_cpy[i] = processed[i];
            processed[i] = neuron.activate(&processed_cpy);
        }
    
        // Process output layer using hidden layer output
        for (i, neuron) in self.output.iter().enumerate() {
            processed_cpy[i] = processed[i];
            processed[i] = neuron.activate(&processed_cpy);
        }

        processed
    }
}