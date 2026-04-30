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
	pub fn calc_money_in_stocks(&self) -> f64 {
		self.stocks.iter().map(|s| s.calc_money_in_stock()).sum()
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
	owned_by_player: f64,
}
impl Stock {
	fn new(name: [char; 4], init_price: f64) -> Self {
		Self {
			name,
			current_price: init_price,
			price_history: vec![init_price],
			owned_by_player: 0.
		}
	}

	pub fn get_name(&self) -> String {
		let [a, b, c, d] = self.name;
		format!("{a}{b}{c}{d}")
	}

	pub fn get_current_price(&self) -> f64 {
		self.current_price
	}

	pub fn get_n_owned_by_player(&self) -> f64 {
		self.owned_by_player
	}

	pub fn get_price_history_full(&self) -> &[f64] {
		&self.price_history
	}

	pub fn get_price_history_sqrt(&self) -> &[f64] {
		let n = (self.price_history.len()).isqrt();
		&self.price_history[self.price_history.len().saturating_sub(n)..]
	}

	pub fn get_price_history_log2(&self) -> &[f64] {
		let n = (self.price_history.len()).ilog2() as usize;
		&self.price_history[self.price_history.len().saturating_sub(n)..]
	}

	pub fn get_price_history_log10(&self) -> &[f64] {
		let n = (self.price_history.len()).ilog10() as usize;
		&self.price_history[self.price_history.len().saturating_sub(n)..]
	}

	pub fn get_price_history_latest(&self, n: u32) -> &[f64] {
		let n = n as usize;
		let i_begin = self.price_history.len().saturating_sub(n);
		let i_end = self.price_history.len();
		&self.price_history[i_begin .. i_end]
	}

	pub fn calc_money_in_stock(&self) -> f64 {
		self.current_price * self.owned_by_player
	}

	pub fn calc_min_max_global(&self) -> (f64, f64) {
		let mut min = f64::MAX;
		let mut max = f64::MIN;
		for price in self.get_price_history_full() {
			if *price < min { min = *price }
			if *price > max { max = *price }
		}
		(min, max)
	}

	pub fn calc_min_max_latest(&self, n: u32) -> (f64, f64) {
		let mut min = f64::MAX;
		let mut max = f64::MIN;
		for price in self.get_price_history_latest(n) {
			if *price < min { min = *price }
			if *price > max { max = *price }
		}
		(min, max)
	}

	fn update(&mut self, rng: &mut ThreadRng) {
		enum Change { None, AddSub(f64), MulDiv(f64) }
		use Change::*;
		let change = match_random_weighted! { rng,
			0.1 => None,
			1. => AddSub(rng.random_range(-1. ..= 1.)),
			0.3 => AddSub(rng.random_range(-0.1 ..= 0.1)),
			0.1 => AddSub(rng.random_range(-10. ..= 10.)),
			0.01 => AddSub(rng.random_range(-100. ..= 100.)),
			0.01 => MulDiv(rng.random_range(1. ..= 1.3)),
			0.001 => MulDiv(rng.random_range(1.3 ..= 3.)),
			0.00001 => MulDiv(rng.random_range(3. ..= 10.)),
		};
		self.current_price = match change {
			None => self.current_price,
			AddSub(delta) => self.current_price + if rng.random_bool(0.5) { delta } else { -delta },
			MulDiv(coef) => self.current_price * if rng.random_bool(0.5) { coef } else { coef.recip() },
		};
		self.price_history.push(self.current_price);
	}

	pub fn to_string_with_minmax(&self, n: u32) -> String {
		let name = self.get_name();
		let price = self.current_price;
		let (min, max) = self.calc_min_max_latest(n);
		let (gmin, gmax) = self.calc_min_max_global();
		format!("{name}: {price:.2}, MIN: {min:.2}, MAX: {max:.2}, GMIN: {gmin:.2}, GMAX: {gmax:.2}")
	}

	pub fn try_buy_with_scale(&mut self, buy_scale: u32, money: &mut f64) -> Result<(), BuyError> {
		self.try_buy(buy_sell_scale_to_n(buy_scale), money)
	}

	pub fn try_sell_with_scale(&mut self, sell_scale: u32, money: &mut f64) -> Result<(), SellError> {
		self.try_sell(buy_sell_scale_to_n(sell_scale), money)
	}

	pub fn try_buy(&mut self, n: f64, money: &mut f64) -> Result<(), BuyError> {
		let is_enough_money = *money >= n * self.current_price;
		if !is_enough_money { return Err(BuyError::NotEnoughMoney) }
		let is_positive_price = self.current_price > 0.;
		if !is_positive_price { return Err(BuyError::CantBuyNegativeValueStock) }
		*money -= n * self.current_price;
		self.owned_by_player += n;
		Ok(())
	}

	pub fn try_sell(&mut self, n: f64, money: &mut f64) -> Result<(), SellError> {
		let is_enough_stocks_owned: bool = self.owned_by_player >= n;
		if !is_enough_stocks_owned { return Err(SellError::NotEnoughStocksOwned) }
		*money += n * self.current_price;
		self.owned_by_player -= n;
		Ok(())
	}
}
// impl ToString for Stock {
// 	fn to_string(&self) -> String {
// 		format!("{name}: {price:.2}", name=self.get_name(), price=self.current_price)
// 	}
// }

pub enum BuyError {
	NotEnoughMoney,
	CantBuyNegativeValueStock,
}

pub enum SellError {
	NotEnoughStocksOwned,
}





pub fn buy_sell_scale_to_n(buy_sell_scale: u32) -> f64 {
	10_f64.powi(buy_sell_scale as i32)
}

pub fn buy_sell_scale_to_n_str(buy_sell_scale: u32) -> String {
	match buy_sell_scale {
		0 => format!("1"),
		1 => format!("10"),
		2 => format!("100"),
		3 => format!("1000"),
		n => format!("10^{n}")
	}
}

