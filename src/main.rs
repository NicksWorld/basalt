use ::chrono::Local;
use ::fern::Dispatch;
use ::log::LevelFilter;
use ::std::io;

mod connection;
mod server;
mod status;
mod types;

use server::Server;

#[::tokio::main]
async fn main() {
	Dispatch::new()
		.format(|out, msg, record| {
			out.finish(format_args!(
				"[{}][{}][{}] {}",
				Local::now().format("%H:%M:%S%.3f"),
				record.target(),
				record.level(),
				msg
			))
		})
		.level(LevelFilter::Debug)
		.chain(io::stdout())
		.apply()
		.unwrap();
	let addr = "0.0.0.0".parse().unwrap();
	let mut server = Server::new(addr).await.unwrap();
	// ...
	server.listen().await.unwrap();
}
