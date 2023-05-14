use lingua::Language as LinguaLanguage;
use rust_bert::pipelines::translation::{Language as BertLanguage};
#[derive(Debug)]
pub struct Language {
    pub lingua_language: LinguaLanguage,
    pub bert_language: BertLanguage,
    pub iso_code_639_1: String,
    pub iso_code_639_3: String,
}


pub struct Convertor {
    languages: Vec<Language>,
}

impl Convertor {
    pub fn load() -> Convertor {
        Convertor {languages : vec![
            Language {
                lingua_language: LinguaLanguage::Afrikaans,
                bert_language: BertLanguage::Afrikaans,
                iso_code_639_1: "af".to_string(),
                iso_code_639_3: "afr".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Albanian,
                bert_language: BertLanguage::Albanian,
                iso_code_639_1: "sq".to_string(),
                iso_code_639_3: "sqi".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Arabic,
                bert_language: BertLanguage::Arabic,
                iso_code_639_1: "ar".to_string(),
                iso_code_639_3: "ara".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Armenian,
                bert_language: BertLanguage::Armenian,
                iso_code_639_1: "hy".to_string(),
                iso_code_639_3: "hye".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Azerbaijani,
                bert_language: BertLanguage::Azerbaijani,
                iso_code_639_1: "az".to_string(),
                iso_code_639_3: "aze".to_string(),
            },
            // Language {
            //     lingua_language: LinguaLanguage::Basque,
            //     bert_language: BertLanguage::Basque,
            //     iso_code_639_1: "eu".to_string(),
            //     iso_code_639_3: "eus".to_string(),
            // },
            Language {
                lingua_language: LinguaLanguage::Belarusian,
                bert_language: BertLanguage::Belarusian,
                iso_code_639_1: "be".to_string(),
                iso_code_639_3: "bel".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Bengali,
                bert_language: BertLanguage::Bengali,
                iso_code_639_1: "bn".to_string(),
                iso_code_639_3: "ben".to_string(),
            },
            // Language {
            //     lingua_language: LinguaLanguage::Bokmal,
            //     bert_language: BertLanguage::Bokmal,
            //     iso_code_639_1: "nb".to_string(),
            //     iso_code_639_3: "nob".to_string(),
            // },
            Language {
                lingua_language: LinguaLanguage::Bosnian,
                bert_language: BertLanguage::Bosnian,
                iso_code_639_1: "bs".to_string(),
                iso_code_639_3: "bos".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Bulgarian,
                bert_language: BertLanguage::Bulgarian,
                iso_code_639_1: "bg".to_string(),
                iso_code_639_3: "bul".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Catalan,
                bert_language: BertLanguage::Catalan,
                iso_code_639_1: "ca".to_string(),
                iso_code_639_3: "cat".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Chinese,
                bert_language: BertLanguage::ChineseMandarin,
                iso_code_639_1: "zh".to_string(),
                iso_code_639_3: "zho".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Croatian,
                bert_language: BertLanguage::Croatian,
                iso_code_639_1: "hr".to_string(),
                iso_code_639_3: "hrv".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Czech,
                bert_language: BertLanguage::Czech,
                iso_code_639_1: "cs".to_string(),
                iso_code_639_3: "ces".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Danish,
                bert_language: BertLanguage::Danish,
                iso_code_639_1: "da".to_string(),
                iso_code_639_3: "dan".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Dutch,
                bert_language: BertLanguage::Dutch,
                iso_code_639_1: "nl".to_string(),
                iso_code_639_3: "nld".to_string(),
            },

            
            Language {
                lingua_language: LinguaLanguage::English,
                bert_language: BertLanguage::English,
                iso_code_639_1: "en".to_string(),
                iso_code_639_3: "eng".to_string(),
            },
            // Language {
            //     lingua_language: LinguaLanguage::Esperanto,
            //     bert_language: BertLanguage::Esperant,
            //     iso_code_639_1: "eo".to_string(),
            //     iso_code_639_3: "epo".to_string(),
            // },
            Language {
                lingua_language: LinguaLanguage::Estonian,
                bert_language: BertLanguage::Estonian,
                iso_code_639_1: "et".to_string(),
                iso_code_639_3: "est".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Finnish,
                bert_language: BertLanguage::Finnish,
                iso_code_639_1: "fi".to_string(),
                iso_code_639_3: "fin".to_string(),
            },
            
            Language {
                lingua_language: LinguaLanguage::French,
                bert_language: BertLanguage::French,
                iso_code_639_1: "fr".to_string(),
                iso_code_639_3: "fra".to_string(),
            },
            // Language {
            //     lingua_language: LinguaLanguage::Ganda,
            //     bert_language: BertLanguage::Galician,
            //     iso_code_639_1: "gl".to_string(),
            //     iso_code_639_3: "glg".to_string(),
            // },
            Language {
                lingua_language: LinguaLanguage::Georgian,
                bert_language: BertLanguage::Georgian,
                iso_code_639_1: "ka".to_string(),
                iso_code_639_3: "kat".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::German,
                bert_language: BertLanguage::German,
                iso_code_639_1: "de".to_string(),
                iso_code_639_3: "deu".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Greek,
                bert_language: BertLanguage::Greek,
                iso_code_639_1: "el".to_string(),
                iso_code_639_3: "ell".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Gujarati,
                bert_language: BertLanguage::Gujarati,
                iso_code_639_1: "gu".to_string(),
                iso_code_639_3: "guj".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Hebrew,
                bert_language: BertLanguage::Hebrew,
                iso_code_639_1: "he".to_string(),
                iso_code_639_3: "heb".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Hindi,
                bert_language: BertLanguage::Hindi,
                iso_code_639_1: "hi".to_string(),
                iso_code_639_3: "hin".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Hungarian,
                bert_language: BertLanguage::Hungarian,
                iso_code_639_1: "hu".to_string(),
                iso_code_639_3: "hun".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Icelandic,
                bert_language: BertLanguage::Icelandic,
                iso_code_639_1: "is".to_string(),
                iso_code_639_3: "isl".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Indonesian,
                bert_language: BertLanguage::Indonesian,
                iso_code_639_1: "id".to_string(),
                iso_code_639_3: "ind".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Irish,
                bert_language: BertLanguage::Irish,
                iso_code_639_1: "ga".to_string(),
                iso_code_639_3: "gle".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Italian,
                bert_language: BertLanguage::Italian,
                iso_code_639_1: "it".to_string(),
                iso_code_639_3: "ita".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Japanese,
                bert_language: BertLanguage::Japanese,
                iso_code_639_1: "ja".to_string(),
                iso_code_639_3: "jpn".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Kazakh,
                bert_language: BertLanguage::Kazakh,
                iso_code_639_1: "kk".to_string(),
                iso_code_639_3: "kaz".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Korean,
                bert_language: BertLanguage::Korean,
                iso_code_639_1: "ko".to_string(),
                iso_code_639_3: "kor".to_string(),
            },
            // Language {
            //     lingua_language: LinguaLanguage::Kurdish,
            //     bert_language: BertLanguage::Kurdish,
            //     iso_code_639_1: "ku".to_string(),
            //     iso_code_639_3: "kur".to_string(),
            // },
            Language {
                lingua_language: LinguaLanguage::Latvian,
                bert_language: BertLanguage::Latvian,
                iso_code_639_1: "lv".to_string(),
                iso_code_639_3: "lav".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Lithuanian,
                bert_language: BertLanguage::Lithuanian,
                iso_code_639_1: "lt".to_string(),
                iso_code_639_3: "lit".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Macedonian,
                bert_language: BertLanguage::Macedonian,
                iso_code_639_1: "mk".to_string(),
                iso_code_639_3: "mkd".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Malay,
                bert_language: BertLanguage::Malay,
                iso_code_639_1: "ms".to_string(),
                iso_code_639_3: "msa".to_string(),
            },
            // Language {
            //     lingua_language: LinguaLanguage::Malayalam,
            //     bert_language: BertLanguage::Malayalam,
            //     iso_code_639_1: "ml".to_string(),
            //     iso_code_639_3: "mal".to_string(),
            // },
            Language {
                lingua_language: LinguaLanguage::Marathi,
                bert_language: BertLanguage::Marathi,
                iso_code_639_1: "mr".to_string(),
                iso_code_639_3: "mar".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Mongolian,
                bert_language: BertLanguage::Mongolian,
                iso_code_639_1: "mn".to_string(),
                iso_code_639_3: "mon".to_string(),
            },
            // Language {
            //     lingua_language: LinguaLanguage::Norwegian,
            //     bert_language: BertLanguage::Norwegian,
            //     iso_code_639_1: "no".to_string(),
            //     iso_code_639_3: "nor".to_string(),
            // },
            // Language {
            //     lingua_language: LinguaLanguage::Persian,
            //     bert_language: BertLanguage::Pe,
            //     iso_code_639_1: "fa".to_string(),
            //     iso_code_639_3: "fas".to_string(),
            // },
            Language {
                lingua_language: LinguaLanguage::Polish,
                bert_language: BertLanguage::Polish,
                iso_code_639_1: "pl".to_string(),
                iso_code_639_3: "pol".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Portuguese,
                bert_language: BertLanguage::Portuguese,
                iso_code_639_1: "pt".to_string(),
                iso_code_639_3: "por".to_string(),
            },
            // Language {
            //     lingua_language: LinguaLanguage::Punjabi,
            //     bert_language: BertLanguage::Punjabi,
            //     iso_code_639_1: "pa".to_string(),
            //     iso_code_639_3: "pan".to_string(),
            // },
            Language {
                lingua_language: LinguaLanguage::Romanian,
                bert_language: BertLanguage::Romanian,
                iso_code_639_1: "ro".to_string(),
                iso_code_639_3: "ron".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Russian,
                bert_language: BertLanguage::Russian,
                iso_code_639_1: "ru".to_string(),
                iso_code_639_3: "rus".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Serbian,
                bert_language: BertLanguage::Serbian,
                iso_code_639_1: "sr".to_string(),
                iso_code_639_3: "srp".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Slovak,
                bert_language: BertLanguage::Slovak,
                iso_code_639_1: "sk".to_string(),
                iso_code_639_3: "slk".to_string(),
            },
            // Language {
            //     lingua_language: LinguaLanguage::Slovene,
            //     bert_language: BertLanguage::Slovene,
            //     iso_code_639_1: "sl".to_string(),
            //     iso_code_639_3: "slv".to_string(),
            // },
            Language {
                lingua_language: LinguaLanguage::Somali,
                bert_language: BertLanguage::Somali,
                iso_code_639_1: "so".to_string(),
                iso_code_639_3: "som".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Spanish,
                bert_language: BertLanguage::Spanish,
                iso_code_639_1: "es".to_string(),
                iso_code_639_3: "spa".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Swahili,
                bert_language: BertLanguage::Swahili,
                iso_code_639_1: "sw".to_string(),
                iso_code_639_3: "swa".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Swedish,
                bert_language: BertLanguage::Swedish,
                iso_code_639_1: "sv".to_string(),
                iso_code_639_3: "swe".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Tamil,
                bert_language: BertLanguage::Tamil,
                iso_code_639_1: "ta".to_string(),
                iso_code_639_3: "tam".to_string(),
            },
            // Language {
            //     lingua_language: LinguaLanguage::Telugu,
            //     bert_language: BertLanguage::Telugu,
            //     iso_code_639_1: "te".to_string(),
            //     iso_code_639_3: "tel".to_string(),
            // },
            Language {
                lingua_language: LinguaLanguage::Thai,
                bert_language: BertLanguage::Thai,
                iso_code_639_1: "th".to_string(),
                iso_code_639_3: "tha".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Turkish,
                bert_language: BertLanguage::Turkish,
                iso_code_639_1: "tr".to_string(),
                iso_code_639_3: "tur".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Ukrainian,
                bert_language: BertLanguage::Ukrainian,
                iso_code_639_1: "uk".to_string(),
                iso_code_639_3: "ukr".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Urdu,
                bert_language: BertLanguage::Urdu,
                iso_code_639_1: "ur".to_string(),
                iso_code_639_3: "urd".to_string(),
            },
            // Language {
            //     lingua_language: LinguaLanguage::Uzbek,
            //     bert_language: BertLanguage::Uzbek,
            //     iso_code_639_1: "uz".to_string(),
            //     iso_code_639_3: "uzb".to_string(),
            // },
            Language {
                lingua_language: LinguaLanguage::Vietnamese,
                bert_language: BertLanguage::Vietnamese,
                iso_code_639_1: "vi".to_string(),
                iso_code_639_3: "vie".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Welsh,
                bert_language: BertLanguage::Welsh,
                iso_code_639_1: "cy".to_string(),
                iso_code_639_3: "cym".to_string(),
            },
            Language {
                lingua_language: LinguaLanguage::Xhosa,
                bert_language: BertLanguage::Xhosa,
                iso_code_639_1: "xh".to_string(),
                iso_code_639_3: "xho".to_string(),
            },

            
        ],
        }

    }
    pub fn get_language(&self, input: &String) -> &Language {
        for language in self.languages.as_slice() {
            if &language.lingua_language.to_string() == input || &language.iso_code_639_3 == input || &language.iso_code_639_1 == input || &language.iso_code_639_3 == input {
                return language;
            }
        }
        // panic!("Language not found");
        return self.get_language(&"en".to_string());
    }
}
