use mongodb::{options::ClientOptions, Client, Collection};

use crate::config::Config;
pub mod users;
use crate::database::users::User;
pub mod data_packets;
use crate::database::data_packets::DataPacket;

pub mod util;

#[derive(Debug, Clone)]
pub struct DatabaseContainer {
    pub users: Collection<User>,
    pub data_packets: Collection<DataPacket>,
}

impl DatabaseContainer {
    pub async fn new(config: &Config) -> Self {
        let client_options = ClientOptions::parse(config.database.connection_string.to_string())
            .await
            .unwrap();
        let client = Client::with_options(client_options).unwrap();
        let db = client.database(&config.database.dbname);

        DatabaseContainer {
            users: db.collection("users"),
            data_packets: db.collection("data-archive"),
        }
    }
}
