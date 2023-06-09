
#include "cld3_wrapper.h"

// required for shared struct `SharedResult`
#include "cld3/src/lib.rs.h"

#include <string>
#include "cld3/base.h"

using chrome_lang_id::NNetLanguageIdentifier;

std::unique_ptr<NNetLanguageIdentifier> new_language_identifier_default() {
    return std::make_unique<chrome_lang_id::NNetLanguageIdentifier>();
}

std::unique_ptr<NNetLanguageIdentifier> new_language_identifier(int minNumBytes, int maxNumBytes) {
    return std::make_unique<chrome_lang_id::NNetLanguageIdentifier>(minNumBytes, maxNumBytes);
}

SharedResult find_language(NNetLanguageIdentifier &lang_id, rust::Str text) {
  const NNetLanguageIdentifier::Result res = lang_id.FindLanguage(std::string(text));
  SharedResult out{};
  out.language = res.language;
  out.probability = res.probability;
  out.is_reliable = res.is_reliable;
  out.proportion = res.proportion;
  return out;
}


rust::Vec<SharedResult> find_topn_most_freq_langs(NNetLanguageIdentifier &li, rust::Str text, int num_langs) {
    const std::vector<NNetLanguageIdentifier::Result> res = li.FindTopNMostFreqLangs(std::string(text), num_langs);

    rust::Vec<SharedResult> results;

    for (size_t i = 0; i < res.size(); i++) {
        SharedResult out{};
        out.language = res[i].language;
        out.probability = res[i].probability;
        out.is_reliable = res[i].is_reliable;
        out.proportion = res[i].proportion;
        results.push_back(out);
    }
    return results;
}