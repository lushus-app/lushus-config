use serde::Deserialize;

use crate::{Cors, Database, Server, Session};

#[derive(Clone, Deserialize)]
pub struct Config {
    #[serde(skip_deserializing)]
    environment: String,
    server: Server,
    cors: Option<Cors>,
    databases: Vec<Database>,
    session: Option<Session>,
}

impl Config {
    pub fn environment(&self) -> &str {
        &self.environment
    }

    pub fn set_environment(&mut self, value: &str) {
        self.environment = value.to_string()
    }

    pub fn server(&self) -> &Server {
        &self.server
    }

    pub fn cors(&self) -> Option<&Cors> {
        self.cors.as_ref()
    }

    pub fn databases(&self) -> &[Database] {
        &self.databases
    }

    pub fn database(&self, id: &str) -> Option<&Database> {
        self.databases.iter().find(|database| database.id() == id)
    }

    pub fn session(&self) -> Option<&Session> {
        self.session.as_ref()
    }
}
