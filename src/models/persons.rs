use mongodb::{
    oid::ObjectId,
};

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
pub struct PersonQueryInput {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, GraphQLInputObject)]
pub struct PersonAgeQueryRange {
    pub gte: i32,
    pub lte: i32,
}

#[derive(Serialize, Deserialize, Debug, Default, GraphQLInputObject)]
pub struct PersonMutateInput {
    pub name: String,
    pub age: i32,
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject)]
pub struct PersonMutateResponse {
    pub id: String,
    pub name: String,
    pub age: i32,
}

