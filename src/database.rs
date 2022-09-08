use serde::Deserialize;
use url::Url;

fn combine_components(
    password: &str,
    host: &str,
    name: &str,
    port: &u16,
    scheme: &str,
    user: &str,
) -> String {
    format!(
        "{scheme}://{user}:{password}@{host}:{port}/{name}",
        password = password,
        host = host,
        name = name,
        port = port,
        scheme = scheme,
        user = user,
    )
}

#[derive(Clone, Deserialize)]
pub struct Database {
    id: String,
    #[serde(flatten)]
    definition: Definition,
}

#[derive(Clone, Deserialize)]
#[serde(untagged)]
enum Definition {
    URL {
        url: String,
    },
    COMPONENTS {
        password: String,
        host: String,
        name: String,
        port: u16,
        scheme: String,
        username: String,
    },
}

impl Database {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn url(&self) -> String {
        match &self.definition {
            Definition::URL { url, .. } => url.to_string(),
            Definition::COMPONENTS {
                password,
                host,
                name,
                port,
                scheme,
                username,
            } => combine_components(password, host, name, port, scheme, username),
        }
    }

    pub fn name(&self) -> String {
        // TODO: error handling for invalid url
        // TODO: error handling for invalid path segments
        let url = Url::parse(&self.url()).unwrap();
        let path_segments = url
            .path_segments()
            .map(|path| path.collect::<Vec<_>>())
            .unwrap_or_default();
        let name = path_segments
            .first()
            .unwrap_or(&"INVALID DATABASE NAME")
            .to_string();
        name
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_be_constructed_from_url() {
        let url = "postgres://user:password@localhost:5432/development";
        let definition = Definition::URL {
            url: url.to_string(),
        };
        let database = Database {
            id: "database".to_string(),
            definition,
        };
        assert_eq!(database.url(), url);
        assert_eq!(database.name(), "development");
    }

    #[test]
    fn can_be_constructed_from_components() {
        let url = "postgres://user:password@localhost:5432/development";
        let definition = Definition::COMPONENTS {
            password: "password".to_string(),
            host: "localhost".to_string(),
            name: "development".to_string(),
            port: 5432,
            scheme: "postgres".to_string(),
            username: "user".to_string(),
        };
        let database = Database {
            id: "development".to_string(),
            definition,
        };
        assert_eq!(database.url(), url);
        assert_eq!(database.name(), "development");
    }

    #[test]
    fn can_be_deserialized_from_url() {
        let url = "postgres://user:password@localhost:5432/development";
        let yaml = r#"
        id: database
        url: "postgres://user:password@localhost:5432/development"
        "#;
        let database: Database = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(database.url(), url);
        assert_eq!(database.name(), "development");
    }

    #[test]
    fn can_be_deserialized_from_components() {
        let url = "postgres://user:password@localhost:5432/development";
        let yaml = r#"
        id: database
        password: "password"
        host: "localhost"
        name: "development"
        port: 5432
        scheme: "postgres"
        username: "user"
        "#;
        let database: Database = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(database.url(), url);
        assert_eq!(database.name(), "development");
    }
}
