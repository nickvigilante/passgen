use seshat::unicode::props::{Blk, Gc, Sc};

use crate::app::config::{
    category::CodePointCategory,
    code_point::{
        FilterCondition, FilterConditionOp, FilterValue, Filterable, generate_all_code_points,
    },
};

fn african_characters() -> FilterCondition {
    FilterCondition::new(
        "blk",
        FilterValue::PropBlk(&[
            Blk::Ethiopic,
            Blk::EthiopicSup,
            Blk::EthiopicExt,
            Blk::Tifinagh,
        ]),
        true,
    )
}

fn ascii_basic_symbols() -> FilterConditionOp {
    let filters: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition::new(
            "blk",
            FilterValue::PropBlk(&[Blk::Ascii]),
            true,
        )),
        Box::new(FilterCondition::new(
            "cp_id_hex",
            FilterValue::PropU32(&[0x21, 0x23, 0x24, 0x25, 0x26, 0x2a, 0x40, 0x5e]),
            true,
        )),
    ];
    FilterConditionOp::And(Box::leak(filters.into_boxed_slice()))
}

fn ascii_digits() -> FilterConditionOp {
    let filters: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition::new(
            "blk",
            FilterValue::PropBlk(&[Blk::Ascii]),
            true,
        )),
        Box::new(FilterCondition::new(
            "gc",
            FilterValue::PropGc(&[Gc::Nd]),
            true,
        )),
    ];
    FilterConditionOp::And(Box::leak(filters.into_boxed_slice()))
}

fn ascii_extended_symbols() -> FilterConditionOp {
    let filters: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition::new(
            "blk",
            FilterValue::PropBlk(&[Blk::Ascii]),
            true,
        )),
        Box::new(FilterCondition::new(
            "gc",
            FilterValue::PropGc(&[Gc::Pc, Gc::Pd, Gc::Pe, Gc::Po, Gc::Ps, Gc::Sk, Gc::Sm]),
            true,
        )),
        Box::new(FilterCondition::new(
            "cp_id_hex",
            FilterValue::PropU32(&[0x21, 0x23, 0x24, 0x25, 0x26, 0x2a, 0x40, 0x5e]),
            false,
        )),
    ];
    FilterConditionOp::And(Box::leak(filters.into_boxed_slice()))
}

fn ascii_lowercase_letters() -> FilterConditionOp {
    let filters: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition::new(
            "blk",
            FilterValue::PropBlk(&[Blk::Ascii]),
            true,
        )),
        Box::new(FilterCondition::new(
            "gc",
            FilterValue::PropGc(&[Gc::Ll]),
            true,
        )),
    ];
    FilterConditionOp::And(Box::leak(filters.into_boxed_slice()))
}

fn ascii_space() -> FilterConditionOp {
    let filters: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition::new(
            "blk",
            FilterValue::PropBlk(&[Blk::Ascii]),
            true,
        )),
        Box::new(FilterCondition::new(
            "gc",
            FilterValue::PropGc(&[Gc::Zs]),
            true,
        )),
    ];
    FilterConditionOp::And(Box::leak(filters.into_boxed_slice()))
}

fn ascii_uppercase_letters() -> FilterConditionOp {
    let filters: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition::new(
            "blk",
            FilterValue::PropBlk(&[Blk::Ascii]),
            true,
        )),
        Box::new(FilterCondition::new(
            "gc",
            FilterValue::PropGc(&[Gc::Lu]),
            true,
        )),
    ];
    FilterConditionOp::And(Box::leak(filters.into_boxed_slice()))
}

fn cjk_characters() -> FilterConditionOp {
    let filters: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition::new(
            "blk",
            FilterValue::PropBlk(&[
                Blk::Nko,
                Blk::CjkRadicalsSup,
                Blk::CjkSymbols,
                Blk::CjkStrokes,
                Blk::EnclosedCjk,
                Blk::CjkCompat,
                Blk::CjkExtA,
                Blk::Cjk,
                Blk::CjkCompatIdeographs,
                Blk::CjkCompatForms,
            ]),
            true,
        )),
        Box::new(FilterCondition::new(
            "gc",
            FilterValue::PropGc(&[Gc::Zs]),
            false,
        )),
    ];
    FilterConditionOp::And(Box::leak(filters.into_boxed_slice()))
}

fn european_characters() -> FilterConditionOp {
    let filters_1: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition::new(
            "blk",
            FilterValue::PropBlk(&[Blk::AlphabeticPf]),
            true,
        )),
        Box::new(FilterCondition::new(
            "sc",
            FilterValue::PropSc(&[Sc::Armn]),
            true,
        )),
    ];
    let filters_2: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition::new(
            "blk",
            FilterValue::PropBlk(&[
                Blk::Greek,
                Blk::GreekExt,
                Blk::Cyrillic,
                Blk::CyrillicExtA,
                Blk::CyrillicExtB,
                Blk::CyrillicExtC,
                Blk::CyrillicSup,
                Blk::Glagolitic,
                Blk::Coptic,
                Blk::CopticEpactNumbers,
                Blk::Armenian,
                Blk::Georgian,
                Blk::GeorgianExt,
                Blk::GeorgianSup,
                Blk::OldItalic,
                Blk::Gothic,
                Blk::OldPermic,
                Blk::Shavian,
                Blk::Elbasan,
                Blk::CaucasianAlbanian,
                Blk::Ogham,
                Blk::Runic,
                Blk::Duployan,
            ]),
            true,
        )),
        Box::new(FilterConditionOp::And(Box::leak(
            filters_1.into_boxed_slice(),
        ))),
    ];

    let filters: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition::new(
            "gc",
            FilterValue::PropGc(&[Gc::Zs]),
            false,
        )),
        Box::new(FilterConditionOp::Or(Box::leak(
            filters_2.into_boxed_slice(),
        ))),
    ];

    FilterConditionOp::And(Box::leak(filters.into_boxed_slice()))
}

fn extended_numbers() -> FilterCondition {
    FilterCondition::new(
        "blk",
        FilterValue::PropBlk(&[
            Blk::AegeanNumbers,
            Blk::AncientGreekNumbers,
            Blk::MayanNumerals,
            Blk::CountingRod,
        ]),
        true,
    )
}

fn extended_symbols_and_emojis() -> FilterConditionOp {
    let filters_1: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition::new(
            "blk",
            FilterValue::PropBlk(&[Blk::AlphabeticPf]),
            true,
        )),
        Box::new(FilterCondition::new(
            "sc",
            FilterValue::PropSc(&[Sc::Hebr]),
            true,
        )),
    ];
    let filters_2: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition::new(
            "blk",
            FilterValue::PropBlk(&[
                Blk::Punctuation,
                Blk::SuperAndSub,
                Blk::LetterlikeSymbols,
                Blk::NumberForms,
                Blk::Arrows,
                Blk::MathOperators,
                Blk::MiscTechnical,
                Blk::Ocr,
                Blk::EnclosedAlphanum,
                Blk::ControlPictures,
                Blk::BoxDrawing,
                Blk::BlockElements,
                Blk::GeometricShapes,
                Blk::GeometricShapesExt,
                Blk::MiscSymbols,
                Blk::Dingbats,
                Blk::MiscMathSymbolsA,
                Blk::SupArrowsA,
                Blk::Braille,
                Blk::SupArrowsB,
                Blk::SupArrowsC,
                Blk::MiscMathSymbolsB,
                Blk::SupMathOperators,
                Blk::MiscArrows,
                Blk::SupPunctuation,
                Blk::Idc,
                Blk::VerticalForms,
                Blk::SmallForms,
                Blk::HalfAndFullForms,
                Blk::AncientSymbols,
                Blk::Phaistos,
                Blk::Deseret,
                Blk::Miao,
                Blk::IdeographicSymbols,
                Blk::Phoenician,
                Blk::ByzantineMusic,
                Blk::Music,
                Blk::AncientGreekMusic,
                Blk::MathAlphanum,
                Blk::SuttonSignwriting,
                Blk::NyiakengPuachueHmong,
                Blk::ArabicMath,
                Blk::Mahjong,
                Blk::Domino,
                Blk::PlayingCards,
                Blk::EnclosedAlphanumSup,
                Blk::EnclosedIdeographicSup,
                Blk::MiscPictographs,
                Blk::Emoticons,
                Blk::TransportAndMap,
                Blk::Alchemical,
                Blk::SupSymbolsAndPictographs,
                Blk::ChessSymbols,
                Blk::SymbolsAndPictographsExtA,
                Blk::SymbolsForLegacyComputing,
                Blk::OrnamentalDingbats,
            ]),
            true,
        )),
        Box::new(FilterConditionOp::And(Box::leak(
            filters_1.into_boxed_slice(),
        ))),
    ];

    let filters: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition::new(
            "gc",
            FilterValue::PropGc(&[Gc::Zs]),
            false,
        )),
        Box::new(FilterConditionOp::Or(Box::leak(
            filters_2.into_boxed_slice(),
        ))),
    ];

    FilterConditionOp::And(Box::leak(filters.into_boxed_slice()))
}

fn first_nations() -> FilterCondition {
    FilterCondition::new(
        "blk",
        FilterValue::PropBlk(&[
            Blk::Cherokee,
            Blk::CherokeeSup,
            Blk::Ucas,
            Blk::UcasExt,
            Blk::Osage,
        ]),
        true,
    )
}

fn indian_characters() -> FilterCondition {
    FilterCondition::new(
        "blk",
        FilterValue::PropBlk(&[
            Blk::Devanagari,
            Blk::DevanagariExt,
            Blk::DevanagariExtA,
            Blk::Bengali,
            Blk::Gurmukhi,
            Blk::Gujarati,
            Blk::Oriya,
            Blk::Tamil,
            Blk::TamilSup,
            Blk::Telugu,
            Blk::Kannada,
            Blk::Malayalam,
            Blk::Lepcha,
            Blk::OlChiki,
            Blk::VedicExt,
            Blk::IndicNumberForms,
            Blk::Saurashtra,
            Blk::MeeteiMayek,
            Blk::MeeteiMayekExt,
            Blk::Brahmi,
            Blk::Kaithi,
            Blk::SoraSompeng,
            Blk::Mahajani,
            Blk::Sharada,
            Blk::Grantha,
            Blk::Modi,
            Blk::Ahom,
            Blk::Dogra,
            Blk::WarangCiti,
            Blk::Bhaiksuki,
            Blk::MasaramGondi,
            Blk::GunjalaGondi,
            Blk::Wancho,
        ]),
        true,
    )
}

fn ipa_extended_chars_and_modifier_letters() -> FilterCondition {
    FilterCondition::new(
        "blk",
        FilterValue::PropBlk(&[
            Blk::IpaExt,
            Blk::PhoneticExt,
            Blk::PhoneticExtSup,
            Blk::ModifierLetters,
        ]),
        true,
    )
}

fn latin_lowercase_characters() -> FilterConditionOp {
    let filters_1: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition::new(
            "blk",
            FilterValue::PropBlk(&[Blk::AlphabeticPf]),
            true,
        )),
        Box::new(FilterCondition::new(
            "sc",
            FilterValue::PropSc(&[Sc::Latn]),
            true,
        )),
    ];
    let filters_2: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition::new(
            "blk",
            FilterValue::PropBlk(&[
                Blk::Latin1Sup,
                Blk::LatinExtA,
                Blk::LatinExtAdditional,
                Blk::LatinExtB,
                Blk::LatinExtC,
                Blk::LatinExtD,
                Blk::LatinExtE,
            ]),
            true,
        )),
        Box::new(FilterCondition::new(
            "gc",
            FilterValue::PropGc(&[Gc::Ll]),
            true,
        )),
    ];

    let filters: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterConditionOp::And(Box::leak(
            filters_1.into_boxed_slice(),
        ))),
        Box::new(FilterConditionOp::And(Box::leak(
            filters_2.into_boxed_slice(),
        ))),
    ];

    FilterConditionOp::Or(Box::leak(filters.into_boxed_slice()))
}

fn latin_symbols() -> FilterConditionOp {
    let filters: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition::new(
            "blk",
            FilterValue::PropBlk(&[Blk::Latin1Sup, Blk::LatinExtD, Blk::LatinExtE]),
            true,
        )),
        Box::new(FilterCondition::new(
            "gc",
            FilterValue::PropGc(&[
                Gc::No,
                Gc::Pf,
                Gc::Pi,
                Gc::Po,
                Gc::Sc,
                Gc::Sk,
                Gc::Sm,
                Gc::So,
            ]),
            true,
        )),
    ];
    FilterConditionOp::And(Box::leak(filters.into_boxed_slice()))
}

fn latin_uppercase_letters() -> FilterConditionOp {
    let filters: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition::new(
            "blk",
            FilterValue::PropBlk(&[
                Blk::Latin1Sup,
                Blk::LatinExtA,
                Blk::LatinExtAdditional,
                Blk::LatinExtB,
                Blk::LatinExtC,
                Blk::LatinExtD,
            ]),
            true,
        )),
        Box::new(FilterCondition::new(
            "gc",
            FilterValue::PropGc(&[Gc::Lu]),
            true,
        )),
    ];
    FilterConditionOp::And(Box::leak(filters.into_boxed_slice()))
}

fn linear_a_linear_b_characters() -> FilterCondition {
    FilterCondition::new(
        "blk",
        FilterValue::PropBlk(&[Blk::LinearA, Blk::LinearBIdeograms, Blk::LinearBSyllabary]),
        true,
    )
}

fn non_ascii_space() -> FilterConditionOp {
    let filters: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition::new(
            "blk",
            FilterValue::PropBlk(&[Blk::Ascii]),
            false,
        )),
        Box::new(FilterCondition::new(
            "gc",
            FilterValue::PropGc(&[Gc::Zs]),
            true,
        )),
    ];
    FilterConditionOp::And(Box::leak(filters.into_boxed_slice()))
}

fn non_cjk_central_and_east_asian_characters() -> FilterCondition {
    FilterCondition::new(
        "blk",
        FilterValue::PropBlk(&[
            Blk::Mongolian,
            Blk::MongolianSup,
            Blk::TaiLe,
            Blk::NewTaiLue,
            Blk::Kangxi,
            Blk::Jamo,
            Blk::JamoExtA,
            Blk::JamoExtB,
            Blk::Hiragana,
            Blk::Katakana,
            Blk::KatakanaExt,
            Blk::Bopomofo,
            Blk::BopomofoExt,
            Blk::CompatJamo,
            Blk::Kanbun,
            Blk::Yijing,
            Blk::YiSyllables,
            Blk::YiRadicals,
            Blk::Vai,
            Blk::ModifierToneLetters,
            Blk::Hangul,
            Blk::Nushu,
            Blk::TaiXuanJing,
        ]),
        true,
    )
}

fn other_latin_characters() -> FilterConditionOp {
    let filters: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition::new(
            "blk",
            FilterValue::PropBlk(&[
                Blk::Latin1Sup,
                Blk::LatinExtB,
                Blk::LatinExtC,
                Blk::LatinExtD,
                Blk::LatinExtE,
            ]),
            true,
        )),
        Box::new(FilterCondition::new(
            "gc",
            FilterValue::PropGc(&[Gc::Lm, Gc::Lo, Gc::Lt]),
            true,
        )),
    ];
    FilterConditionOp::And(Box::leak(filters.into_boxed_slice()))
}

fn south_and_southeast_asian_characters() -> FilterCondition {
    FilterCondition::new(
        "blk",
        FilterValue::PropBlk(&[
            Blk::Sinhala,
            Blk::SinhalaArchaicNumbers,
            Blk::Thai,
            Blk::Lao,
            Blk::Tibetan,
            Blk::Myanmar,
            Blk::MyanmarExtA,
            Blk::MyanmarExtB,
            Blk::Tagalog,
            Blk::Hanunoo,
            Blk::Buhid,
            Blk::Tagbanwa,
            Blk::Khmer,
            Blk::Limbu,
            Blk::KhmerSymbols,
            Blk::Buginese,
            Blk::TaiTham,
            Blk::Balinese,
            Blk::Sundanese,
            Blk::SundaneseSup,
            Blk::Batak,
            Blk::SylotiNagri,
            Blk::PhagsPa,
            Blk::KayahLi,
            Blk::Rejang,
            Blk::Javanese,
            Blk::Cham,
            Blk::TaiViet,
            Blk::Lisu,
            Blk::Chakma,
            Blk::Khojki,
            Blk::Multani,
            Blk::Khudawadi,
            Blk::Newa,
            Blk::Tirhuta,
            Blk::Siddham,
            Blk::Takri,
            Blk::ZanabazarSquare,
            Blk::Soyombo,
            Blk::PauCinHau,
            Blk::Marchen,
            Blk::Mro,
            Blk::PahawhHmong,
        ]),
        true,
    )
}

fn west_asian_and_middle_eastern_characters() -> FilterCondition {
    FilterCondition::new(
        "blk",
        FilterValue::PropBlk(&[
            Blk::Arabic,
            Blk::ArabicPfA,
            Blk::Carian,
            Blk::Lycian,
            Blk::Ugaritic,
            Blk::OldPersian,
            Blk::Avestan,
            Blk::Cuneiform,
            Blk::CuneiformNumbers,
            Blk::EarlyDynasticCuneiform,
            Blk::Tangut,
            Blk::TangutComponents,
            Blk::AnatolianHieroglyphs,
        ]),
        true,
    )
}

pub fn generate_code_point_categories() -> Vec<CodePointCategory> {
    let all_code_points = generate_all_code_points();

    let mut categories: Vec<CodePointCategory> = vec![
        CodePointCategory::new(
            "ASCII Lowercase Letters",
            true,
            Box::new(ascii_lowercase_letters()),
            1,
        ),
        CodePointCategory::new(
            "ASCII Uppercase Letters",
            true,
            Box::new(ascii_uppercase_letters()),
            1,
        ),
        CodePointCategory::new("ASCII Digits", true, Box::new(ascii_digits()), 1),
        CodePointCategory::new(
            "ASCII Basic Symbols",
            true,
            Box::new(ascii_basic_symbols()),
            1,
        ),
        CodePointCategory::new(
            "ASCII Extended Symbols",
            true,
            Box::new(ascii_extended_symbols()),
            1,
        ),
        CodePointCategory::new("ASCII Space", true, Box::new(ascii_space()), 1),
        CodePointCategory::new(
            "African Characters",
            true,
            Box::new(african_characters()),
            1,
        ),
        CodePointCategory::new("CJK Characters", true, Box::new(cjk_characters()), 1),
        CodePointCategory::new(
            "European Characters",
            true,
            Box::new(european_characters()),
            1,
        ),
        CodePointCategory::new(
            "African Characters",
            true,
            Box::new(african_characters()),
            1,
        ),
        CodePointCategory::new("Extended Numbers", true, Box::new(extended_numbers()), 1),
        CodePointCategory::new(
            "Extended Symbols and Emojis",
            true,
            Box::new(extended_symbols_and_emojis()),
            1,
        ),
        CodePointCategory::new(
            "First Nations Characters",
            true,
            Box::new(first_nations()),
            1,
        ),
        CodePointCategory::new("Indian Characters", true, Box::new(indian_characters()), 1),
        CodePointCategory::new(
            "IPA Extended Characters and Modifier Letters",
            true,
            Box::new(ipa_extended_chars_and_modifier_letters()),
            1,
        ),
        CodePointCategory::new(
            "Latin Lowercase Characters",
            true,
            Box::new(latin_lowercase_characters()),
            1,
        ),
        CodePointCategory::new(
            "Latin Uppercase Characters",
            true,
            Box::new(latin_uppercase_letters()),
            1,
        ),
        CodePointCategory::new("Latin Symbols", true, Box::new(latin_symbols()), 1),
        CodePointCategory::new(
            "Other Latin Characters",
            true,
            Box::new(other_latin_characters()),
            1,
        ),
        CodePointCategory::new(
            "Linear A and Linear B Characters",
            true,
            Box::new(linear_a_linear_b_characters()),
            1,
        ),
        CodePointCategory::new("Non-ASCII Space", true, Box::new(non_ascii_space()), 1),
        CodePointCategory::new(
            "Non-CJK Central and East Asian Characters",
            true,
            Box::new(non_cjk_central_and_east_asian_characters()),
            1,
        ),
        CodePointCategory::new(
            "South and Southeast Asian Characters",
            true,
            Box::new(south_and_southeast_asian_characters()),
            1,
        ),
        CodePointCategory::new(
            "West Asian and Middle Eastern Characters",
            true,
            Box::new(west_asian_and_middle_eastern_characters()),
            1,
        ),
    ];

    for category in &mut categories {
        category.code_points = all_code_points
            .iter()
            .filter(|cpc| cpc.matches_filter_condition_op(&category.filters))
            .cloned()
            .collect();
    }

    categories
}
