extern crate iron;
extern crate mount;
extern crate logger;
#[macro_use]
extern crate juniper;
#[macro_use]
extern crate juniper_iron;
#[macro_use]
extern crate juniper_codegen;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate mongodb;

mod mongo;

mod server;
use server::Server;

mod schema;

fn main() {
    Server::graph_start();
}
