extern crate nalgebra as na;
pub mod gate;
pub mod simulator;
pub mod pbit;
pub mod circuit;

#[cfg(test)]
mod tests {
    use gate::{Copy, Or, And};
    use pbit::GridPbit;
    use circuit::Circuit;
    use simulator::MCMC;

    use super::*;

    #[test]
    fn it_runs_copy_gate() {
        let p0 = GridPbit::new(0, 0);
        let p1 = GridPbit::new(0, 1);

        let cp0 = Copy::new(p0, p1);

        let mut circuit = Circuit::from_vector(vec![Box::new(cp0)]);
        circuit.compile();

        let mut sim = MCMC::new(circuit);
        sim.run();
    }

    #[test]
    fn it_runs_and_gate() {
        let p0 = GridPbit::new(0, 0);
        let p1 = GridPbit::new(0, 1);
        let p2 = GridPbit::new(1, 0);

        let and0 = And::new(p0, p1, p2);

        let mut circuit = Circuit::from_vector(vec![Box::new(and0)]);
        circuit.compile();

        let mut sim = MCMC::new(circuit);
        sim.run();
    }

    #[test]
    fn it_runs_or_gate() {
        let p0 = GridPbit::new(0, 0);
        let p1 = GridPbit::new(0, 1);
        let p2 = GridPbit::new(1, 0);

        let and0 = Or::new(p0, p1, p2);

        let mut circuit = Circuit::from_vector(vec![Box::new(and0)]);
        circuit.compile();

        let mut sim = MCMC::new(circuit);
        sim.run();
    }

    #[test]
    fn it_runs_composed_circuit() {
        let p1 = GridPbit::new(0, 0);
        let p2 = GridPbit::new(0, 1);
        let p3 = GridPbit::new(1, 0);
        let p4 = GridPbit::new(2, 0);
        let p5 = GridPbit::new(2, 1);

        // Constructo circuit and compose
        let mut circuit = Circuit::new();
        circuit.append(Box::new(And::new(p1, p2, p3)));
        circuit.append(Box::new(Or::new(p3, p4, p5)));
        circuit.compile();

        let mut sim = MCMC::new(circuit);
        sim.run();
    }
}
