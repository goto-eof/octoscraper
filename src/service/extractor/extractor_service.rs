use crate::structure::config_struct::Config;

use super::extractors::{
    audio_extractor::AudioExtractor, image_extractor::ImageExtractor,
    other_extractor::OtherFileExtractor, resource_extractor::ResourceExtractor,
    video_extractor::VideoExtractor,
};

pub type ExtractorType = Box<dyn ResourceExtractor>;
pub fn retrieve_extractors(config: &Config, processing_page_link: &str) -> Vec<ExtractorType> {
    let mut extractors: Vec<ExtractorType> = Vec::new();
    extractors.push(Box::new(VideoExtractor {
        enabled: config._is_video_extractor_enabled,
        extensions: config._video_extractor_extensions.to_owned(),
        is_same_domain_enabled: config.processing_same_domain,
        domain: config.website.clone(),
        processing_page_link: processing_page_link.to_owned(),
    }));
    extractors.push(Box::new(ImageExtractor {
        enabled: config._is_image_extractor_enabled,
        extensions: config._image_extractor_extensions.to_owned(),
        is_same_domain_enabled: config.processing_same_domain,
        domain: config.website.clone(),
        processing_page_link: processing_page_link.to_owned(),
    }));
    extractors.push(Box::new(AudioExtractor {
        enabled: config._is_audio_extractor_enabled,
        extensions: config._audio_extractor_extensions.to_owned(),
        is_same_domain_enabled: config.processing_same_domain,
        domain: config.website.clone(),
        processing_page_link: processing_page_link.to_owned(),
    }));
    extractors.push(Box::new(OtherFileExtractor {
        enabled: config._is_other_extractor_enabled,
        extensions: config._other_extractor_extensions.to_owned(),
        is_same_domain_enabled: config.processing_same_domain,
        domain: config.website.clone(),
        processing_page_link: processing_page_link.to_owned(),
    }));
    return extractors;
}
