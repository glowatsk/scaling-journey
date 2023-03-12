#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        Ok(Self(s))
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberEmail;
    use claim::{assert_err, assert_ok};

    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(SubscriberEmail::parse(name));
    }

    #[test]
    fn email_missinig_at_symbol_is_rejected() {
        let name = "ursuladomain.com".to_string();
        assert_err!(SubscriberEmail::parse(name));
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let name = "@domain.com".to_string();
        assert_err!(SubscriberEmail::parse(name));
    }

    #[test]
    fn valid_email_is_not_rejected() {
        let name = "ursula_le_guin@domain.com".to_string();
        assert_ok!(SubscriberEmail::parse(name));
    }
}
