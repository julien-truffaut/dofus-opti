use crate::model::Language;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TranslatedName {
    pub en: String,
    pub fr: String,
}

impl TranslatedName {
    pub fn localized(&self, language: Language) -> &str {
        match language {
            Language::English => &self.en,
            Language::French => &self.fr,
        }
    }
}
