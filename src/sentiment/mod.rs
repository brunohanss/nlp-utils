
use rocket::tokio;
use rust_bert::{pipelines::sentiment::{SentimentModel, SentimentPolarity}, RustBertError};

use crate::SentimentOutput;
pub struct Sentiment {
    model: SentimentModel
}

impl Sentiment {
    pub async fn load() -> Sentiment { 
        
        let sentiment_model =
        tokio::task::spawn_blocking(|| -> Result<SentimentModel, RustBertError> {
            let models= SentimentModel::new(Default::default());
            Ok(models.unwrap())
        });
    let result = sentiment_model.await.unwrap().unwrap();
    // match result {
    //     Ok(model) => model,
    //     Err(error) => panic!("Problem getting pos: {:?}", error),
    // }
    Sentiment { model: result } }

    
    pub fn predict(&self, input: &str) -> SentimentOutput {
        let sentiment = self.model.predict(vec![input] );
        println!("Sentiment: {:?}", sentiment);
        SentimentOutput { confidence: sentiment[0].score, sentiment: get_polarity_as_string(&sentiment[0].polarity)  }
    }

    
}
pub fn get_polarity_as_string(polarity: &SentimentPolarity) -> String {
    match polarity {
        SentimentPolarity::Positive => "Positive".to_string(),
        SentimentPolarity::Negative => "Negative".to_string(),
    }
}
