use serde::{Deserialize, Serialize};

/// Model representing a language available in TIO.
#[derive(Clone, Debug)]
pub struct Language {
    /// The name of the language TIO expects us to provide for execution.
    pub tio_name: String,
    /// The actual, raw name of the language.
    pub name: String,
    /// Some tags for the programming language.
    pub categories: Vec<String>,
    /// The encoding format of the language, e.g. utf-8.
    pub encoding: String,
    /// The link to the home page of the language.
    pub link: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LanguageData {
    pub name: String,
    pub categories: Vec<String>,
    pub encoding: String,
    pub link: String,
}

impl Language {
    pub fn new(name: &str, data: LanguageData) -> Self {
        Self {
            tio_name: name.to_owned(),
            name: data.name,
            categories: data.categories,
            encoding: data.encoding,
            link: data.link,
        }
    }
}

impl PartialEq for Language {
    fn eq(&self, other: &Self) -> bool {
        self.tio_name == other.tio_name
    }
}

impl PartialEq<str> for Language {
    fn eq(&self, other: &str) -> bool {
        [&self.tio_name as &str, &self.name].contains(&other)
    }
}
