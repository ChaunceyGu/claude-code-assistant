use chrono::{DateTime, Utc};

/// Format a datetime to a human-readable string
pub fn format_datetime(dt: DateTime<Utc>) -> String {
    dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}

/// Get current timestamp as string
pub fn current_timestamp() -> String {
    Utc::now().to_rfc3339()
}

/// Truncate string with ellipsis
pub fn truncate(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        s.to_string()
    } else {
        format!("{}...", &s.chars().take(max_chars).collect::<String>())
    }
}

/// Validate file name (basic security check)
pub fn is_valid_filename(name: &str) -> bool {
    !name.is_empty()
        && !name.contains("..")
        && !name.contains('/')
        && !name.contains('\\')
        && !name.contains('\0')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("hello", 10), "hello");
        assert_eq!(truncate("hello world", 5), "hello...");
    }

    #[test]
    fn test_is_valid_filename() {
        assert!(is_valid_filename("test.txt"));
        assert!(!is_valid_filename(""));
        assert!(!is_valid_filename("../etc/passwd"));
        assert!(!is_valid_filename("path/to/file"));
    }
}
