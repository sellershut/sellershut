pub fn validate_username(username: &str) -> bool {
    let username = username.trim();

    if !(3..=15).contains(&username.len()) {
        return false;
    }

    if !username
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
    {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::users::validate_username;

    #[track_caller]
    fn check(input: &str, expected_result: bool) {
        let actual_result = validate_username(input);
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn empty() {
        check("", false);
        check("  ", false);
    }

    #[test]
    fn length() {
        check("123456789012345", true);
        check("1234567890123451", false);
        check("12", false);
        check("1", false);
    }

    #[test]
    fn whitespace() {
        check("  123456789012345  ", true);
        check("  1234567890123451  ", false);
        check("12 3", false);
    }

    #[test]
    fn non_ascii() {
        check("  Ω ∑ π ≤ ≥abc  ", false);
    }
}
