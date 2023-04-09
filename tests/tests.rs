#[cfg(test)]
mod tests {
    use lushus_config::load;
    use std::env;
    use std::path::Path;
    use std::time::Duration;

    #[test]
    fn load_parses_environment() {
        env::set_var("DATABASE_PORT", "5432");

        let path = Path::new("./tests/test-data/development.yml");
        let config = load(path).unwrap();
        let environment = config.environment();

        assert_eq!(environment, "development");
    }

    #[test]
    fn load_parses_server() {
        let path = Path::new("./tests/test-data/server.yml");
        let config = load(path).unwrap();
        let server = config.server();

        assert_eq!(server.host(), "domain.com");
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
    fn load_parses_session() {
        env::set_var("DATABASE_PORT", "5432");

        let path = Path::new("./tests/test-data/development.yml");
        let config = load(path).unwrap();
        let session = config.session().unwrap();

        let expected_encryption_key = "abc-123-def";
        assert_eq!(session.encryption_key, expected_encryption_key);

        let expected_timeout = Duration::from_secs(3600);
        assert_eq!(session.timeout, expected_timeout);

        let expected_secure_cookies = false;
        assert_eq!(session.secure_cookies, expected_secure_cookies);
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
