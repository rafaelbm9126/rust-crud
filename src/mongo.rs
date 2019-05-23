use mongodb::{
    Client,
    ThreadedClient,
    coll::{
        Collection,
        options::IndexOptions
    },
    oid::ObjectId,
    bson,
};

pub struct Mongo;

impl Mongo {
    pub fn person_collection () -> Collection {
        let connect = Client::connect("localhost", 27017);
        let db = connect.unwrap().db("Rust");
        let mut collect = Collection::new(db, "persons", true, None, None);
        let mut index_opts = IndexOptions::new();
        index_opts.unique = Some(true);
        collect.create_index(doc! { "name": "text" }, Some(index_opts)).unwrap();
        collect
    }
}
