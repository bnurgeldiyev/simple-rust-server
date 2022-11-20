use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Datastore {
    pub db_host: String,
    pub db_user: String,
    pub db_password: String,
    pub db_name: String,
}
