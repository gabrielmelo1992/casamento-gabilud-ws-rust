use std::env;
use mongodb::{
    bson::{
        // oid::ObjectId,
        doc,
        DateTime as BsonDateTime,
    }, //modify here
    // results::{ InsertOneResult },
    Client, Database,
    // Document,
    // Collection
};

#[derive(Clone)]

pub struct Mongo {
    // col: Collection<User>,
    db: Database,
}
async fn test_connection (db: &Database) -> bool {
    let ping = db.run_command(doc! {"ping": 1}, None).await;
    match ping {
        Ok(_) => true,
        Err(_) => false,
    }
}

impl Mongo {
    pub async fn init() -> Self {
        let mongo_url = match env::var("MONGO_URL") {
            Ok(v) => v.to_string(),
            Err(_) => panic!("Error loading env MONGO_URL variable"),
        };
        let mongo_db = match env::var("MONGO_DB") {
            Ok(v) => v.to_string(),
            Err(_) => panic!("Error loading env MONGO_DB variable"),
        };

        let client = Client::with_uri_str(&mongo_url).await.unwrap();
        let db = client.database(&mongo_db);
        test_connection(&db)
            .await
            .then(|| println!("🚀 Connected to MongoDB"))
            .unwrap_or_else(|| panic!("🔥 Failed to connect to MongoDB"));

        Self { db }
    }

    // pub async fn get(&self) -> Database {
    //     self.db.clone()
    // }

    pub async fn insert_invite(&self, name: String) -> Result<(), Box<dyn std::error::Error>> {
        let collection = self.db.collection("invite");
        let document = doc! {
            "name": name.clone(),
            "date": BsonDateTime::now()
        };
        let result = collection.insert_one(document, None).await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}