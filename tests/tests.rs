#[cfg(test)]
mod tests {
    use lushus_config::load;
    use std::env;
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
        env::set_var("DATABASE_PORT", "5432");

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

    #[test]
    fn load_parses_environment_variables() {
        const DATABASE_URL: &str = "postgres://user:password@localhost:5432/production";
        const CACHE_URL: &str = "redis://:password@localhost:6379/1";
        env::set_var("DATABASE_URL", DATABASE_URL);
        env::set_var("CACHE_URL", CACHE_URL);

        let path = Path::new("./tests/test-data/production.yml");
        let config = load(path).unwrap();
        let databases = config.databases();

        let expected_url = DATABASE_URL;
        assert_eq!(databases[0].url(), expected_url);

        let expected_url = CACHE_URL;
        assert_eq!(databases[1].url(), expected_url);
    }
}
