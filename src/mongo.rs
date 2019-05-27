use mongodb::{
    Client,
    ThreadedClient,
    coll::{
        Collection,
        options::IndexOptions
    },
    ordered::OrderedDocument,
    bson,
    Bson,
};

pub struct Mongo;

impl Mongo {
    pub fn person_collection () -> Collection {
        let connect = Client::connect("localhost", 27017);
        let db = connect.unwrap().db("Rust");
        let collect = Collection::new(db, "persons", true, None, None);
        let mut index_opts = IndexOptions::new();
        index_opts.unique = Some(true);
        collect.create_index(doc! { "name": "text" }, Some(index_opts)).unwrap();
        collect
    }
    pub fn query_struct (document: Option<OrderedDocument>) -> OrderedDocument {
        let step_1 = document.unwrap();
        let step_2 = step_1.iter();
        let mut step_3 = step_2.filter(|i| -> bool {
            let (_, value) = i;
            (value.as_null() == None)
        });
        let mut copy_aux = OrderedDocument::new();
        loop {
            let item = step_3.next();
            if item == None {
                break;
            }
            let (name, value): (&String, &Bson) = item.unwrap();
            copy_aux.insert_bson(name.to_string(), value.clone());
        }
        
        copy_aux
    }
    pub fn normalice_to_doc (document: Option<Option<OrderedDocument>>) -> Bson {
        let checking_query = match document {
            Some(obj) => obj.unwrap_or(OrderedDocument::new()),
            None => OrderedDocument::new(),
        };
        Bson::Document(checking_query)
    }
    pub fn resolve_object_id (object_id: Option<Bson>) -> Option<String> {
        if object_id.is_some() { Some(object_id.unwrap().as_object_id().unwrap().to_hex()) } else { None }
    }
    pub fn resolve_query_one (document: OrderedDocument, collection: Collection) -> Bson {
        let response_query = collection.find_one(Some(document), None);
        let checking_query = match response_query {
            Ok(obj) => obj.or(Some( OrderedDocument::new() )),
            _error => Some( OrderedDocument::new() ),
        };
        Bson::Document(checking_query.unwrap())
    }
    pub fn insert_doc_operation (mut document: OrderedDocument) -> Option<Bson> {
        document.insert_bson("".to_owned(), Bson::Document(doc! {}))
    }
}
