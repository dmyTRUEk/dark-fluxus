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
	price_history: Vec<f64>, // TODO(optim): use VecDeque and pop if len > 10^6 or something
	owned_by_player: f64,
	bought_at: Vec<u32>,
	sold_at: Vec<u32>,
}
impl Stock {
	fn new(name: [char; 4], init_price: f64) -> Self {
		Self {
			name,
			current_price: init_price,
			price_history: vec![init_price],
			owned_by_player: 0.,
			bought_at: vec![],
			sold_at: vec![],
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
	pub fn get_price_at(&self, index: u32) -> f64 {
		self.price_history[index as usize]
	}

	pub fn get_price_history_len(&self) -> u32 {
		self.price_history.len() as u32
	}
	pub fn get_price_history_full(&self) -> &[f64] {
		&self.price_history
	}
	pub fn get_price_history_sqrt(&self) -> &[f64] {
		let len = self.get_price_history_len();
		let n = len.isqrt();
		&self.price_history[len.saturating_sub(n) as usize..]
	}
	pub fn get_price_history_log2(&self) -> &[f64] {
		let len = self.get_price_history_len();
		let n = len.ilog2();
		&self.price_history[len.saturating_sub(n) as usize..]
	}
	pub fn get_price_history_log10(&self) -> &[f64] {
		let len = self.get_price_history_len();
		let n = len.ilog10();
		&self.price_history[len.saturating_sub(n) as usize..]
	}
	pub fn get_price_history_latest(&self, n: u32) -> &[f64] {
		let len = self.get_price_history_len();
		&self.price_history[len.saturating_sub(n) as usize..]
	}

	pub fn get_bought_at_full(&self) -> &[u32] {
		&self.bought_at
	}
	pub fn get_sold_at_full(&self) -> &[u32] {
		&self.sold_at
	}
	pub fn get_bought_at_recent(&self, n: u32) -> &[u32] {
		let len = self.get_price_history_len();
		let n = len.saturating_sub(n);
		&self.bought_at[self.bought_at.partition_point(|&i| i < n)..]
	}
	pub fn get_sold_at_recent(&self, n: u32) -> &[u32] {
		let len = self.get_price_history_len();
		let n = len.saturating_sub(n);
		&self.sold_at[self.sold_at.partition_point(|&i| i < n)..]
	}

	pub fn calc_money_in_stock(&self) -> f64 {
		self.current_price * self.owned_by_player
	}

	pub fn calc_min_max_global(&self) -> (f64, f64) {
		let mut min = f64::MAX;
		let mut max = f64::MIN;
		// TODO(optim): benchmark this vs `.min` + `.max`
		for price in self.get_price_history_full() {
			if *price < min { min = *price }
			if *price > max { max = *price }
		}
		(min, max)
	}
	pub fn calc_min_max_latest(&self, n: u32) -> (f64, f64) {
		let mut min = f64::MAX;
		let mut max = f64::MIN;
		// TODO(optim): benchmark this vs `.min` + `.max`
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
			0.003 => MulDiv(rng.random_range(1. ..= 1.3)),
			0.0003 => MulDiv(rng.random_range(1.3 ..= 3.)),
			0.00003 => MulDiv(rng.random_range(3. ..= 10.)),
			1e-5 => MulDiv(-1.),
		};
		self.current_price = match change {
			None => self.current_price,
			AddSub(delta) => self.current_price + if rng.random_bool(0.5) { delta } else { -delta },
			MulDiv(coef) => self.current_price * if rng.random_bool(0.5) { coef } else { coef.recip() },
		};
		self.price_history.push(self.current_price);
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
		self.bought_at.push(self.price_history.len() as u32 - 1);
		Ok(())
	}
	pub fn try_sell(&mut self, n: f64, money: &mut f64) -> Result<(), SellError> {
		let is_enough_stocks_owned: bool = self.owned_by_player >= n;
		if !is_enough_stocks_owned { return Err(SellError::NotEnoughStocksOwned) }
		*money += n * self.current_price;
		self.owned_by_player -= n;
		self.sold_at.push(self.price_history.len() as u32 - 1);
		// TODO: improve, to clear more often (when its logical)
		if self.owned_by_player < 0.5 {
			self.bought_at.clear();
			self.sold_at.clear();
		}
		Ok(())
	}
}

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

