use language_convertor::Convertor;
use rocket::futures::lock::Mutex;

use lingua::LanguageDetector;
use rocket::{post, launch, Build, routes, State, Rocket, serde::json::Json};
use rust_bert::pipelines::translation::{TranslationModel, Language};
use serde::{Deserialize, Serialize};
use crate::language_detector::detect;
use rust_bert::pipelines::sentiment::SentimentPolarity;
// use crate::language_convertor;

mod language_detector;
mod language_convertor;
mod translation;
mod sentiment;

#[launch]
async fn rocket() -> Rocket<Build> {
    let models = Mutex::new(Models {
    
        // pos: pos::load_pos_model().await,
        // corrector: NlpRuleChecker::new(),
        convertor: Convertor::load(),
        language_detect: language_detector::load(),
        translation: translation::load().await,
        sentiment: sentiment::Sentiment::load().await,
        // question_answering: question_answering::load().await,
        // summarize: summarization::load().await, // symspell: spell_check::load(),
        // conversation_module: ConvModule::new().await,
    });
    
    rocket::build()
        .manage(models)
        .mount(
            "/",
            routes![
                // question_answering_endpoint,
                // pos_endpoint,
                detect_language_endpoint,
                translation_endpoint,
                sentiment_endpoint,
                // summarize_endpoint,
                // nlp,
                // dialog_endpoint
            ],
        )
    // .mount("/", routes![nlp])
}

#[post("/sentiment", format = "json", data = "<input>")]
async fn sentiment_endpoint<'a>(
    model: &'a State<Mutex<Models>>,
    input: Json<SentimentInput>,
) -> Json<SentimentOutput> {
    println!("Request - Input : {}", &input.input);
    let language: SentimentOutput =  model
        .lock()
        .await.sentiment.predict(&input.input);
    Json(language)
}
#[post("/language", format = "json", data = "<input>")]
async fn detect_language_endpoint<'a>(
    model: &'a State<Mutex<Models>>,
    input: Json<LanguageDetectionInput>,
) -> Json<TranslationOutput> {
    println!("Request - Input : {}", &input.input);
    let language =  detect(&model
        .lock()
        .await

        .language_detect, &input.input);
    Json(TranslationOutput { language })
}
#[post("/translation", format = "json", data = "<input>")]
async fn translation_endpoint(
    model: &State<Mutex<Models>>,
    input: Json<TranslationInput>,
) -> Json<Vec<std::string::String>> {
    println!("Request - Input : {}", &input.input);
    let target = model
    .lock()
    .await.convertor.get_language(&input.target).bert_language;
let origin = model
    .lock()
    .await.convertor.get_language(&input.origin).bert_language;
println!("Target : {:?}", &target);
if (origin == Language::English && (target == Language::Spanish || target == Language::French)) { 
    let input_translated =
            model
                .lock()
                .await
                .translation.marian
                .translate(&[&input.input], origin, target );
            println!("Input translated : {:?}", &input_translated);
        let result = match input_translated {
            Ok(translation) => translation,
            Err(error) => panic!("Problem getting translation: {:?}", error),
        };
        
        return Json(result)
}else  {
    let input_translated =
            model
                .lock()
                .await
                .translation.m2m_100
                .translate(&[&input.input], origin, target );
            println!("Input translated : {:?}", &input_translated);
        let result = match input_translated {
            Ok(translation) => translation,
            Err(error) => panic!("Problem getting translation: {:?}", error),
        };
        return Json(result)
}
    
}

pub struct Models {
    // pos: POSModel,
    // corrector: NlpRuleChecker,
    convertor: language_convertor::Convertor,
    language_detect: LanguageDetector,
    translation: TranslationModels,
    // question_answering: QuestionAnsweringModel,
    // summarize: SummarizationModel,
    // conversation_module: ConvModule,
    sentiment: sentiment::Sentiment,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct SentimentInput {
    input: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct LanguageDetectionInput {
    input: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TranslationInput {
    input: String,
    target: String,
    origin: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TranslationOutput {
    language: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SentimentOutput {
    confidence: f64,
    sentiment: String ,
}

pub struct TranslationModels {
    pub marian: TranslationModel,
    pub m2m_100: TranslationModel,
}