use crate::circuit::Circuit;
use crate::simulator::Plotter;
use rand::{Rng, thread_rng};
use rand::distributions::Uniform;

pub struct MCMCConfig {
	pub warmup: usize,
	pub sweeps: usize,
	pub plot: bool,
	pub beta: f64,
}

pub struct MCMC {
	pub circuit: Circuit,
	pub config: MCMCConfig,
	pub activations: Vec<i32>,
}

impl MCMC {
	pub fn new(circuit: Circuit) -> MCMC {
		MCMC { 
			circuit, 
			config: MCMCConfig {
				warmup: 20,
				sweeps: 10000,
				plot: true,
				beta: 1.0
			},
			activations: Vec::new()
		}
	}

	pub fn from_config(circuit: Circuit, config: MCMCConfig) -> MCMC {
		MCMC { circuit, config, activations: Vec::new() }
	}

	pub fn run(&mut self) {
		// Initialize activations and plotter
		self.activations = vec![0; self.circuit.weight.shape().0];
		let mut plotter = Plotter::new(1 << (self.activations.len() - 1));

		for sweep in 0..self.config.sweeps {

			// One sweep is N updates
			let sweep_size = self.activations.len();
			for _ in 0..sweep_size {
				// Update each pbit in order
				for i in 0..sweep_size {
					// Update the gate synapse
					let synapse = self.synapse(i);
					let activation = self.activation(synapse);
					self.activations[i] = activation;
				}
			}

			// Collect some information about configuration energy
			if sweep > self.config.warmup {
				let _energy_sample = self.energy();
	
				plotter.add_sample(&self.activations);
			}
		}

		if self.config.plot {
			plotter.plot("mcmc.png").unwrap();
		}
	}

	fn synapse(&self, pbit_index: usize) -> i32 {
		let mut synapse = 0;
		let weight = self.circuit.weight.clone();
		let bias = self.circuit.bias.clone();

		for j in 0..self.activations.len() {
			if pbit_index != j {
				// Update the synapse
				synapse += weight[(pbit_index, j)] * self.activations[j];
			}
		}

		synapse + bias[pbit_index]
	}

	fn activation(&self, synapse: i32) -> i32 {
		let mut rng = thread_rng();
		let activation_dist = Uniform::new(-1.0, 1.0);

		let mut raw_activ = (self.config.beta * synapse as f64).tanh();
		raw_activ -= rng.sample(activation_dist);

		if raw_activ > 0.0 {
			1
		} else {
			-1
		}
	}

	fn energy(&self) -> f64 {
		let weight = self.circuit.weight.clone();
		let bias = self.circuit.bias.clone();

		let mut energy = 0.0;
		for j in 0..self.activations.len() {
			for i in 0..j {
				energy += (weight[(i, j)] * self.activations[i] * self.activations[j]) as f64;
			}
		}

		for i in 0..self.activations.len() {
			energy += (bias[i] * self.activations[i]) as f64;
		}

		-energy
	}
}