use mongodb::{
    oid::ObjectId,
    bson,
    ordered::OrderedDocument,
};

use crate::mongo::Mongo;

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    #[serde(rename="_id")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub age: i32,
}
graphql_object!(Person: () |&self| {
    field id() -> String {
        if let Some(ref id) = self.id { id.to_hex() } else { "".into() }
    }
    field name() -> String {
        self.name.to_string()
    }
    field age() -> i32 {
        self.age
    }
});
 
 
#[derive(Serialize, Deserialize, Debug, GraphQLInputObject)]
pub struct PersonInput {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Query;
graphql_object!(Query: () |&self| {
    field foo() -> String {
        "Bar".to_owned()
    }
    field person(input: PersonInput) -> Option<Person> {
        let collection = Mongo::person_collection();
        let result = collection.find_one(None, None).unwrap().unwrap();
        let mut to_bson = bson::to_bson(&result).unwrap();
        let fr_bson = bson::from_bson::<Person>(to_bson).unwrap();



        let resolvet = bson::to_bson(&input).unwrap().as_document().cloned();
        let resolve = bson::to_bson(&input).unwrap().as_document().cloned();

        let find = OrderedDocument::new();
        let u = resolve.unwrap().iter().filter(|i| -> bool {
            let (item, value) = i;
            (value.as_null() == None)
        });
        
        println!("{:#?}", u);
        // println!("{:#?} {:?}", collection.find_one(resolve, None).unwrap().unwrap(), resolvet.into_iter());

        Some (fr_bson)
    }
});
 
 
#[derive(Serialize, Deserialize, Debug, Default, GraphQLInputObject)]
pub struct PersonInputMut {
    pub name: String,
    pub age: i32,
}
 
#[derive(Debug, Default)]
pub struct Mutation;
graphql_object!(Mutation: () |&self| {
    field foo() -> String {
        println!("{:#?}", self);
        "Bar".to_string()
    }
});