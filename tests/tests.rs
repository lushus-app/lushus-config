#[cfg(test)]
mod tests {
    use config::load;
    use std::path::Path;

    #[test]
    fn load_parses_environment() {
        let path = Path::new("./tests/test-data/development.yml");
        let config = load(path).unwrap();
        let environment = config.environment();

        assert_eq!(environment, "development");
    }

    #[test]
    fn load_parses_server() {
        let path = Path::new("./tests/test-data/development.yml");
        let config = load(path).unwrap();
        let server = config.server();

        assert_eq!(server.host(), "localhost");
        assert_eq!(server.port(), 3000);
    }

    #[test]
    fn load_parses_databases() {
        let path = Path::new("./tests/test-data/development.yml");
        let config = load(path).unwrap();
        let databases = config.databases();

        let expected_url = "postgres://user:password@localhost:5432/development";
        assert_eq!(databases[0].url(), expected_url);

        let expected_url = "postgres://user:password@localhost:5432/backup";
        assert_eq!(databases[1].url(), expected_url);

        let expected_url = "redis://:password@localhost:6379/1";
        assert_eq!(databases[2].url(), expected_url);
    }
}
