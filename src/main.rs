use ::chrono::Local;
use ::fern::Dispatch;
use ::log::LevelFilter;
use ::std::io;

mod config;
mod connection;
mod protocol;
mod server;
mod status;
mod types;

use config::Config;
use server::Server;

#[::tokio::main]
async fn main() {
	Dispatch::new()
		.format(|out, msg, record| {
			out.finish(format_args!(
				"[{}][{}] {}",
				Local::now().format("%H:%M:%S%.3f"),
				record.level(),
				msg
			))
		})
		.level(LevelFilter::Debug)
		.chain(io::stdout())
		.apply()
		.unwrap();
	let config = Config::read("basalt.toml").await.unwrap();
	let mut server = Server::new(&config).await.unwrap();
	// ...
	server.listen(&config).await.unwrap();
}
