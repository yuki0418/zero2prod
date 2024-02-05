use unicode_segmentation::UnicodeSegmentation;

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}

pub struct SubscriberName(String);

impl SubscriberName {
    pub fn inner(self) -> String {
        self.0
    }

    pub fn inner_mut(&mut self) -> &mut String {
        &mut self.0
    }

    pub fn inner_ref(&self) -> &str {
        &self.0
    }

    pub fn parse(s: String) -> SubscriberName {
        let is_empty_or_whitespace = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contain_forbidden_characters =
            s.chars().any(|char| forbidden_characters.contains(&char));

        if is_empty_or_whitespace && is_too_long && contain_forbidden_characters {
            panic!("Subscriber name {} is invalid.", s);
        } else {
            Self(s)
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}