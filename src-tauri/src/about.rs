pub const REPO_URL: &str = "https://github.com/arvinmoj/GhostWriter";
pub const DIALOG_TITLE: &str = "About GhostWriter";
pub const OPEN_REPO_BUTTON: &str = "View on GitHub";
pub const OK_BUTTON: &str = "OK";

pub fn dialog_body() -> String {
    format!("GhostWriter v{}\n\n{}", env!("CARGO_PKG_VERSION"), REPO_URL)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn body_contains_version() {
        assert!(dialog_body().contains(env!("CARGO_PKG_VERSION")));
    }

    #[test]
    fn body_contains_repo_url() {
        assert!(dialog_body().contains(REPO_URL));
    }

    #[test]
    fn body_has_product_name() {
        assert!(dialog_body().contains("GhostWriter"));
    }

    #[test]
    fn repo_url_is_https_github() {
        assert!(REPO_URL.starts_with("https://github.com/"));
    }

    #[test]
    fn dialog_title_mentions_product() {
        assert!(DIALOG_TITLE.contains("GhostWriter"));
    }

    #[test]
    fn body_has_two_lines_separated_by_blank() {
        let body = dialog_body();
        assert!(body.contains("\n\n"));
    }
}
