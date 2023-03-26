pub struct DomainFilter {
    pub is_same_domain: bool,
    pub domain: String,
}
pub struct ExtensionFilter {
    pub enabled: bool,
    pub extensions: Vec<String>,
}
