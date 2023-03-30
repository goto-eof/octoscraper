use super::resource_extractor_service::{ImageExtractor, ResourceExtractor, VideoExtractor};

pub type ExtractorType = Box<dyn ResourceExtractor>;
pub fn retrieve_strategy() -> Vec<ExtractorType> {
    let mut extractors: Vec<ExtractorType> = Vec::new();
    extractors.push(Box::new(VideoExtractor {}));
    extractors.push(Box::new(ImageExtractor {}));
    return extractors;
}
