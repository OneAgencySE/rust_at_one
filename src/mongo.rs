use super::Result;
use crate::AppError;
use mongodb::{bson::oid::ObjectId, Client, Database};

#[derive(Clone)]
pub struct Mongo {
    client: Client,
    pub main_db: Database,
}

impl Mongo {
    pub async fn initialize(connection_string: &str) -> Result<Self> {
        let client = Client::with_uri_str(connection_string).await?;
        let main_db = client.database("rust_at_one");
        Ok(Mongo { client, main_db })
    }

    pub fn to_object_id(id: &str) -> Result<ObjectId> {
        Ok(ObjectId::with_string(id).map_err(|e| AppError::InternalServerError(e.to_string()))?)
    }
}
