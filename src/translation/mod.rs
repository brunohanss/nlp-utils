use rocket::tokio;
use rust_bert::{
    pipelines::{translation::{Language, TranslationModel, TranslationModelBuilder}, common::{ModelType, self}},
    RustBertError, marian::MarianSourceLanguages,
};

use crate::{ TranslationModels};

pub async fn load() -> TranslationModels {
    TranslationModels {
        m2m_100: load_m2m_100().await,
        marian: load_marian().await,
    }
}
pub async fn load_m2m_100() -> TranslationModel {
    let languages = vec![Language::Afrikaans, Language::Albanian, Language::Amharic, Language::Arabic, Language::Armenian, Language::Azerbaijani, Language::Belarusian, Language::Bengali, Language::Bosnian, Language::Bulgarian, Language::Catalan, Language::Cebuano, Language::Croatian, Language::Czech, Language::Danish, Language::Dutch, Language::English,  Language::Estonian,  Language::Finnish, Language::French,  Language::Galician, Language::Georgian, Language::German, Language::Greek, Language::Gujarati, Language::HaitianCreole, Language::Hausa,  Language::Hebrew, Language::Hindi,  Language::Hungarian, Language::Icelandic, Language::Igbo, Language::Indonesian, Language::Irish, Language::Italian, Language::Japanese, Language::Javanese, Language::Kannada, Language::Kazakh, Language::Korean, Language::Lao, Language::Latvian, Language::Lithuanian, Language::Luxembourgish, Language::Macedonian, Language::Malagasy, Language::Malay, Language::Malayalam,  Language::Marathi, Language::Mongolian, Language::Nepali, Language::Norwegian, Language::Pashto,  Language::Polish, Language::Portuguese,  Language::Romanian, Language::Russian,   Language::Serbian,   Language::Sindhi, Language::Sinhala, Language::Slovak, Language::Slovenian, Language::Somali, Language::Spanish, Language::Sundanese ];
    let target_languages = vec![Language::Afrikaans, Language::Albanian, Language::Amharic, Language::Arabic, Language::Armenian, Language::Azerbaijani, Language::Belarusian, Language::Bengali, Language::Bosnian, Language::Bulgarian, Language::Catalan, Language::Cebuano, Language::Croatian, Language::Czech, Language::Danish, Language::Dutch, Language::English,  Language::Estonian,  Language::Finnish, Language::French,  Language::Galician, Language::Georgian, Language::German, Language::Greek, Language::Gujarati, Language::HaitianCreole, Language::Hausa,  Language::Hebrew, Language::Hindi,  Language::Hungarian, Language::Icelandic, Language::Igbo, Language::Indonesian, Language::Irish, Language::Italian, Language::Japanese, Language::Javanese, Language::Kannada, Language::Kazakh, Language::Korean, Language::Lao, Language::Latvian, Language::Lithuanian, Language::Luxembourgish, Language::Macedonian, Language::Malagasy, Language::Malay, Language::Malayalam,  Language::Marathi, Language::Mongolian, Language::Nepali, Language::Norwegian, Language::Pashto,  Language::Polish, Language::Portuguese,  Language::Romanian, Language::Russian,   Language::Serbian,   Language::Sindhi, Language::Sinhala, Language::Slovak, Language::Slovenian, Language::Somali, Language::Spanish, Language::Sundanese ];
    let translation_model =
        tokio::task::spawn_blocking(|| -> Result<TranslationModel, RustBertError> {
            let models: TranslationModel = TranslationModelBuilder::new().with_source_languages(languages).with_target_languages(target_languages)
                .create_model()?;
            Ok(models)
        });
    let result = translation_model.await.unwrap();
    match result {
        Ok(model) => model,
        Err(error) => panic!("Problem getting pos: {:?}", error),
    }
}
pub async fn load_marian() -> TranslationModel {
    
    let languages = vec![Language::English];
    let target_languages = vec![Language::French, Language::Spanish];
    let translation_model =
        tokio::task::spawn_blocking(|| -> Result<TranslationModel, RustBertError> {
            let models: TranslationModel = TranslationModelBuilder::new().with_model_type(ModelType::Marian).with_source_languages(languages).with_target_languages(target_languages)
                .create_model()?;
            Ok(models)
        });
    let result = translation_model.await.unwrap();
    match result {
        Ok(model) => model,
        Err(error) => panic!("Problem getting pos: {:?}", error),
    }
}
pub fn translate(
    translator: TranslationModel,
    input: &str,
    // source_language: &str,
    // target_language: &str,
) -> Result<Vec<String>, RustBertError> {
    translator.translate(&[input], None, Language::English)
}


