use std::env;

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
}
