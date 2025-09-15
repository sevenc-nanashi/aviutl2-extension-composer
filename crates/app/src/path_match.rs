pub fn matches_path(
    target: &str,
    pattern: &str,
) -> Option<std::collections::HashMap<String, String>> {
    assert!(pattern.starts_with('/'), "Pattern must start with '/'");
    if !target.starts_with('/') {
        return None;
    }

    let target_parts: Vec<&str> = target.trim_start_matches('/').split('/').collect();
    let pattern_parts: Vec<&str> = pattern.trim_start_matches('/').split('/').collect();
    if target_parts.len() != pattern_parts.len() {
        return None;
    }
    let mut params = std::collections::HashMap::new();
    for (t, p) in target_parts.iter().zip(pattern_parts.iter()) {
        if p.starts_with(':') {
            params.insert(p.trim_start_matches(':').to_string(), t.to_string());
        } else if t != p {
            return None;
        }
    }
    Some(params)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matches_path_basic() {
        let target = "/users/123/profile";
        let pattern = "/users/:user_id/profile";
        let result = matches_path(target, pattern);
        assert!(result.is_some());
        let params = result.unwrap();
        assert_eq!(params.get("user_id").unwrap(), "123");
    }

    #[test]
    fn test_matches_path_no_match() {
        let target = "/users/123/profile";
        let pattern = "/users/:user_id/settings";
        let result = matches_path(target, pattern);
        assert!(result.is_none());
    }

    #[test]
    fn test_matches_path_exact() {
        let target = "/users/123/profile";
        let pattern = "/users/123/profile";
        let result = matches_path(target, pattern);
        assert!(result.is_some());
        let params = result.unwrap();
        assert!(params.is_empty());
    }
}
