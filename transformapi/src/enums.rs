#[derive(Debug, Clone, PartialEq)]
pub enum TransformType {
    Uppercase,
    Lowercase,
}

impl TransformType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "uppercase" => Some(TransformType::Uppercase),
            "lowercase" => Some(TransformType::Lowercase),
            _ => None,
        }
    }

    pub fn transform_string(&self, input: &str) -> String {
        match self {
            TransformType::Uppercase => input.to_uppercase(),
            TransformType::Lowercase => input.to_lowercase(),
        }
    }
} 