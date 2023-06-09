#pragma once
#include "rust/cxx.h"
#include <memory>

#include "cld3/nnet_language_identifier.h"

#include <stdbool.h>
#include <stdlib.h>
#include <string>
#include <string.h>
#include <stdexcept>

// ref https://github.com/brson/wasm-opt-rs/blob/66b161e294bb332947f8319993ae1f8d3498e1e8/components/wasm-opt-cxx-sys/src/shims.h#L13
// https://brson.github.io/2022/10/26/creating-wasm-opt-rust-bindings-with-cxx
// https://github.com/dtolnay/cxx/pull/74/files#diff-b43c1d065c83e99920c09c2d8dbed19687b44a3aeb8e1400a6f5228064a3629f
// https://cxx.rs/binding/result.html
namespace rust::behavior {
    template <typename Try, typename Fail>
    static void trycatch(Try &&func, Fail &&fail) noexcept try {
        func();
    } catch (const std::exception &e) {
        fail(e.what());
    }
}

using namespace chrome_lang_id;

struct SharedResult;

std::unique_ptr<NNetLanguageIdentifier> new_language_identifier_default();
std::unique_ptr<NNetLanguageIdentifier> new_language_identifier(int minNumBytes, int maxNumBytes);

SharedResult find_language(NNetLanguageIdentifier &li, rust::Str text);

rust::Vec<SharedResult> find_topn_most_freq_langs(NNetLanguageIdentifier &li, rust::Str text, int num_langs);

