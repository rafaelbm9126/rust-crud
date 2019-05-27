extern crate iron;
extern crate mount;
extern crate logger;
#[macro_use]
extern crate juniper;
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

mod models;

fn main() {
    Server::graph_start();
}
