
#include "cld3.h"
#include <string>
#include "cld3/base.h"

using chrome_lang_id::NNetLanguageIdentifier;

std::unique_ptr<NNetLanguageIdentifier> new_language_identifier_default() {
    return std::make_unique<chrome_lang_id::NNetLanguageIdentifier>();
}

std::unique_ptr<NNetLanguageIdentifier> new_language_identifier(int minNumBytes, int maxNumBytes) {
    return std::make_unique<chrome_lang_id::NNetLanguageIdentifier>(minNumBytes, maxNumBytes);
}

//SharedResult find_language(NNetLanguageIdentifier &lang_id, rust::Str text) {
//  const NNetLanguageIdentifier::Result res = lang_id.FindLanguage(std::string(text));
//  SharedResult out;
//  out.language = res.language;
//  out.probability = res.probability;
//  out.is_reliable = res.is_reliable;
//  out.proportion = res.proportion;
//  return out;
//}


//const Result* find_topn_most_freq_langs(CLanguageIdentifier li, char *data, int length, int num_langs, int *out_size) {
//    NNetLanguageIdentifier* lang_id = (NNetLanguageIdentifier*)li;
//    std::string text(data, length);
//    const std::vector<NNetLanguageIdentifier::Result> res = lang_id->FindTopNMostFreqLangs(text, num_langs);
//
//    *out_size = res.size();
//
//    if (res.size() == 0) {
//        return NULL;
//    }
//    Result* out = new Result[res.size()];
//    for (int i = 0; i < res.size(); i++) {
//        out[i].language = new char[res[i].language.length() + 1];
//        strcpy(out[i].language, res[i].language.c_str());
//        out[i].len_language = res[i].language.length();
//        out[i].probability = res[i].probability;
//        out[i].is_reliable = res[i].is_reliable;
//        out[i].proportion = res[i].proportion;
//    }
//    return out;
//}