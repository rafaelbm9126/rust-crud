use mongodb::{
    bson,
    oid::ObjectId,
    Bson,
};

use crate::mongo::Mongo;

use crate::models::persons::{
    Person,
    PersonQueryInput,
    PersonAgeQueryRange,
    PersonMutateInput,
    PersonMutateResponse,
};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Query;
graphql_object!(Query: () |&self| {
    field person(input: PersonQueryInput) -> Option<Person> {
        let collection = Mongo::person_collection();
        let input_to_document = bson::to_bson(&input).unwrap().as_document().cloned();
        let result_document = Mongo::query_struct(input_to_document);
        let result_query = Mongo::resolve_query_one(result_document, collection);
        bson::from_bson::<Person>(result_query).ok()
    }
    field person_id(id: String) -> Option<Person> {
        let collection = Mongo::person_collection();
        let object_id = ObjectId::with_string(&id);
        if object_id.is_ok() {
            let result_query = collection.find_one(Some(doc! { "_id": object_id.ok().unwrap() }), None).ok();
            let normaliced = Mongo::normalice_to_doc(result_query);
            return bson::from_bson::<Person>(normaliced).ok()
        }

        None
    }
    field persons(limit: i32, input: PersonQueryInput) -> Vec<Option<Person>> {
        let collection = Mongo::person_collection();
        let mut result_query: Vec<Option<Person>> = Vec::new();
        let input_to_document = bson::to_bson(&input).unwrap().as_document().cloned();
        let result_document = Mongo::query_struct(input_to_document);
        let result_cursor = collection.find(Some(result_document),None);
        let resolve_query = match result_cursor {
            Ok(mut cursor) => cursor.next_n(limit as usize).ok(),
            _error => None
        };
        if resolve_query.is_some() {
            let resolve_vector = resolve_query.unwrap();
            for item in resolve_vector.into_iter() {
                result_query.push(
                    bson::from_bson::<Person>(Bson::Document(item)).ok()
                )
            }
        }
        result_query
    }
    field persons_find(limit: i32, input: PersonAgeQueryRange) ->  Vec<Option<Person>> {
        let collection = Mongo::person_collection();
        let mut result_query: Vec<Option<Person>> = Vec::new();
        let result_cursor = collection.find(Some(
            doc! { "age": doc! { "$lte": input.lte, "$gte": input.gte } }
        ), None);
        
         let resolve_query = match result_cursor {
            Ok(mut cursor) => cursor.next_n(limit as usize).ok(),
            _error => None
        };

        if resolve_query.is_some() {
            let resolve_vector = resolve_query.unwrap();
            for item in resolve_vector.into_iter() {
                result_query.push(
                    bson::from_bson::<Person>(Bson::Document(item)).ok()
                )
            }
        }
        result_query
    }
});

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Mutation;
graphql_object!(Mutation: () |&self| {
    field person(input: PersonMutateInput) -> Option<PersonMutateResponse> {
        let collection = Mongo::person_collection();
        let input_to_document = bson::to_bson(&input).unwrap().as_document().cloned();
        let u = collection.insert_one(input_to_document.unwrap(), None);
        let ok_insert = u.unwrap();
        if ok_insert.inserted_id.is_some() {
            let id_str = Mongo::resolve_object_id(ok_insert.inserted_id);
            return Some (
                PersonMutateResponse {
                    id: id_str.unwrap(),
                    name: input.name,
                    age: input.age,
                }
            );
        }
        None
    }
});
