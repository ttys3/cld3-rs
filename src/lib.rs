#[cxx::bridge]
mod ffi {

    struct SharedResult {
        language: String,
        probability: f64, // Language probability.
        is_reliable: bool,  // Whether the prediction is reliable.
        proportion: f64, // Proportion of bytes in the input that were identified as this language.
    }

    unsafe extern "C++" {
        include!("cld3/cld3.h");

        pub(crate) type NNetLanguageIdentifier;

        fn new_language_identifier_default() -> UniquePtr<NNetLanguageIdentifier>;

        // fn find_language(li: Pin<&mut NNetLanguageIdentifier>, text: &str) -> SharedResult;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        ffi::new_language_identifier_default();
    }
}
