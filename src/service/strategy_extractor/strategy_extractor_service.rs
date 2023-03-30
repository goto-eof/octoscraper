use super::{
    strategy_audio_extractor::AudioExtractor, strategy_image_extractor::ImageExtractor,
    strategy_resource_extractor::ResourceExtractor, strategy_video_extractor::VideoExtractor,
};

pub type ExtractorType = Box<dyn ResourceExtractor>;
pub fn retrieve_strategy() -> Vec<ExtractorType> {
    let mut extractors: Vec<ExtractorType> = Vec::new();
    extractors.push(Box::new(VideoExtractor {}));
    extractors.push(Box::new(ImageExtractor {}));
    extractors.push(Box::new(AudioExtractor {}));
    return extractors;
}
