use rand::{
    SeedableRng, TryRngCore,
    distr::{Distribution, slice::Choose},
    rngs::OsRng,
};
use rand_chacha::ChaCha20Rng;

pub fn generate_random_chars_from_char_vec(
    chars: Vec<char>,
    num_chars: usize,
    rng: &mut ChaCha20Rng,
) -> Vec<char> {
    Choose::new(&chars)
        .unwrap()
        .sample_iter(rng)
        .take(num_chars)
        .map(|c| *c)
        .collect()
}

pub fn generate_rng() -> ChaCha20Rng {
    let mut seed = [0u8; 32];
    if let Err(e) = OsRng.try_fill_bytes(&mut seed) {
        eprintln!("Failed to generate seed: {e}");
        std::process::exit(1);
    }
    ChaCha20Rng::from_seed(seed)
}

// pub fn get_category_name(cp: CodePoint) -> String {
//     let (gc, blk, id) = (cp.gc(), cp.blk(), cp.to_u32());
//     String::from(match (gc, blk, id) {
//         (Gc::Lu, Blk::Ascii, ..) => "A-Z",
//         (Gc::Ll, Blk::Ascii, ..) => "a-z",
//         (Gc::Nd, Blk::Ascii, ..) => "0-9",
//         (
//             Gc::Po | Gc::Sc | Gc::Sk,
//             Blk::Ascii,
//             0x21 | 0x23 | 0x24 | 0x25 | 0x26 | 0x2a | 0x40 | 0x5e,
//         ) => "!@*&#%^$",
//         (.., 0x20) => "Space",
//         (_, Blk::Ascii, _) => "Other ASCII Punctuation",
//         (
//             Gc::Lu,
//             Blk::Latin1Sup
//             | Blk::LatinExtA
//             | Blk::LatinExtB
//             | Blk::LatinExtC
//             | Blk::LatinExtD
//             | Blk::LatinExtE
//             | Blk::LatinExtF
//             | Blk::LatinExtG
//             | Blk::LatinExtAdditional,
//             _,
//         ) => "Latin Uppercase Letters",
//         (
//             Gc::Ll,
//             Blk::Latin1Sup
//             | Blk::LatinExtA
//             | Blk::LatinExtB
//             | Blk::LatinExtC
//             | Blk::LatinExtD
//             | Blk::LatinExtE
//             | Blk::LatinExtF
//             | Blk::LatinExtG
//             | Blk::LatinExtAdditional,
//             _,
//         ) => "Latin Lowercase Letters",
//         (
//             Gc::Lo | Gc::Lt,
//             Blk::Latin1Sup
//             | Blk::LatinExtA
//             | Blk::LatinExtB
//             | Blk::LatinExtC
//             | Blk::LatinExtD
//             | Blk::LatinExtE
//             | Blk::LatinExtF
//             | Blk::LatinExtG
//             | Blk::LatinExtAdditional,
//             _,
//         ) => "Other Latin Letters",
//         (Gc::Lm, Blk::LatinExtC | Blk::LatinExtD | Blk::LatinExtE, _) => "Other Latin Letters",
//         (_, Blk::IpaExt, _) => "IPA Extensions",
//         (Gc::Sk, Blk::LatinExtD | Blk::LatinExtE, _) => "Latin Modifier Symbols",
//         (
//             Gc::Lm,
//             Blk::LatinExtF,
//             0x10783 | 0x10784 | 0x10792 | 0x10796 | 0x107a5 | 0x107aa | 0x107b2,
//         ) => "Other Latin Letters",
//         (Gc::Sc, ..) => "Currency Symbols",
//         (Gc::Zs, ..) => "Non-ASCII Spaces",
//         (_, Blk::Latin1Sup, _) => "Latin Symbols",
//         (Gc::Lm, Blk::ModifierLetters, _) => "Modifier Letters",
//         (Gc::Sk, Blk::ModifierLetters, _) => "Modifier Symbols",
//         (Gc::Lu, Blk::Greek | Blk::GreekExt, _) => "Greek Uppercase Letters",
//         (Gc::Ll, Blk::Greek | Blk::GreekExt, _) => "Greek Lowercase Letters",
//         (Gc::Lt, Blk::GreekExt, _) => "Other Greek Letters",
//         (_, Blk::Greek | Blk::GreekExt, _) => "Greek Symbols",
//         (Gc::Lu, Blk::Cyrillic | Blk::CyrillicExtB | Blk::CyrillicSup, _) => {
//             "Cyrillic Uppercase Letters"
//         }
//         (Gc::Ll, Blk::Cyrillic | Blk::CyrillicExtB | Blk::CyrillicExtC | Blk::CyrillicSup, _) => {
//             "Cyrillic Lowercase Letters"
//         }
//         (Gc::Lm, Blk::CyrillicExtB | Blk::CyrillicExtD, _) => "Cyrillic Modifier Letters",
//         (
//             _,
//             Blk::Cyrillic
//             | Blk::CyrillicExtB
//             | Blk::CyrillicExtC
//             | Blk::CyrillicExtD
//             | Blk::CyrillicSup,
//             _,
//         ) => "Cyrillic Symbols",
//         (Gc::Lu, Blk::Armenian, _) => "Armenian Uppercase Letters",
//         (Gc::Ll, Blk::Armenian, _) => "Armenian Lowercase Letters",
//         (_, Blk::Armenian, _) => "Armenian Symbols",
//         (Gc::Nd, Blk::Arabic, _) => "Arabic Numbers",
//         (_, Blk::Arabic | Blk::ArabicMath | Blk::ArabicPfA, _) => "Arabic Symbols",
//         (.., 0xfdfd) => "Arabic Symbols",
//         (_, Blk::Nko, _) => "Nko Symbols",
//         (Gc::Lo, Blk::Devanagari | Blk::DevanagariExt, _) => "Devanagari Letters",
//         (Gc::Nd, Blk::Devanagari, _) => "Devanagari Numbers",
//         (_, Blk::Devanagari | Blk::DevanagariExt, _) => "Devanagari Symbols",
//         (Gc::Lo, Blk::Bengali, _) => "Bengali Letters",
//         (Gc::Nd | Gc::No, Blk::Bengali, _) => "Bengali Numbers",
//         (_, Blk::Bengali, _) => "Bengali Symbols",
//         (Gc::Lo, Blk::Gurmukhi, _) => "Gurmukhi Letters",
//         (Gc::Nd, Blk::Gurmukhi, _) => "Gurmukhi Numbers",
//         (_, Blk::Gurmukhi, _) => "Gurmukhi Symbols",
//         (Gc::Lo, Blk::Gujarati, _) => "Gujarati Letters",
//         (Gc::Nd, Blk::Gujarati, _) => "Gujarati Numbers",
//         (_, Blk::Gujarati, _) => "Gujarati Symbols",
//         (Gc::Lo, Blk::Oriya, _) => "Oriya Letters",
//         (Gc::Nd | Gc::No, Blk::Oriya, _) => "Oriya Numbers",
//         (_, Blk::Oriya, _) => "Oriya Symbols",
//         (Gc::Lo, Blk::Tamil, _) => "Tamil Letters",
//         (Gc::Nd | Gc::No, Blk::Tamil | Blk::TamilSup, _) => "Tamil Numbers",
//         (_, Blk::Tamil | Blk::TamilSup, _) => "Tamil Symbols",
//         (Gc::Lo, Blk::Telugu, _) => "Telugu Letters",
//         (Gc::Nd | Gc::No, Blk::Telugu, _) => "Telugu Numbers",
//         (_, Blk::Telugu, _) => "Telugu Symbols",
//         (Gc::Lo, Blk::Kannada, _) => "Kannada Letters",
//         (Gc::Nd | Gc::No, Blk::Kannada, _) => "Kannada Numbers",
//         (_, Blk::Kannada, _) => "Kannada Symbols",
//         (Gc::Lo, Blk::Malayalam, _) => "Malayalam Letters",
//         (Gc::Nd | Gc::No, Blk::Malayalam, _) => "Malayalam Numbers",
//         (_, Blk::Malayalam, _) => "Malayalam Symbols",
//         (Gc::Lo, Blk::Sinhala, _) => "Sinhala Letters",
//         (Gc::Nd | Gc::No, Blk::Sinhala | Blk::SinhalaArchaicNumbers, _) => "Sinhala Numbers",
//         (_, Blk::Sinhala, _) => "Sinhala Symbols",
//         (Gc::Lo, Blk::Thai, _) => "Thai Letters",
//         (Gc::Nd | Gc::No, Blk::Thai, _) => "Thai Numbers",
//         (_, Blk::Thai, _) => "Thai Symbols",
//         (Gc::Lo, Blk::Lao, _) => "Lao Letters",
//         (Gc::Nd | Gc::No, Blk::Lao, _) => "Lao Numbers",
//         (_, Blk::Lao, _) => "Lao Symbols",
//         (Gc::Lo, Blk::Tibetan, _) => "Tibetan Letters",
//         (Gc::Nd | Gc::No, Blk::Tibetan, _) => "Tibetan Numbers",
//         (_, Blk::Tibetan, _) => "Tibetan Symbols",
//         (Gc::Nd, Blk::MyanmarExtC, _) => "Undefined",
//         (Gc::Lo, Blk::Myanmar | Blk::MyanmarExtA | Blk::MyanmarExtB | Blk::MyanmarExtC, _) => {
//             "Myanmar Letters"
//         }
//         (Gc::Nd | Gc::No, Blk::Myanmar | Blk::MyanmarExtB, _) => "Myanmar Numbers",
//         (_, Blk::Myanmar | Blk::MyanmarExtA | Blk::MyanmarExtB | Blk::MyanmarExtC, _) => {
//             "Myanmar Symbols"
//         }
//         (Gc::Lu, Blk::Georgian, _) => "Georgian Uppercase Letters",
//         (Gc::Ll, Blk::Georgian, _) => "Georgian Lowercase Letters",
//         (Gc::Ll, Blk::GeorgianSup, _) => "Georgian Nuskhuri Lowercase Letters",
//         (Gc::Lu, Blk::GeorgianExt, _) => "Georgian Mtavruli Uppercase Letters",
//         (_, Blk::Georgian | Blk::GeorgianExt | Blk::GeorgianSup, _) => "Georgian Symbols",
//         (Gc::Lo, Blk::Jamo | Blk::JamoExtA | Blk::JamoExtB, _) => "Hangul Jamo Letters",
//         (Gc::Lo, Blk::CompatJamo, _) => "Hangul Compatibility Jamo Numbers",
//         (Gc::Lo, Blk::Ethiopic | Blk::EthiopicExt | Blk::EthiopicExtA | Blk::EthiopicSup, _) => {
//             "Ethiopic Letters"
//         }
//         (Gc::Nd | Gc::No, Blk::Ethiopic | Blk::EthiopicExt | Blk::EthiopicSup, _) => {
//             "Ethiopic Numbers"
//         }
//         (_, Blk::Ethiopic | Blk::EthiopicExt | Blk::EthiopicSup, _) => "Ethiopic Symbols",
//         (Gc::Lu, Blk::Cherokee | Blk::CherokeeSup, _) => "Cherokee Uppercase Letters",
//         (Gc::Ll, Blk::Cherokee | Blk::CherokeeSup, _) => "Cherokee Lowercase Letters",
//         (Gc::Lo, Blk::Ucas | Blk::UcasExt, _) => {
//             "Unified Canadian Aboriginal Syllabics (UCAS) Letters"
//         }
//         (_, Blk::Ucas | Blk::UcasExt, _) => "Unified Canadian Aboriginal Syllabics (UCAS) Symbols",
//         (Gc::Lo, Blk::Ogham, _) => "Ogham Letters",
//         (_, Blk::Ogham, _) => "Ogham Symbols",
//         (Gc::Lo, Blk::Runic, _) => "Runic Letters",
//         (Gc::Nl, Blk::Runic, _) => "Runic Letter Numbers",
//         (_, Blk::Runic, _) => "Runic Symbols",
//         (
//             Gc::Lo,
//             Blk::Tagalog,
//             0x1700 | 0x1701 | 0x1702 | 0x1703 | 0x1704 | 0x1705 | 0x1706 | 0x1707 | 0x1708 | 0x1709
//             | 0x170a | 0x170b | 0x170c | 0x170d | 0x170e | 0x170f | 0x1710 | 0x1711 | 0x171f,
//         ) => "Tagalog Letters",
//         (Gc::Lo, Blk::Hanunoo, _) => "Hanunoo Letters",
//         (_, Blk::Hanunoo, _) => "Hanunoo Symbols",
//         (Gc::Lo, Blk::Buhid, _) => "Buhid Letters",
//         (Gc::Lo, Blk::Tagbanwa, _) => "Tagbanwa Letters",
//         (Gc::Lo, Blk::Khmer, _) => "Khmer Letters",
//         (Gc::Nd | Gc::No, Blk::Khmer, _) => "Khmer Numbers",
//         (_, Blk::Khmer | Blk::KhmerSymbols, _) => "Khmer Symbols",
//         (Gc::Lo, Blk::Mongolian, _) => "Mongolian Letters",
//         (Gc::Nd | Gc::No, Blk::Mongolian, _) => "Mongolian Numbers",
//         (_, Blk::Mongolian | Blk::MongolianSup, _) => "Mongolian Symbols",
//         (Gc::Lo, Blk::Limbu, _) => "Limbu Letters",
//         (Gc::Nd | Gc::No, Blk::Limbu, _) => "Limbu Numbers",
//         (_, Blk::Limbu, _) => "Limbu Symbols",
//         (_, Blk::TaiLe, _) => "Tai Le Letters",
//         (Gc::Lo, Blk::NewTaiLue, _) => "New Tai Lue Letters",
//         (Gc::Nd | Gc::No, Blk::NewTaiLue, _) => "New Tai Lue Numbers",
//         (_, Blk::NewTaiLue, _) => "New Tai Lue Symbols",
//         (Gc::Lo, Blk::Buginese, _) => "Buginese Letters",
//         (_, Blk::Buginese, _) => "Buginese Symbols",
//         (Gc::Lo, Blk::TaiTham, _) => "Tai Tham Letters",
//         (Gc::Nd | Gc::No, Blk::TaiTham, _) => "Tai Tham Numbers",
//         (_, Blk::TaiTham, _) => "Tai Tham Symbols",
//         (Gc::Lo, Blk::Balinese, _) => "Balinese Letters",
//         (Gc::Nd | Gc::No, Blk::Balinese, _) => "Balinese Numbers",
//         (_, Blk::Balinese, _) => "Balinese Symbols",
//         (Gc::Lo, Blk::Sundanese, _) => "Sundanese Letters",
//         (Gc::Nd | Gc::No, Blk::Sundanese, _) => "Sundanese Numbers",
//         (_, Blk::Sundanese | Blk::SundaneseSup, _) => "Sundanese Symbols",
//         (Gc::Lo, Blk::Batak, _) => "Batak Letters",
//         (Gc::Nd | Gc::No, Blk::Batak, _) => "Batak Numbers",
//         (_, Blk::Batak, _) => "Batak Symbols",
//         (Gc::Lo, Blk::Lepcha, _) => "Lepcha Letters",
//         (Gc::Nd | Gc::No, Blk::Lepcha, _) => "Lepcha Numbers",
//         (_, Blk::Lepcha, _) => "Lepcha Symbols",
//         (Gc::Lo, Blk::OlChiki, _) => "Ol Chiki Letters",
//         (Gc::Nd | Gc::No, Blk::OlChiki, _) => "Ol Chiki Numbers",
//         (_, Blk::OlChiki, _) => "Ol Chiki Symbols",
//         (_, Blk::VedicExt, _) => "Vedic Letters and Symbols",
//         (Gc::Ll, Blk::PhoneticExt, _) => "Phonetic Lowercase Letters",
//         (Gc::Lm, Blk::PhoneticExt, _) => "Phonetic Modifier Letters",
//         (_, Blk::Punctuation, _) => "Non-ASCII Punctuation and Symbols",
//         (_, Blk::SuperAndSub, _) => "Superscripts and Subscripts",
//         (_, Blk::LetterlikeSymbols, _) => "Letterlike Symbols",
//         (_, Blk::NumberForms, _) => "Number Forms",
//         (_, Blk::Arrows | Blk::SupArrowsA | Blk::SupArrowsB | Blk::MiscArrows, _) => "Arrows",
//         (
//             _,
//             Blk::SupArrowsC,
//             0x1f800 | 0x1f801 | 0x1f802 | 0x1f803 | 0x1f804 | 0x1f805 | 0x1f806 | 0x1f807 | 0x1f808
//             | 0x1f809 | 0x1f80a | 0x1f80b | 0x1f810 | 0x1f811 | 0x1f812 | 0x1f813 | 0x1f814
//             | 0x1f815 | 0x1f816 | 0x1f817 | 0x1f818 | 0x1f819 | 0x1f81a | 0x1f81b | 0x1f81c
//             | 0x1f81d | 0x1f81e | 0x1f81f | 0x1f820 | 0x1f821 | 0x1f822 | 0x1f823 | 0x1f824
//             | 0x1f825 | 0x1f826 | 0x1f827 | 0x1f828 | 0x1f829 | 0x1f82a | 0x1f82b | 0x1f82c
//             | 0x1f82d | 0x1f82e | 0x1f82f | 0x1f830 | 0x1f831 | 0x1f832 | 0x1f833 | 0x1f834
//             | 0x1f835 | 0x1f836 | 0x1f837 | 0x1f838 | 0x1f839 | 0x1f83a | 0x1f83b | 0x1f83c
//             | 0x1f83d | 0x1f83e | 0x1f83f | 0x1f840 | 0x1f841 | 0x1f842 | 0x1f843 | 0x1f844
//             | 0x1f845 | 0x1f846 | 0x1f847 | 0x1f850 | 0x1f851 | 0x1f852 | 0x1f853 | 0x1f854
//             | 0x1f855 | 0x1f856 | 0x1f857 | 0x1f858 | 0x1f859 | 0x1f860 | 0x1f861 | 0x1f862
//             | 0x1f863 | 0x1f864 | 0x1f865 | 0x1f866 | 0x1f867 | 0x1f868 | 0x1f869 | 0x1f86a
//             | 0x1f86b | 0x1f86c | 0x1f86d | 0x1f86e | 0x1f86f | 0x1f870 | 0x1f871 | 0x1f872
//             | 0x1f873 | 0x1f874 | 0x1f875 | 0x1f876 | 0x1f877 | 0x1f878 | 0x1f879 | 0x1f87a
//             | 0x1f87b | 0x1f87c | 0x1f87d | 0x1f87e | 0x1f87f | 0x1f880 | 0x1f881 | 0x1f882
//             | 0x1f883 | 0x1f884 | 0x1f885 | 0x1f886 | 0x1f887 | 0x1f890 | 0x1f891 | 0x1f892
//             | 0x1f893 | 0x1f894 | 0x1f895 | 0x1f896 | 0x1f897 | 0x1f898 | 0x1f899 | 0x1f89a
//             | 0x1f89b | 0x1f89c | 0x1f89d | 0x1f89e | 0x1f89f | 0x1f8a0 | 0x1f8a1 | 0x1f8a2
//             | 0x1f8a3 | 0x1f8a4 | 0x1f8a5 | 0x1f8a6 | 0x1f8a7 | 0x1f8a8 | 0x1f8a9 | 0x1f8aa
//             | 0x1f8ab | 0x1f8ac | 0x1f8ad | 0x1f8b0 | 0x1f8b1,
//         ) => "Arrows",
//         (_, Blk::MathOperators | Blk::SupMathOperators, _) => "Math Operators",
//         (_, Blk::MiscTechnical, _) => "Miscellaneous Technical Symbols",
//         (
//             Gc::So,
//             Blk::ControlPictures,
//             0x2400 | 0x2401 | 0x2402 | 0x2403 | 0x2404 | 0x2405 | 0x2406 | 0x2407 | 0x2408 | 0x2409
//             | 0x240a | 0x240b | 0x240c | 0x240d | 0x240e | 0x240f | 0x2410 | 0x2411 | 0x2412
//             | 0x2413 | 0x2414 | 0x2415 | 0x2416 | 0x2417 | 0x2418 | 0x2419 | 0x241a | 0x241b
//             | 0x241c | 0x241d | 0x241e | 0x241f | 0x2420 | 0x2421 | 0x2422 | 0x2423 | 0x2424
//             | 0x2425 | 0x2426,
//         ) => "Control Pictures",
//         (_, Blk::Ocr, _) => "Optical Character Recognition (OCR)",
//         (Gc::No, Blk::EnclosedAlphanum | Blk::EnclosedAlphanumSup, _) => "Enclosed Numbers",
//         (Gc::So, Blk::EnclosedAlphanum, _) => "Enclosed Letters",
//         (
//             Gc::So,
//             Blk::EnclosedAlphanumSup,
//             0x1f110 | 0x1f111 | 0x1f112 | 0x1f113 | 0x1f114 | 0x1f115 | 0x1f116 | 0x1f117 | 0x1f118
//             | 0x1f119 | 0x1f11a | 0x1f11b | 0x1f11c | 0x1f11d | 0x1f11e | 0x1f11f | 0x1f120
//             | 0x1f121 | 0x1f122 | 0x1f123 | 0x1f124 | 0x1f125 | 0x1f126 | 0x1f127 | 0x1f128
//             | 0x1f129 | 0x1f12a | 0x1f12b | 0x1f12c | 0x1f12d | 0x1f12e | 0x1f12f | 0x1f130
//             | 0x1f131 | 0x1f132 | 0x1f133 | 0x1f134 | 0x1f135 | 0x1f136 | 0x1f137 | 0x1f138
//             | 0x1f139 | 0x1f13a | 0x1f13b | 0x1f13c | 0x1f13d | 0x1f13e | 0x1f13f | 0x1f140
//             | 0x1f141 | 0x1f142 | 0x1f143 | 0x1f144 | 0x1f145 | 0x1f146 | 0x1f147 | 0x1f148
//             | 0x1f149 | 0x1f14a | 0x1f14b | 0x1f14c | 0x1f14d | 0x1f14e | 0x1f14f | 0x1f150
//             | 0x1f151 | 0x1f152 | 0x1f153 | 0x1f154 | 0x1f155 | 0x1f156 | 0x1f157 | 0x1f158
//             | 0x1f159 | 0x1f15a | 0x1f15b | 0x1f15c | 0x1f15d | 0x1f15e | 0x1f15f | 0x1f160
//             | 0x1f161 | 0x1f162 | 0x1f163 | 0x1f164 | 0x1f165 | 0x1f166 | 0x1f167 | 0x1f168
//             | 0x1f169 | 0x1f16a | 0x1f16b | 0x1f16c | 0x1f170 | 0x1f171 | 0x1f172 | 0x1f173
//             | 0x1f174 | 0x1f175 | 0x1f176 | 0x1f177 | 0x1f178 | 0x1f179 | 0x1f17a | 0x1f17b
//             | 0x1f17c | 0x1f17d | 0x1f17e | 0x1f17f | 0x1f180 | 0x1f181 | 0x1f182 | 0x1f183
//             | 0x1f184 | 0x1f185 | 0x1f186 | 0x1f187 | 0x1f188 | 0x1f189 | 0x1f18a | 0x1f18b
//             | 0x1f18c | 0x1f18d | 0x1f18e | 0x1f18f | 0x1f190 | 0x1f191 | 0x1f192 | 0x1f193
//             | 0x1f194 | 0x1f195 | 0x1f196 | 0x1f197 | 0x1f198 | 0x1f199 | 0x1f19a | 0x1f19b
//             | 0x1f19c | 0x1f19d | 0x1f19e | 0x1f19f | 0x1f1a0 | 0x1f1a1 | 0x1f1a2 | 0x1f1a3
//             | 0x1f1a4 | 0x1f1a5 | 0x1f1a6 | 0x1f1a7 | 0x1f1a8 | 0x1f1a9 | 0x1f1aa | 0x1f1ab
//             | 0x1f1ac | 0x1f1e6 | 0x1f1e7 | 0x1f1e8 | 0x1f1e9 | 0x1f1ea | 0x1f1eb | 0x1f1ec
//             | 0x1f1ed | 0x1f1ee | 0x1f1ef | 0x1f1f0 | 0x1f1f1 | 0x1f1f2 | 0x1f1f3 | 0x1f1f4
//             | 0x1f1f5 | 0x1f1f6 | 0x1f1f7 | 0x1f1f8 | 0x1f1f9 | 0x1f1fa | 0x1f1fb | 0x1f1fc
//             | 0x1f1fd | 0x1f1fe | 0x1f1ff,
//         ) => "Enclosed Symbols",
//         (
//             Gc::So,
//             Blk::EnclosedIdeographicSup,
//             0x1f200 | 0x1f201 | 0x1f202 | 0x1f210 | 0x1f211 | 0x1f212 | 0x1f213 | 0x1f214 | 0x1f215
//             | 0x1f216 | 0x1f217 | 0x1f218 | 0x1f219 | 0x1f21a | 0x1f21b | 0x1f21c | 0x1f21d
//             | 0x1f21e | 0x1f21f | 0x1f220 | 0x1f221 | 0x1f222 | 0x1f223 | 0x1f224 | 0x1f225
//             | 0x1f226 | 0x1f227 | 0x1f228 | 0x1f229 | 0x1f22a | 0x1f22b | 0x1f22c | 0x1f22d
//             | 0x1f22e | 0x1f22f | 0x1f230 | 0x1f231 | 0x1f232 | 0x1f233 | 0x1f234 | 0x1f235
//             | 0x1f236 | 0x1f237 | 0x1f238 | 0x1f239 | 0x1f23a | 0x1f23b | 0x1f240 | 0x1f241
//             | 0x1f242 | 0x1f243 | 0x1f244 | 0x1f245 | 0x1f246 | 0x1f247 | 0x1f248 | 0x1f250
//             | 0x1f251,
//         ) => "Enclosed Ideographic Supplement",
//         (_, Blk::BoxDrawing, _) => "Box Drawings",
//         (_, Blk::BlockElements, _) => "Block Elements",
//         (_, Blk::GeometricShapes, _) => "Geometric Shapes",
//         (_, Blk::MiscSymbols, _) => "Miscellaneous Symbols",
//         (_, Blk::Dingbats, _) => "Dingbats",
//         (_, Blk::MiscMathSymbolsA | Blk::MiscMathSymbolsB, _) => "Math Symbols",
//         (_, Blk::AlphabeticPf, 0xfb29) => "Math Symbols",
//         (_, Blk::Braille, _) => "Braille",
//         (Gc::Lu, Blk::Glagolitic, _) => "Glagolitic Uppercase Letters",
//         (Gc::Ll, Blk::Glagolitic, _) => "Glagolitic Lowercase Letters",
//         (Gc::Lu, Blk::Coptic, _) => "Coptic Uppercase Letters",
//         (Gc::Ll, Blk::Coptic, _) => "Coptic Lowercase Letters",
//         (Gc::No, Blk::CopticEpactNumbers, _) => "Coptic Epact Numbers",
//         (_, Blk::Coptic, _) => "Coptic Numbers and Symbols",
//         (Gc::Lo, Blk::Tifinagh, _) => "Tifinagh Symbols",
//         (_, Blk::Tifinagh, _) => "Tifinagh Symbols",
//         (
//             _,
//             Blk::SupPunctuation,
//             0x2e00 | 0x2e01 | 0x2e02 | 0x2e03 | 0x2e04 | 0x2e05 | 0x2e06 | 0x2e07 | 0x2e08 | 0x2e09
//             | 0x2e0a | 0x2e0b | 0x2e0c | 0x2e0d | 0x2e0e | 0x2e0f | 0x2e10 | 0x2e11 | 0x2e12
//             | 0x2e13 | 0x2e14 | 0x2e15 | 0x2e16 | 0x2e17 | 0x2e18 | 0x2e19 | 0x2e1a | 0x2e1b
//             | 0x2e1c | 0x2e1d | 0x2e1e | 0x2e1f | 0x2e20 | 0x2e21 | 0x2e22 | 0x2e23 | 0x2e24
//             | 0x2e25 | 0x2e26 | 0x2e27 | 0x2e28 | 0x2e29 | 0x2e2a | 0x2e2b | 0x2e2c | 0x2e2d
//             | 0x2e2e | 0x2e2f | 0x2e30 | 0x2e31 | 0x2e32 | 0x2e33 | 0x2e34 | 0x2e35 | 0x2e36
//             | 0x2e37 | 0x2e38 | 0x2e39 | 0x2e3a | 0x2e3b | 0x2e3c | 0x2e3d | 0x2e3e | 0x2e3f
//             | 0x2e40 | 0x2e41 | 0x2e42 | 0x2e43 | 0x2e44 | 0x2e45 | 0x2e46 | 0x2e47 | 0x2e48
//             | 0x2e49 | 0x2e4a | 0x2e4b | 0x2e4c | 0x2e4d | 0x2e4e | 0x2e4f | 0x2e50 | 0x2e51
//             | 0x2e52 | 0x2e53 | 0x2e54 | 0x2e55 | 0x2e56 | 0x2e57 | 0x2e58 | 0x2e59 | 0x2e5a
//             | 0x2e5b | 0x2e5c | 0x2e5d,
//         ) => "Non-ASCII Punctuation and Symbols",
//         (_, Blk::CjkRadicalsSup, _) => "CJK Radicals",
//         (_, Blk::Kangxi, _) => "Kangxi Radicals",
//         (
//             _,
//             Blk::Idc,
//             0x2ff0 | 0x2ff1 | 0x2ff2 | 0x2ff3 | 0x2ff4 | 0x2ff5 | 0x2ff6 | 0x2ff7 | 0x2ff8 | 0x2ff9
//             | 0x2ffa | 0x2ffb,
//         ) => "Ideographic Description Characters",
//         (_, Blk::CjkSymbols, _) => "CJK Symbols",
//         (_, Blk::Hiragana, _) => "Hiragana",
//         (_, Blk::Katakana | Blk::KatakanaExt, _) => "Katakana",
//         (_, Blk::Bopomofo, _) => "Bopomofo",
//         (
//             _,
//             Blk::BopomofoExt,
//             0x31a0 | 0x31a1 | 0x31a2 | 0x31a3 | 0x31a4 | 0x31a5 | 0x31a6 | 0x31a7 | 0x31a8 | 0x31a9
//             | 0x31aa | 0x31ab | 0x31ac | 0x31ad | 0x31ae | 0x31af | 0x31b0 | 0x31b1 | 0x31b2
//             | 0x31b3 | 0x31b4 | 0x31b5 | 0x31b6 | 0x31b7 | 0x31b8 | 0x31b9 | 0x31ba | 0x31bb,
//         ) => "Bopomofo",
//         (_, Blk::Kanbun, _) => "Kanbun",
//         (
//             _,
//             Blk::CjkStrokes,
//             0x31c0 | 0x31c1 | 0x31c2 | 0x31c3 | 0x31c4 | 0x31c5 | 0x31c6 | 0x31c7 | 0x31c8 | 0x31c9
//             | 0x31ca | 0x31cb | 0x31cc | 0x31cd | 0x31ce | 0x31cf | 0x31d0 | 0x31d1 | 0x31d2
//             | 0x31d3 | 0x31d4 | 0x31d5 | 0x31d6 | 0x31d7 | 0x31d8 | 0x31d9 | 0x31da | 0x31db
//             | 0x31dc | 0x31dd | 0x31de | 0x31df | 0x31e0 | 0x31e1 | 0x31e2 | 0x31e3,
//         ) => "CJK Strokes",
//         (_, Blk::EnclosedCjk, _) => "Enclosed CJK",
//         (
//             _,
//             Blk::CjkCompat,
//             0x3300 | 0x3301 | 0x3302 | 0x3303 | 0x3304 | 0x3305 | 0x3306 | 0x3307 | 0x3308 | 0x3309
//             | 0x330a | 0x330b | 0x330c | 0x330d | 0x330e | 0x330f | 0x3310 | 0x3311 | 0x3312
//             | 0x3313 | 0x3314 | 0x3315 | 0x3316 | 0x3317 | 0x3318 | 0x3319 | 0x331a | 0x331b
//             | 0x331c | 0x331d | 0x331e | 0x331f | 0x3320 | 0x3321 | 0x3322 | 0x3323 | 0x3324
//             | 0x3325 | 0x3326 | 0x3327 | 0x3328 | 0x3329 | 0x332a | 0x332b | 0x332d | 0x332e
//             | 0x332f | 0x3330 | 0x3331 | 0x3332 | 0x3333 | 0x3334 | 0x3335 | 0x3336 | 0x3337
//             | 0x3338 | 0x3339 | 0x333a | 0x333b | 0x333c | 0x333d | 0x333e | 0x333f | 0x3340
//             | 0x3341 | 0x3342 | 0x3343 | 0x3344 | 0x3345 | 0x3346 | 0x3347 | 0x3348 | 0x3349
//             | 0x334a | 0x334b | 0x334c | 0x334d | 0x334e | 0x334f | 0x3350 | 0x3351 | 0x3352
//             | 0x3353 | 0x3354 | 0x3355 | 0x3356 | 0x3357 | 0x3358 | 0x3359 | 0x335a | 0x335b
//             | 0x335c | 0x335d | 0x335e | 0x335f | 0x3360 | 0x3361 | 0x3362 | 0x3363 | 0x3364
//             | 0x3365 | 0x3366 | 0x3367 | 0x3368 | 0x3369 | 0x336a | 0x336b | 0x336c | 0x336d
//             | 0x336e | 0x336f | 0x3370 | 0x3371 | 0x3372 | 0x3373 | 0x3374 | 0x3375 | 0x3376
//             | 0x3377 | 0x3378 | 0x3379 | 0x337a | 0x337b | 0x337c | 0x337d | 0x337e | 0x337f
//             | 0x3380 | 0x3381 | 0x3382 | 0x3383 | 0x3384 | 0x3385 | 0x3386 | 0x3387 | 0x3388
//             | 0x3389 | 0x338a | 0x338b | 0x338c | 0x338d | 0x338e | 0x338f | 0x3390 | 0x3391
//             | 0x3392 | 0x3393 | 0x3394 | 0x3395 | 0x3396 | 0x3397 | 0x3398 | 0x3399 | 0x339a
//             | 0x339b | 0x339c | 0x339d | 0x339e | 0x339f | 0x33a0 | 0x33a1 | 0x33a2 | 0x33a3
//             | 0x33a4 | 0x33a5 | 0x33a6 | 0x33a7 | 0x33a8 | 0x33a9 | 0x33aa | 0x33ab | 0x33ac
//             | 0x33ad | 0x33ae | 0x33af | 0x33b0 | 0x33b1 | 0x33b2 | 0x33b3 | 0x33b4 | 0x33b5
//             | 0x33b6 | 0x33b7 | 0x33b8 | 0x33b9 | 0x33ba | 0x33bb | 0x33bc | 0x33bd | 0x33be
//             | 0x33bf | 0x33c0 | 0x33c1 | 0x33c2 | 0x33c3 | 0x33c4 | 0x33c5 | 0x33c6 | 0x33c7
//             | 0x33c8 | 0x33c9 | 0x33ca | 0x33cb | 0x33cc | 0x33cd | 0x33ce | 0x33cf | 0x33d0
//             | 0x33d1 | 0x33d2 | 0x33d3 | 0x33d4 | 0x33d5 | 0x33d6 | 0x33d7 | 0x33d8 | 0x33d9
//             | 0x33da | 0x33db | 0x33dc | 0x33dd | 0x33de | 0x33df | 0x33e0 | 0x33e1 | 0x33e2
//             | 0x33e3 | 0x33e4 | 0x33e5 | 0x33e6 | 0x33e7 | 0x33e8 | 0x33e9 | 0x33ea | 0x33eb
//             | 0x33ec | 0x33ed | 0x33ee | 0x33ef | 0x33f0 | 0x33f1 | 0x33f2 | 0x33f3 | 0x33f4
//             | 0x33f5 | 0x33f6 | 0x33f7 | 0x33f8 | 0x33f9 | 0x33fa | 0x33fb | 0x33fc | 0x33fd
//             | 0x33fe | 0x33ff,
//         ) => "CJK Compatibility Characters",
//         (_, Blk::CjkExtA, _) => "Rare Han Ideographs",
//         (_, Blk::Yijing, _) => "Yijing Symbols",
//         (_, Blk::Cjk, _) => "CJK Unified Ideograph",
//         (_, Blk::YiSyllables, _) => "Yi Syllables",
//         (_, Blk::YiRadicals, _) => "Yi Radicals",
//         (Gc::Lo, Blk::Lisu, _) => "Lisu Letters",
//         (_, Blk::Lisu, _) => "Lisu Symbols",
//         (Gc::Lo, Blk::Vai, _) => "Vai Letters",
//         (Gc::Nd | Gc::No, Blk::Vai, _) => "Vai Numbers",
//         (_, Blk::Vai, _) => "Vai Symbols",
//         (Gc::Lo, Blk::Bamum | Blk::BamumSup, _) => "Bamum Letters",
//         (Gc::Nd | Gc::Nl | Gc::No, Blk::Bamum, _) => "Bamum Numbers",
//         (_, Blk::Bamum, _) => "Bamum Symbols",
//         (_, Blk::ModifierToneLetters, _) => "Chinese Modifier Tone Letters",
//         (Gc::Lo, Blk::SylotiNagri, _) => "Syloti Nagri Letters",
//         (_, Blk::SylotiNagri, _) => "Syloti Nagri Symbols",
//         (_, Blk::IndicNumberForms, _) => "Indic Number Forms and Symbols",
//         (Gc::Lo, Blk::PhagsPa, _) => "Phags Pa Letters",
//         (_, Blk::PhagsPa, _) => "Phags Pa Symbols",
//         (Gc::Lo, Blk::Saurashtra, _) => "Saurashtra Letters",
//         (Gc::Nd | Gc::No, Blk::Saurashtra, _) => "Saurashtra Numbers",
//         (_, Blk::Saurashtra, _) => "Saurashtra Symbols",
//         (Gc::Lo, Blk::KayahLi, _) => "Kayah Li Letters",
//         (Gc::Nd | Gc::No, Blk::KayahLi, _) => "Kayah Li Numbers",
//         (_, Blk::KayahLi, _) => "Kayah Li Symbols",
//         (Gc::Lo, Blk::Rejang, _) => "Rejang Letters",
//         (_, Blk::Rejang, _) => "Rejang Symbols",
//         (Gc::Lo, Blk::Javanese, _) => "Javanese Letters",
//         (Gc::Nd | Gc::No, Blk::Javanese, _) => "Javanese Numbers",
//         (_, Blk::Javanese, _) => "Javanese Symbols",
//         (Gc::Lo, Blk::Cham, _) => "Cham Letters",
//         (Gc::Nd | Gc::No, Blk::Cham, _) => "Cham Numbers",
//         (_, Blk::Cham, _) => "Cham Symbols",
//         (Gc::Lo, Blk::TaiViet, _) => "Tai Viet Letters",
//         (Gc::Nd | Gc::No, Blk::TaiViet, _) => "Tai Viet Numbers",
//         (_, Blk::TaiViet, _) => "Tai Viet Symbols",
//         (Gc::Lo, Blk::MeeteiMayek | Blk::MeeteiMayekExt, _) => "Manipuri Letters",
//         (Gc::Nd | Gc::No, Blk::MeeteiMayek | Blk::MeeteiMayekExt, _) => "Manipuri Numbers",
//         (_, Blk::MeeteiMayek | Blk::MeeteiMayekExt, _) => "Manipuri Symbols",
//         (_, Blk::Hangul, _) => "Hangul (Korean) Letters",
//         (_, Blk::CjkCompatIdeographs, _) => "CJK Compatibility Ideographs",
//         (_, Blk::AlphabeticPf, _) => "Alphabetic Presentation Forms",
//         (_, Blk::VerticalForms, _) => "Chinese Vertical Punctuation Forms",
//         (_, Blk::CjkCompatForms, _) => "CJK Compatibility Vertical Punctuation Forms",
//         (_, Blk::SmallForms, _) => "CJK Compatibility Small Punctuation Forms",
//         (Gc::Lu, Blk::HalfAndFullForms, _) => "Half-Width/Full-Width Latin Uppercase Letters",
//         (Gc::Ll, Blk::HalfAndFullForms, _) => "Half-Width/Full-Width Latin Lowercase Letters",
//         (Gc::Lo, Blk::HalfAndFullForms, _) => "Other Half-Width/Full-Width Letters",
//         (Gc::Nd | Gc::No, Blk::HalfAndFullForms, _) => "Half-Width/Full-Width Numbers",
//         (_, Blk::HalfAndFullForms, _) => "Half-Width/Full-Width Symbols",
//         (_, Blk::LinearBSyllabary, _) => "Linear B Syllabary",
//         (_, Blk::LinearBIdeograms, _) => "Linear B Ideograms",
//         (Gc::No, Blk::AegeanNumbers, _) => "Aegean Numbers",
//         (_, Blk::AegeanNumbers, _) => "Aegean Punctuation and Measures",
//         (Gc::Nl, Blk::AncientGreekNumbers, _) => "Ancient Greek Acrophonic Numerals",
//         (_, Blk::AncientGreekNumbers, _) => "Ancient Greek Symbols and Papyrological Numbers",
//         (_, Blk::AncientSymbols, _) => "Ancient Symbols",
//         (_, Blk::Phaistos, _) => "Phaistos Symbols",
//         (_, Blk::Lycian, _) => "Lycian Letters",
//         (_, Blk::Carian, _) => "Carian Letters",
//         (Gc::Lo, Blk::OldItalic, _) => "Old Italic Letters",
//         (_, Blk::OldItalic, _) => "Old Italic Letters and Numbers",
//         (_, Blk::Gothic, _) => "Gothic Letters and Numbers",
//         (_, Blk::OldPermic, _) => "Old Permic Letters",
//         (_, Blk::Ugaritic, _) => "Ugaritic Letters and Symbols",
//         (_, Blk::OldPersian, _) => "Old Persian Letters, Numbers, and Symbols",
//         (Gc::Lu, Blk::Deseret, _) => "Deseret Uppercase Letters",
//         (Gc::Ll, Blk::Deseret, _) => "Deseret Lowercase Letters",
//         (_, Blk::Shavian, _) => "Shavian Letters",
//         (Gc::Lo, Blk::Osmanya, _) => "Osmanya Letters",
//         (Gc::Nd, Blk::Osmanya, _) => "Osmanya Numbers",
//         (Gc::Lu, Blk::Osage, _) => "Osage Uppercase Letters",
//         (Gc::Ll, Blk::Osage, _) => "Osage Lowercase Letters",
//         (Gc::Lo, Blk::Elbasan, _) => "Elbasan Letters",
//         (Gc::Lo, Blk::CaucasianAlbanian, _) => "Caucasian Albanian Letters",
//         (_, Blk::LinearA, _) => "Linear A",
//         (_, Blk::Garay, _) => "Undefined",
//         (Gc::Lo, Blk::Brahmi, 0x11071 | 0x11072 | 0x11075) => "Undefined",
//         (Gc::Lo, Blk::Brahmi, _) => "Brahmi Letters",
//         (Gc::Nd | Gc::No, Blk::Brahmi, _) => "Brahmi Numbers",
//         (_, Blk::Brahmi, _) => "Brahmi Letters",
//         (_, Blk::Kaithi, _) => "Kaithi Letters and Punctuation",
//         (Gc::Lo, Blk::SoraSompeng, _) => "Sora Sompeng Letters",
//         (Gc::Nd, Blk::SoraSompeng, _) => "Sora Sompeng Letters",
//         (Gc::Lo, Blk::Chakma, 0x11147) => "Undefined",
//         (Gc::Lo, Blk::Chakma, _) => "Chakma Letters",
//         (Gc::Nd, Blk::Chakma, _) => "Chakma Numbers",
//         (_, Blk::Chakma, _) => "Chakma Symbols",
//         (_, Blk::Mahajani, _) => "Mahajani Letters and Symbols",
//         (Gc::Lo, Blk::Sharada, _) => "Sharada Letters",
//         (Gc::Nd | Gc::No, Blk::Sharada, _) => "Sharada Numbers",
//         (_, Blk::Sharada, _) => "Sharada Symbols",
//         (_, Blk::Khojki, _) => "Khojki Letters and Symbols",
//         (_, Blk::Multani, _) => "Multani Letters and Symbols",
//         (Gc::Lo, Blk::Khudawadi, _) => "Khudawadi Letters",
//         (Gc::Nd, Blk::Khudawadi, _) => "Khudawadi Numbers",
//         (_, Blk::Grantha, _) => "Grantha Letters",
//         (Gc::Lo, Blk::Newa, _) => "Newa Letters",
//         (Gc::Nd | Gc::No, Blk::Newa, _) => "Newa Numbers",
//         (_, Blk::Newa, _) => "Newa Symbols",
//         (Gc::Lo, Blk::Tirhuta, _) => "Tirhuta Letters",
//         (Gc::Nd | Gc::No, Blk::Tirhuta, _) => "Tirhuta Numbers",
//         (_, Blk::Tirhuta, _) => "Non-ASCII Punctuation and Symbols",
//         (Gc::Lo, Blk::Siddham, _) => "Siddham Letters",
//         (_, Blk::Siddham, _) => "Siddham Symbols",
//         (Gc::Lo, Blk::Modi, _) => "Modi Letters",
//         (Gc::Nd | Gc::No, Blk::Modi, _) => "Modi Numbers",
//         (_, Blk::Modi, _) => "Modi Symbols",
//         (Gc::Po, Blk::Takri, _) => "Undefined",
//         (Gc::Lo, Blk::Takri, _) => "Takri Letters",
//         (Gc::Nd, Blk::Takri, _) => "Takri Numbers",
//         (
//             Gc::Lo,
//             Blk::Ahom,
//             0x11740 | 0x11741 | 0x11742 | 0x11743 | 0x11744 | 0x11745 | 0x11746,
//         ) => "Undefined",
//         (Gc::Lo, Blk::Ahom, _) => "Ahom Letters",
//         (Gc::Nd | Gc::No, Blk::Ahom, _) => "Ahom Numbers",
//         (_, Blk::Ahom, _) => "Ahom Symbols",
//         (_, Blk::Dogra, _) => "Dogra",
//         (Gc::Lu, Blk::WarangCiti, _) => "Varang Kshiti Uppercase Letters",
//         (Gc::Ll, Blk::WarangCiti, _) => "Varang Kshiti Lowercase Letters",
//         (Gc::Lo, Blk::WarangCiti, _) => "Other Varang Kshiti Letters",
//         (Gc::Nd | Gc::No, Blk::WarangCiti, _) => "Varang Kshiti Numbers",
//         (Gc::Lo, Blk::ZanabazarSquare, _) => "Zanabazar Square Letters",
//         (_, Blk::ZanabazarSquare, _) => "Zanabazar Square Symbols",
//         (Gc::Lo, Blk::Soyombo, _) => "Soyombo Letters",
//         (_, Blk::Soyombo, _) => "Soyombo Symbols",
//         (_, Blk::PauCinHau, _) => "Pau Cin Hau Letters",
//         (Gc::Lo, Blk::Bhaiksuki, _) => "Bhaiksuki Letters",
//         (Gc::Nd | Gc::No, Blk::Bhaiksuki, _) => "Bhaiksuki Numbers",
//         (_, Blk::Bhaiksuki, _) => "Bhaiksuki Symbols",
//         (_, Blk::Marchen, _) => "Marchen Letters and Symbols",
//         (Gc::Lo, Blk::MasaramGondi, _) => "Masaram Gondi Letters",
//         (Gc::Nd, Blk::MasaramGondi, _) => "Masaram Gondi Numbers",
//         (Gc::Lo, Blk::GunjalaGondi, _) => "Gunjala Gondi Letters",
//         (Gc::Nd, Blk::GunjalaGondi, _) => "Gunjala Gondi Numbers",
//         (_, Blk::Cuneiform, _) => "Cuneiform Letters",
//         (_, Blk::CuneiformNumbers, _) => "Cuneiform Numbers",
//         (_, Blk::EarlyDynasticCuneiform, _) => "Early Dynastic Cuneiform Letters",
//         (_, Blk::EgyptianHieroglyphs, 0x1342f) => "Undefined",
//         (_, Blk::EgyptianHieroglyphs, _) => "Egyptian Hieroglyphs",
//         (_, Blk::AnatolianHieroglyphs, _) => "Anatolian Hieroglyphs",
//         (Gc::Lo, Blk::Mro, _) => "Mro Letters",
//         (Gc::Nd, Blk::Mro, _) => "Mro Numbers",
//         (_, Blk::Mro, _) => "Mro Symbols",
//         (_, Blk::BassaVah, _) => "Bassa Vah",
//         (Gc::Lm | Gc::Lo, Blk::PahawhHmong, _) => "Pahawh Hmong Letters",
//         (Gc::Nd | Gc::No, Blk::PahawhHmong, _) => "Pahawh Hmong Numbers",
//         (_, Blk::PahawhHmong, _) => "Pahawh Hmong Symbols",
//         (Gc::Lu, Blk::Medefaidrin, _) => "Medefaidrin Uppercase Letters",
//         (Gc::Ll, Blk::Medefaidrin, _) => "Medefaidrin Lowercase Letters",
//         (Gc::No | Gc::Po, Blk::Medefaidrin, _) => "Medefaidrin Numbers and Symbols",
//         (_, Blk::Miao, _) => "Miao Letters",
//         (_, Blk::IdeographicSymbols, 0x16fe2 | 0x16fe3) => "Undefined",
//         (_, Blk::IdeographicSymbols, _) => "Non-ASCII Punctuation and Symbols",
//         (_, Blk::Tangut, _) => "Tangut Hieroglyphs",
//         (
//             _,
//             Blk::TangutComponents,
//             0x18af3 | 0x18af4 | 0x18af5 | 0x18af6 | 0x18af7 | 0x18af8 | 0x18af9 | 0x18afa | 0x18afb
//             | 0x18afc | 0x18afd | 0x18afe | 0x18aff,
//         ) => "Undefined",
//         (_, Blk::TangutComponents, _) => "Tangut Hieroglyph Components",
//         (_, Blk::Nushu, _) => "Nushu",
//         (_, Blk::Duployan, _) => "Duployan",
//         (_, Blk::ByzantineMusic, _) => "Byzantine Music Symbols",
//         (_, Blk::Music, 0x1d1e9 | 0x1d1ea) => "Undefined",
//         (_, Blk::Music, _) => "Music Symbols",
//         (_, Blk::AncientGreekMusic, _) => "Ancient Greek Music Symbols",
//         (_, Blk::MayanNumerals, _) => "Mayan Numerals",
//         (_, Blk::TaiXuanJing, _) => "Tai Xuan Jing",
//         (_, Blk::CountingRod, _) => "Counting Rod Numerals",
//         (Gc::Lu, Blk::MathAlphanum, _) => "Mathematical Uppercase Letters",
//         (Gc::Ll, Blk::MathAlphanum, _) => "Mathematical Lowercase Letters",
//         (Gc::Nd, Blk::MathAlphanum, _) => "Mathematical Numbers",
//         (_, Blk::MathAlphanum, _) => "Mathematical Symbols",
//         (_, Blk::SuttonSignwriting, _) => "Sutton Signwriting Symbols",
//         (Gc::Lo | Gc::Lm, Blk::NyiakengPuachueHmong, _) => "Nyiakeng Puachue Hmong Letters",
//         (Gc::Nd, Blk::NyiakengPuachueHmong, _) => "Nyiakeng Puachue Hmong Numbers",
//         (_, Blk::NyiakengPuachueHmong, _) => "Nyiakeng Puachue Hmong Symbols",
//         (Gc::Lo, Blk::Wancho, _) => "Wancho Letters",
//         (Gc::Nd, Blk::Wancho, _) => "Wancho Numbers",
//         (_, Blk::Mahjong, _) => "Mahjong Tiles",
//         (_, Blk::Domino, _) => "Domino Tiles",
//         (_, Blk::PlayingCards, _) => "Playing Cards",
//         (_, Blk::MiscPictographs, _) => "Miscellaneous Symbols and Pictographs",
//         (_, Blk::Emoticons, _) => "Emoticons",
//         (_, Blk::OrnamentalDingbats, _) => "Ornamental Dingbats",
//         (_, Blk::TransportAndMap, _) => "Transportation and Mapping Emojis",
//         (_, Blk::Alchemical, _) => "Alchemical Symbols",
//         (_, Blk::GeometricShapesExt, _) => "Geometric Shapes",
//         (_, Blk::SupSymbolsAndPictographs, 0x1f908 | 0x1f909 | 0x1f90a | 0x1f90b) => "Undefined",
//         (_, Blk::SupSymbolsAndPictographs, _) => "Supplemental Symbols and Pictographs",
//         (_, Blk::ChessSymbols, _) => "Chess Symbols",
//         (_, Blk::SymbolsAndPictographsExtA, _) => "Miscellaneous Symbols and Pictographs",
//         (
//             _,
//             Blk::SymbolsForLegacyComputing,
//             0x1fbcb | 0x1fbcc | 0x1fbcd | 0x1fbce | 0x1fbcf | 0x1fbd0 | 0x1fbd1 | 0x1fbd2 | 0x1fbd3
//             | 0x1fbd4 | 0x1fbd5 | 0x1fbd6 | 0x1fbd7 | 0x1fbd8 | 0x1fbd9 | 0x1fbda | 0x1fbdb
//             | 0x1fbdc | 0x1fbdd | 0x1fbde | 0x1fbdf | 0x1fbe0 | 0x1fbe1 | 0x1fbe2 | 0x1fbe3
//             | 0x1fbe4 | 0x1fbe5 | 0x1fbe6 | 0x1fbe7 | 0x1fbe8 | 0x1fbe9 | 0x1fbea | 0x1fbeb
//             | 0x1fbec | 0x1fbed | 0x1fbee | 0x1fbef,
//         ) => "Undefined",
//         (_, Blk::SymbolsForLegacyComputing, _) => "Symbols for Legacy Computing",
//         (_, Blk::PhoneticExtSup, _) => "Phonetic Extensions Supplement",
//         // catch all for other punctuation
//         (Gc::Po, ..) => "Non-ASCII Punctuation and Symbols",
//         // everything else is undefined
//         _ => "Undefined",
//     })
// }
