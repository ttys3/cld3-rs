#[cxx::bridge]
mod ffi {

    #[derive(Debug, PartialEq)]
    pub struct SharedResult {
        pub language: String,
        pub probability: f64, // Language probability.
        pub is_reliable: bool,  // Whether the prediction is reliable.
        pub proportion: f64, // Proportion of bytes in the input that were identified as this language.
    }

    unsafe extern "C++" {
        include!("cld3/cld3.h");

        pub(crate) type NNetLanguageIdentifier;

        fn new_language_identifier_default() -> UniquePtr<NNetLanguageIdentifier>;

        fn new_language_identifier(min_num_bytes: i32, max_num_bytes: i32) -> Result<UniquePtr<NNetLanguageIdentifier>>;

        fn find_language(li: Pin<&mut NNetLanguageIdentifier>, text: &str) -> SharedResult;

        fn find_topn_most_freq_langs(li: Pin<&mut NNetLanguageIdentifier>, text: &str, num_langs: i32) -> Vec<SharedResult>;
    }
}

// warning!!! DO NOT USE `Result` from the cld3 C++ code
pub type DetectResult = ffi::SharedResult;

pub struct NNetLanguageIdentifier{
    pub cxxp: cxx::UniquePtr<ffi::NNetLanguageIdentifier>,
}

impl NNetLanguageIdentifier {
    pub fn new(min_num_bytes: i32, max_num_bytes: i32) -> Result<Self, cxx::Exception> {
        Ok(Self { cxxp: ffi::new_language_identifier(min_num_bytes, max_num_bytes)? })
    }

    pub fn find_language(&mut self, text: &str) -> DetectResult {
        ffi::find_language(self.cxxp.pin_mut(), text)
    }

    pub fn find_topn_most_freq_langs(&mut self, text: &str, num_langs: i32) -> Vec<DetectResult> {
        ffi::find_topn_most_freq_langs(self.cxxp.pin_mut(), text, num_langs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn implicit_constructor() {
        ffi::new_language_identifier_default();
    }

    #[test]
    fn constructor_with_param() {
        let mut ld = ffi::new_language_identifier(0, 500).unwrap();
        let rs = ffi::find_language(ld.pin_mut(), "こんにちは");
        assert_eq!(rs.language, "ja");

        let mut rustic_ld = NNetLanguageIdentifier::new(0, 500).unwrap();
        let rs = rustic_ld.find_language("こんにちは");
        assert_eq!(rs.language, "ja");
    }

    #[test]
    fn find_language() {
        let mut ld = NNetLanguageIdentifier::new(0, 500).unwrap();
        // multi lang and lang code test cases in format: (text, lang_code)
        let test_cases = vec![
            ("This is English text.", "en"),
            ("Dies ist deutscher Text.", "de"),
            ("Detta är svenska text.", "sv"),
            ("これは日本語のテキストです。", "ja"),
            ("이것은 한국어 텍스트입니다.", "ko"),
            ("這是中文文本。", "zh"),
            ("Toto je český text.", "cs"),
            ("Tämä on suomen kielen tekstiä.", "fi"),
            ("Αυτό είναι ελληνικό κείμενο.", "el"),
            ("Ez magyar szöveg.", "hu"),
            ("Ini adalah teks Bahasa Indonesia.", "id"),
            ("Questo è un testo italiano.", "it"),

            // cld3 can not pass below cases ...
            // ("Hola", "es"),
            // ("bonjour", "fr"),
            // ("ciao", "it"),
        ];
        for (text, expected) in test_cases {
            let rs = ld.find_language(text);
            println!("real: {}, expected: {}", rs.language, expected);
            assert_eq!(rs.language, expected);
        }
    }

    #[test]
    fn find_topn_most_freq_langs() {
        let mut ld = NNetLanguageIdentifier::new(0, 100).unwrap();
        // can't believe cld3 can not detect "Hola, bonjour, ciao" as es, fr, it
        let test_cases = vec![
            ("你好 こんにちは 안녕하세요", vec!["zh", "ja", "ko"]),
            ("Hola, bonjour, ciao", vec!["es", "fr", "it"]),
        ];
        for (text, expected_lang_codes) in test_cases {
            // Act
            let result = ld.find_topn_most_freq_langs(text, expected_lang_codes.len() as i32);

            // Assert len
            // println!("actual len: {:?}, expected len: {:?}", result.len(), expected_lang_codes.len());
            assert_eq!(result.len(), expected_lang_codes.len()); // Expecting the same number of languages in the result

            // check all value of result[i].language is in expected_lang_codes
            let detected_lang_codes: Vec<&str> = result.iter().map(|res| res.language.as_str()).collect();
            for lang_code in &detected_lang_codes {
                println!("text: {:?}, check lang: {:?}, detected langs: {:?}, should in array:{:?}",
                         text, &lang_code, &detected_lang_codes, &expected_lang_codes);
                assert!(expected_lang_codes.contains(&lang_code)); // Expecting the language to be in the expected_lang_codes
            }
        }
    }
}
