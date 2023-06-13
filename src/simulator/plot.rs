/// Plot sweep results
use plotters::prelude::*;

pub struct Plotter {
	data: Vec<u32>,
	buckets: usize,
}

impl Plotter {
	pub fn new(buckets: usize) -> Plotter {
		Plotter {
			data: Vec::new(),
			buckets
		}
	}

	pub fn add_sample(&mut self, config: &Vec<i32>) {
		// Compute number representation
		let number = self.config_to_num(config);
		self.data.push(number);
	}

	pub fn plot(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
		// Setup plot
		let path = String::from("out/") + &name;
		let root = BitMapBackend::new(&path, (1024, 768)).into_drawing_area();
		root.fill(&WHITE)?;

		let mut chart = ChartBuilder::on(&root)
		.x_label_area_size(35)
		.y_label_area_size(40)
		.margin(5)
		.caption("MCMC Sweep", ("sans-serif", 50.0))
		.build_cartesian_2d((0u32..(self.buckets + 1) as u32).into_segmented(), 0u32..3000u32)?;

		chart
			.configure_mesh()
			.disable_x_mesh()
			.bold_line_style(&WHITE.mix(0.3))
			.y_desc("Frequency")
			.x_desc("Spin State")
			.axis_desc_style(("sans-serif", 15))
			.draw()?;

		chart.draw_series(
			Histogram::vertical(&chart)
				.style(BLUE.mix(0.5).filled())
				.data(self.data.iter().map(|x: &u32| (*x, 1))),
		).unwrap();

		root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
		Ok(())
	}

	fn spin_to_bit(&self, spin: i32) -> u32 {
		if spin == 1 {
			1
		} else {
			0
		}
	}

	fn config_to_num(&self, c: &Vec<i32>) -> u32 {
		let mut power = 1;
		let mut number = 0;

		for val in c.iter().rev() {
			number += self.spin_to_bit(*val) * power;
			power <<= 1;
		}

		number
	}
}