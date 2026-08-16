use secrecy::{ExposeSecret, SecretString};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct RedactedSecret(SecretString);

impl RedactedSecret {
    pub fn expose(&self) -> String {
        self.0.expose_secret().to_owned()
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for RedactedSecret {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("")
    }
}
