use std::iter::Iterator;
use std::string::String;

pub fn escape_like_query(query: String) -> String {
    query
        .chars()
        .map(|s| match s {
            '!' | '%' | '_' => format!("\\{}", s),
            _ => s.to_string(),
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn escaped() {
        let query = "!%_";
        assert_eq!(escape_like_query(query.to_string()), "\\!\\%\\_")
    }

    #[test]
    pub fn not_escaped() {
        let query = "adasdf";
        assert_eq!(escape_like_query(query.to_string()), query)
    }
}
