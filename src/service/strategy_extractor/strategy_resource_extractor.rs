

pub trait ResourceExtractor {
    fn enabled(&mut self, enabled: bool);
    fn extract(&self, resource_str: &str) -> Vec<String>;
}
