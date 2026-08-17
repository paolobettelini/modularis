use uuid::Uuid;

/// 244 random bits represented as URL/path-safe hexadecimal text.
pub fn random_token() -> String {
    format!(
        "{}{}",
        Uuid::new_v4().simple(),
        Uuid::new_v4().simple()
    )
}
