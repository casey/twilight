use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ActivityAssets {
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::ActivityAssets;
    use serde_test::Token;

    #[test]
    fn test_activity_secrets() {
        let value = ActivityAssets {
            large_image: Some("large image hash".to_owned()),
            large_text: Some("large image text".to_owned()),
            small_image: Some("small image hash".to_owned()),
            small_text: Some("small text hash".to_owned()),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "ActivityAssets",
                    len: 4,
                },
                Token::Str("large_image"),
                Token::Some,
                Token::Str("large image hash"),
                Token::Str("large_text"),
                Token::Some,
                Token::Str("large image text"),
                Token::Str("small_image"),
                Token::Some,
                Token::Str("small image hash"),
                Token::Str("small_text"),
                Token::Some,
                Token::Str("small text hash"),
                Token::StructEnd,
            ],
        );
    }
}
