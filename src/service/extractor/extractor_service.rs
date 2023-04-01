use super::extractors::{
    audio_link_extractor::AudioLinkExtractor, image_link_extractor::ImageLinkExtractor,
    other_link_extractor::OtherFileLinkExtractor, resource_extractor::ResourceExtractor,
    video_link_extractor::VideoLinkExtractor,
};
use crate::structure::config_struct::Config;

pub type ExtractorType = Box<dyn ResourceExtractor>;
pub fn retrieve_extractors(config: &Config, processing_page_link: &str) -> Vec<ExtractorType> {
    let mut extractors: Vec<ExtractorType> = Vec::new();
    extractors.push(Box::new(VideoLinkExtractor {
        enabled: config.is_video_extractor_enabled,
        extensions: config.video_extractor_extensions.to_owned(),
        is_same_domain_enabled: config.processing_same_domain,
        domain: config.website.clone(),
        processing_page_link: processing_page_link.to_owned(),
    }));
    extractors.push(Box::new(ImageLinkExtractor {
        enabled: config.is_image_extractor_enabled,
        extensions: config.image_extractor_extensions.to_owned(),
        is_same_domain_enabled: config.processing_same_domain,
        domain: config.website.clone(),
        processing_page_link: processing_page_link.to_owned(),
    }));
    extractors.push(Box::new(AudioLinkExtractor {
        enabled: config.is_audio_extractor_enabled,
        extensions: config.audio_extractor_extensions.to_owned(),
        is_same_domain_enabled: config.processing_same_domain,
        domain: config.website.clone(),
        processing_page_link: processing_page_link.to_owned(),
    }));
    extractors.push(Box::new(OtherFileLinkExtractor {
        enabled: config.is_other_extractor_enabled,
        extensions: config.other_extractor_extensions.to_owned(),
        is_same_domain_enabled: config.processing_same_domain,
        domain: config.website.clone(),
        processing_page_link: processing_page_link.to_owned(),
    }));
    return extractors;
}
