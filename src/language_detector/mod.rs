use std::time::Instant;
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};

use crate::language_convertor::Convertor;

pub fn load() -> LanguageDetector {
    // let languages = vec![English, French];
    let detector = LanguageDetectorBuilder::from_all_languages().build();
    detector
}
pub fn detect(detector: &LanguageDetector, input: &str) -> String {
    let start = Instant::now();
    println!("Starting detection");
    let detected_language: String = detector.detect_language_of(input).unwrap_or(lingua::Language::English).to_string();
    println!("Detection ended");
    let duration = start.elapsed();
    println!("Ran detection in: {:?}", duration);
    Convertor::load().get_language(&detected_language).bert_language.get_iso_639_1_code().to_string()
}