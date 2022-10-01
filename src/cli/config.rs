use once_cell::sync::OnceCell;

pub static CF: OnceCell<Config> = OnceCell::new();

pub struct Config {
	pub bind: String,
	pub path: String,
}

pub fn init(bind: String, path: String) {
	let _ = CF.set(Config { bind, path });
}
