use seshat::unicode::props::{Blk, Gc, Sc};

use super::code_point::{
    CodePointConfig, FilterCondition, FilterConditionOp, FilterValue, Filterable,
    generate_all_code_points,
};

#[allow(dead_code)]
pub struct CodePointCategory {
    name: String,
    enabled: bool,
    filters: Box<dyn Filterable>,
    min_characters: usize,
    code_points: Vec<CodePointConfig>,
}

impl CodePointCategory {
    pub fn new(
        name: &str,
        enabled: bool,
        filters: Box<dyn Filterable>,
        min_characters: usize,
    ) -> Self {
        Self {
            name: name.to_string(),
            enabled,
            filters,
            min_characters,
            code_points: Vec::new(),
        }
    }

    pub fn get_min_chars(&self) -> usize {
        self.min_characters
    }

    pub fn get_active_code_points(&self) -> Vec<CodePointConfig> {
        self.code_points
            .iter()
            .filter(|cpc| cpc.is_enabled())
            .cloned()
            .collect()
    }

    pub fn get_active_chars(&self) -> Vec<char> {
        self.get_active_code_points()
            .iter()
            .map(|cpc| cpc.get_char())
            .collect()
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

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
            true,
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
        Box::new(FilterConditionOp::And(Box::leak(
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
        Box::new(FilterConditionOp::And(Box::leak(
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

pub fn generate_code_point_categories(include_extended: bool) -> Vec<CodePointCategory> {
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
            include_extended,
            Box::new(ascii_extended_symbols()),
            include_extended as usize,
        ),
        CodePointCategory::new(
            "ASCII Space",
            include_extended,
            Box::new(ascii_space()),
            include_extended as usize,
        ),
        CodePointCategory::new(
            "African Characters",
            include_extended,
            Box::new(african_characters()),
            include_extended as usize,
        ),
        CodePointCategory::new(
            "CJK Characters",
            include_extended,
            Box::new(cjk_characters()),
            include_extended as usize,
        ),
        CodePointCategory::new(
            "European Characters",
            include_extended,
            Box::new(european_characters()),
            include_extended as usize,
        ),
        CodePointCategory::new(
            "African Characters",
            include_extended,
            Box::new(african_characters()),
            include_extended as usize,
        ),
        CodePointCategory::new(
            "Extended Numbers",
            include_extended,
            Box::new(extended_numbers()),
            include_extended as usize,
        ),
        CodePointCategory::new(
            "Extended Symbols and Emojis",
            include_extended,
            Box::new(extended_symbols_and_emojis()),
            include_extended as usize,
        ),
        CodePointCategory::new(
            "First Nations Characters",
            include_extended,
            Box::new(first_nations()),
            include_extended as usize,
        ),
        CodePointCategory::new(
            "Indian Characters",
            include_extended,
            Box::new(indian_characters()),
            include_extended as usize,
        ),
        CodePointCategory::new(
            "IPA Extended Characters and Modifier Letters",
            include_extended,
            Box::new(ipa_extended_chars_and_modifier_letters()),
            include_extended as usize,
        ),
        CodePointCategory::new(
            "Latin Lowercase Characters",
            include_extended,
            Box::new(latin_lowercase_characters()),
            include_extended as usize,
        ),
        CodePointCategory::new(
            "Latin Uppercase Characters",
            include_extended,
            Box::new(latin_uppercase_letters()),
            include_extended as usize,
        ),
        CodePointCategory::new(
            "Latin Symbols",
            include_extended,
            Box::new(latin_symbols()),
            include_extended as usize,
        ),
        CodePointCategory::new(
            "Other Latin Characters",
            include_extended,
            Box::new(other_latin_characters()),
            include_extended as usize,
        ),
        CodePointCategory::new(
            "Linear A and Linear B Characters",
            include_extended,
            Box::new(linear_a_linear_b_characters()),
            include_extended as usize,
        ),
        CodePointCategory::new(
            "Non-ASCII Space",
            include_extended,
            Box::new(non_ascii_space()),
            include_extended as usize,
        ),
        CodePointCategory::new(
            "Non-CJK Central and East Asian Characters",
            include_extended,
            Box::new(non_cjk_central_and_east_asian_characters()),
            include_extended as usize,
        ),
        CodePointCategory::new(
            "South and Southeast Asian Characters",
            include_extended,
            Box::new(south_and_southeast_asian_characters()),
            include_extended as usize,
        ),
        CodePointCategory::new(
            "West Asian and Middle Eastern Characters",
            include_extended,
            Box::new(west_asian_and_middle_eastern_characters()),
            include_extended as usize,
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
