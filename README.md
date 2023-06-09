## cld3-rs

[![crates.io](https://badgen.net/crates/v/cld3)](https://crates.io/crates/cld3)
[![docs.rs](https://docs.rs/cld3/badge.svg)](https://docs.rs/cld3)
![downloads](https://badgen.net/crates/d/cld3)
![latest version downloads](https://badgen.net/crates/dl/cld3)

Rust binding for Compact Language Detector v3 (CLD3), a neural network model for language identification.

the C++ code is from https://github.com/google/cld3

## Usage

`libprotobuf` is required to build this crate.

```rust
let mut cld = cld3::NNetLanguageIdentifier::new(0, 500).unwrap();
let rs = cld.find_language("こんにちは");
println!("{:?}", rs.language);
// ja
```
### Compact Language Detector v3 (CLD3)

* [Model](#model)
* [Supported Languages](#supported-languages)

### Model

CLD3 is a neural network model for language identification. This package
contains the inference code and a trained model. The inference code
extracts character ngrams from the input text and computes the fraction
of times each of them appears. For example, as shown in the figure below,
if the input text is "banana", then one of the extracted trigrams is "ana"
and the corresponding fraction is 2/4. The ngrams are hashed down to an id
within a small range, and each id is represented by a dense embedding vector
estimated during training.

The model averages the embeddings corresponding to each ngram type according
to the fractions, and the averaged embeddings are concatenated to produce
the embedding layer. The remaining components of the network are a hidden
(Rectified linear) layer and a softmax layer.

To get a language prediction for the input text, we simply perform a forward
pass through the network.

![Figure](https://github.com/google/cld3/raw/master/model.png "CLD3")

### Supported Languages

The model outputs BCP-47-style language codes, shown in the table below. For
some languages, output is differentiated by script. Language and script names
from
[Unicode CLDR](https://github.com/unicode-cldr/cldr-localenames-modern/blob/master/main/en).

Output Code | Language Name   | Script Name
----------- | --------------- | ------------------------------------------
af          | Afrikaans       | Latin
am          | Amharic         | Ethiopic
ar          | Arabic          | Arabic
bg          | Bulgarian       | Cyrillic
bg-Latn     | Bulgarian       | Latin
bn          | Bangla          | Bangla
bs          | Bosnian         | Latin
ca          | Catalan         | Latin
ceb         | Cebuano         | Latin
co          | Corsican        | Latin
cs          | Czech           | Latin
cy          | Welsh           | Latin
da          | Danish          | Latin
de          | German          | Latin
el          | Greek           | Greek
el-Latn     | Greek           | Latin
en          | English         | Latin
eo          | Esperanto       | Latin
es          | Spanish         | Latin
et          | Estonian        | Latin
eu          | Basque          | Latin
fa          | Persian         | Arabic
fi          | Finnish         | Latin
fil         | Filipino        | Latin
fr          | French          | Latin
fy          | Western Frisian | Latin
ga          | Irish           | Latin
gd          | Scottish Gaelic | Latin
gl          | Galician        | Latin
gu          | Gujarati        | Gujarati
ha          | Hausa           | Latin
haw         | Hawaiian        | Latin
hi          | Hindi           | Devanagari
hi-Latn     | Hindi           | Latin
hmn         | Hmong           | Latin
hr          | Croatian        | Latin
ht          | Haitian Creole  | Latin
hu          | Hungarian       | Latin
hy          | Armenian        | Armenian
id          | Indonesian      | Latin
ig          | Igbo            | Latin
is          | Icelandic       | Latin
it          | Italian         | Latin
iw          | Hebrew          | Hebrew
ja          | Japanese        | Japanese
ja-Latn     | Japanese        | Latin
jv          | Javanese        | Latin
ka          | Georgian        | Georgian
kk          | Kazakh          | Cyrillic
km          | Khmer           | Khmer
kn          | Kannada         | Kannada
ko          | Korean          | Korean
ku          | Kurdish         | Latin
ky          | Kyrgyz          | Cyrillic
la          | Latin           | Latin
lb          | Luxembourgish   | Latin
lo          | Lao             | Lao
lt          | Lithuanian      | Latin
lv          | Latvian         | Latin
mg          | Malagasy        | Latin
mi          | Maori           | Latin
mk          | Macedonian      | Cyrillic
ml          | Malayalam       | Malayalam
mn          | Mongolian       | Cyrillic
mr          | Marathi         | Devanagari
ms          | Malay           | Latin
mt          | Maltese         | Latin
my          | Burmese         | Myanmar
ne          | Nepali          | Devanagari
nl          | Dutch           | Latin
no          | Norwegian       | Latin
ny          | Nyanja          | Latin
pa          | Punjabi         | Gurmukhi
pl          | Polish          | Latin
ps          | Pashto          | Arabic
pt          | Portuguese      | Latin
ro          | Romanian        | Latin
ru          | Russian         | Cyrillic
ru-Latn     | Russian         | English
sd          | Sindhi          | Arabic
si          | Sinhala         | Sinhala
sk          | Slovak          | Latin
sl          | Slovenian       | Latin
sm          | Samoan          | Latin
sn          | Shona           | Latin
so          | Somali          | Latin
sq          | Albanian        | Latin
sr          | Serbian         | Cyrillic
st          | Southern Sotho  | Latin
su          | Sundanese       | Latin
sv          | Swedish         | Latin
sw          | Swahili         | Latin
ta          | Tamil           | Tamil
te          | Telugu          | Telugu
tg          | Tajik           | Cyrillic
th          | Thai            | Thai
tr          | Turkish         | Latin
uk          | Ukrainian       | Cyrillic
ur          | Urdu            | Arabic
uz          | Uzbek           | Latin
vi          | Vietnamese      | Latin
xh          | Xhosa           | Latin
yi          | Yiddish         | Hebrew
yo          | Yoruba          | Latin
zh          | Chinese         | Han (including Simplified and Traditional)
zh-Latn     | Chinese         | Latin
zu          | Zulu            | Latin


## similar works

Natural language detection library for Rust.  https://github.com/greyblake/whatlang-rs

Translation of Google's cld3 language detector into Rust. Just for fun. https://github.com/edvakf/langdetect