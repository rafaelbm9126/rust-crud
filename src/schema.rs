use mongodb::{
    oid::ObjectId,
    bson,
    Bson,
    ordered::OrderedDocument,
    Error,
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

        let mut resolvet = bson::to_bson(&input).unwrap().as_document().cloned();
        let resolve = bson::to_bson(&input).unwrap().as_document().cloned();

        let find = OrderedDocument::new();
        let step_1 = resolve.unwrap();
        let step_2 = step_1.iter();
        let mut step_3 = step_2.filter(|i| -> bool {
            let (item, value) = i;
            (value.as_null() == None)
        });
        
        let mut docky = OrderedDocument::new();

        loop {
            let item = step_3.next();
            if item == None {
                break;
            }
            let (name, value): (&String, &Bson) = item.unwrap();
            docky.insert_bson(name.to_string(), value.clone());
        }

        let resultt = collection.find_one(Some(docky), None);

        let trans = match resultt {
            Ok(obj) => obj.or(Some( OrderedDocument::new() )),
            Error => Some( OrderedDocument::new() ),
        };

        let tranformation_OrDoc_to_Bson = Bson::Document(trans.unwrap());

        let que = bson::from_bson::<Person>(tranformation_OrDoc_to_Bson);

        que.ok()
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
