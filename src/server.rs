use std::env;
use logger::Logger;
use mount::Mount;
use iron::prelude::*;
use juniper_iron::{
    GraphQLHandler,
    GraphiQLHandler,
};
use crate::schema::{
    Query,
    Mutation,
};

pub struct Server;

fn context_factory(req: &mut Request) -> IronResult<()> {
    Ok(())
}

fn engine (mut mount: Mount) -> Chain {
    let graphql_endpoint = GraphQLHandler::new(
        context_factory,
        Query { ..Default::default() },
        Mutation { ..Default::default() },
    );
    let graphiql_endpoint = GraphiQLHandler::new("/graphql");
    mount.mount("/graphql", graphql_endpoint);
    mount.mount("/", graphiql_endpoint);
    Chain::new(mount)
}

impl Server {
    pub fn graph_start () {
        env::set_var("RUST_BACKTRACE", "full");
        let (logger_before, logger_after) = Logger::new(None);
        let mount = Mount::new();
        let mut chain = engine(mount);
        chain.link_before(logger_before);
        chain.link_after(logger_after);
        let host = env::var("LISTEN").unwrap_or("localhost:9009".to_owned());
        println!("GraphQL server started on {}", host);
        Iron::new(chain).http(host.as_str()).unwrap();
    }
}
