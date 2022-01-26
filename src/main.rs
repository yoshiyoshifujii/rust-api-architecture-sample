#[macro_use]
extern crate diesel;

mod domains;
mod interface_adaptors;
mod repositories;
mod server;
mod usecases;

fn main() -> std::io::Result<()> {
    server::run()
}
