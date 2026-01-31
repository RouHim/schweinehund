use std::env;

pub fn port() -> u16 {
    env::var("PORT")
        .ok()
        .and_then(|v| v.parse::<u16>().ok())
        .unwrap_or(8090)
}

pub fn database_url() -> String {
    env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:data/schweinehund.sqlite".to_string())
        .trim()
        .to_string()
}

pub fn ntfy_url() -> String {
    env::var("NTFY_URL")
        .unwrap_or_else(|_| "http://ntfy:80/schweinehund".to_string())
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ntfy_url_default_and_override() {
        let old = env::var("NTFY_URL").ok();

        env::remove_var("NTFY_URL");
        assert_eq!(ntfy_url(), "http://ntfy:80/schweinehund".to_string());

        env::set_var("NTFY_URL", " http://example.local/topic ");
        assert_eq!(ntfy_url(), "http://example.local/topic".to_string());

        match old {
            Some(v) => env::set_var("NTFY_URL", v),
            None => env::remove_var("NTFY_URL"),
        }
    }

    #[test]
    fn test_database_url_default_and_override() {
        let old = env::var("DATABASE_URL").ok();

        env::remove_var("DATABASE_URL");
        assert_eq!(
            database_url(),
            "sqlite:data/schweinehund.sqlite".to_string()
        );

        env::set_var("DATABASE_URL", " sqlite:/tmp/test.sqlite ");
        assert_eq!(database_url(), "sqlite:/tmp/test.sqlite".to_string());

        match old {
            Some(v) => env::set_var("DATABASE_URL", v),
            None => env::remove_var("DATABASE_URL"),
        }
    }
}
