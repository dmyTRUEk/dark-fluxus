//! stock market

use rand::{RngExt, rngs::ThreadRng};

use crate::match_random_weighted;



pub struct StockMarket {
	pub stocks: Vec<Stock>,
	// TODO: other/ai market players (buyers/sellers)
}
impl StockMarket {
	pub fn new() -> Self {
		StockMarket {
			stocks: vec![
				Stock::new( // void/dark
					['D', 'K', 'V', 'D'],
					0.,
				),
				Stock::new( // life/living
					['L', 'V', 'N', 'G'],
					1_000.,
				),
				Stock::new( // space
					['S', 'P', 'C', 'E'],
					10_000.,
				),
				Stock::new( // matter
					['M', 'T', 'T', 'R'],
					100.,
				),
			],
		}
	}
	pub fn update(&mut self, rng: &mut ThreadRng) {
		for stock in self.stocks.iter_mut() {
			stock.update(rng);
		}
	}
}





pub struct Stock {
	name: [char; 4],
	current_price: f64,
	price_history: Vec<f64>,
}
impl Stock {
	fn new(name: [char; 4], init_price: f64) -> Self {
		Self {
			name,
			current_price: init_price,
			price_history: vec![init_price],
		}
	}

	fn get_name(&self) -> String {
		let [a, b, c, d] = self.name;
		format!("{a}{b}{c}{d}")
	}

	pub fn get_min_max(&self) -> (f64, f64) {
		let mut min = f64::MAX;
		let mut max = f64::MIN;
		for price in self.get_full_price_history() {
			if *price < min { min = *price }
			if *price > max { max = *price }
		}
		(min, max)
	}

	pub fn get_min_max_latest(&self, n: u32) -> (f64, f64) {
		let mut min = f64::MAX;
		let mut max = f64::MIN;
		for price in self.get_latest_price_history(n) {
			if *price < min { min = *price }
			if *price > max { max = *price }
		}
		(min, max)
	}

	pub fn get_full_price_history(&self) -> &[f64] {
		&self.price_history
	}

	pub fn get_latest_price_history(&self, n: u32) -> &[f64] {
		let n = n as usize;
		let i_begin = self.price_history.len().saturating_sub(n);
		let i_end = self.price_history.len();
		&self.price_history[i_begin .. i_end]
	}

	fn update(&mut self, rng: &mut ThreadRng) {
		enum Change { AddSub(f64), MulDiv(f64) }
		use Change::*;
		let change = match_random_weighted! { rng,
			1. => AddSub(rng.random_range(-1. ..= 1.)),
			0.2 => AddSub(rng.random_range(-0.1 ..= 0.1)),
			0.1 => AddSub(rng.random_range(-10. ..= 10.)),
			0.03 => AddSub(rng.random_range(-100. ..= 100.)),
			0.3 => MulDiv(rng.random_range(1. ..= 1.3)),
			0.01 => MulDiv(rng.random_range(1.3 ..= 3.)),
			0.001 => MulDiv(rng.random_range(3. ..= 10.)),
		};
		self.current_price = match change {
			AddSub(delta) => self.current_price + if rng.random_bool(0.5) { delta } else { -delta },
			MulDiv(coef) => self.current_price * if rng.random_bool(0.5) { coef } else { coef.recip() },
		};
		self.price_history.push(self.current_price);
	}
}
impl ToString for Stock {
	fn to_string(&self) -> String {
		format!("{name}: {price:.2}", name=self.get_name(), price=self.current_price)
	}
}

