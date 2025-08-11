use seshat::unicode::{
    CodePoint, Ucd,
    props::{Age, Bc, Blk, Gc, Gcb, Insc, Sc},
};

use std::any::Any;

#[derive(Clone, Copy)]
pub struct CodePointConfig {
    enabled: bool,
    code_point: CodePoint,
}

#[derive(Clone, Copy)]
pub enum FilterValue {
    PropAge(&'static [Age]),
    PropBc(&'static [Bc]),
    PropBinary(bool),
    PropBlk(&'static [Blk]),
    // PropCcc(&'static [Ccc]),
    // PropDt(&'static [Dt]),
    PropGc(&'static [Gc]),
    PropGcb(&'static [Gcb]),
    // PropHst(&'static [Hst]),
    // PropIncb(&'static [Incb]),
    PropInsc(&'static [Insc]),
    PropSc(&'static [Sc]),
    // PropString(&'static [String]),
    PropU32(&'static [u32]),
    // PropWb(&'static [Wb]),
}

#[allow(dead_code)]
pub trait Filterable: Any {
    fn get_type(&self) -> &str;
}

#[derive(Copy)]
pub struct FilterCondition {
    func: &'static str,
    values: FilterValue,
    include: bool,
}

impl FilterCondition {
    pub fn new(func: &'static str, values: FilterValue, include: bool) -> Self {
        FilterCondition {
            func,
            values,
            include,
        }
    }
}

impl Filterable for FilterCondition {
    fn get_type(&self) -> &str {
        "FilterCondition"
    }
}

impl Clone for FilterCondition {
    fn clone(&self) -> Self {
        FilterCondition {
            func: self.func,
            values: self.values.clone(),
            include: self.include,
        }
    }
}

pub enum FilterConditionOp {
    And(&'static [Box<dyn Filterable>]),
    Or(&'static [Box<dyn Filterable>]),
    Not(Box<dyn Filterable>),
}

impl Filterable for FilterConditionOp {
    fn get_type(&self) -> &str {
        match *self {
            FilterConditionOp::And(_) => "FilterConditionOp::And",
            FilterConditionOp::Not(_) => "FilterConditionOp::Not",
            FilterConditionOp::Or(_) => "FilterConditionOp::Or",
        }
    }
}

impl CodePointConfig {
    pub fn get_char(&self) -> char {
        char::from_u32(self.code_point.to_u32()).unwrap_or(char::REPLACEMENT_CHARACTER)
    }

    fn matches_filter_condition(&self, fc: &FilterCondition) -> bool {
        let m = match &fc.values {
            FilterValue::PropAge(age) => age.contains(&self.code_point.age()),
            FilterValue::PropBc(bc) => bc.contains(&self.code_point.bc()),
            FilterValue::PropBinary(b) => match fc.func {
                "ahex" => self.code_point.ahex() == *b,
                "alpha" => self.code_point.alpha() == *b,
                "bidi_c" => self.code_point.bidi_c() == *b,
                "cased" => self.code_point.cased() == *b,
                "ce" => self.code_point.ce() == *b,
                "ci" => self.code_point.ci() == *b,
                "comp_ex" => self.code_point.comp_ex() == *b,
                "dash" => self.code_point.dash() == *b,
                "dep" => self.code_point.dep() == *b,
                "di" => self.code_point.di() == *b,
                "dia" => self.code_point.dia() == *b,
                "ebase" => self.code_point.ebase() == *b,
                "ecomp" => self.code_point.ecomp() == *b,
                "emod" => self.code_point.emod() == *b,
                "emoji" => self.code_point.emoji() == *b,
                "epres" => self.code_point.epres() == *b,
                "ext_pict" => self.code_point.ext_pict() == *b,
                "ext" => self.code_point.ext() == *b,
                "gr_ext" => self.code_point.gr_ext() == *b,
                "hex" => self.code_point.hex() == *b,
                "hyphen" => self.code_point.hyphen() == *b,
                "idc" => self.code_point.idc() == *b,
                "ideo" => self.code_point.ideo() == *b,
                "ids" => self.code_point.ids() == *b,
                "idsb" => self.code_point.idsb() == *b,
                "idst" => self.code_point.idst() == *b,
                "join_c" => self.code_point.join_c() == *b,
                "loe" => self.code_point.loe() == *b,
                "lower" => self.code_point.lower() == *b,
                "math" => self.code_point.math() == *b,
                "nchar" => self.code_point.nchar() == *b,
                "oalpha" => self.code_point.oalpha() == *b,
                "odi" => self.code_point.odi() == *b,
                "ogr_ext" => self.code_point.ogr_ext() == *b,
                "oidc" => self.code_point.oidc() == *b,
                "oids" => self.code_point.oids() == *b,
                "olower" => self.code_point.olower() == *b,
                "omath" => self.code_point.omath() == *b,
                "oupper" => self.code_point.oupper() == *b,
                "pat_syn" => self.code_point.pat_syn() == *b,
                "pat_ws" => self.code_point.pat_ws() == *b,
                "pcm" => self.code_point.pcm() == *b,
                "qmark" => self.code_point.qmark() == *b,
                "radical" => self.code_point.radical() == *b,
                "ri" => self.code_point.ri() == *b,
                "sd" => self.code_point.sd() == *b,
                "sterm" => self.code_point.sterm() == *b,
                "term" => self.code_point.term() == *b,
                "uideo" => self.code_point.uideo() == *b,
                "upper" => self.code_point.upper() == *b,
                "vs" => self.code_point.vs() == *b,
                "wspace" => self.code_point.wspace() == *b,
                "xidc" => self.code_point.xidc() == *b,
                "xids" => self.code_point.xids() == *b,
                _ => false,
            },
            FilterValue::PropBlk(blk) => blk.contains(&self.code_point.blk()),
            // FilterValue::PropCcc(ccc) => ccc.contains(&self.code_point.ccc()),
            // FilterValue::PropDt(dt) => dt.contains(&self.code_point.dt()),
            FilterValue::PropGc(gc) => gc.contains(&self.code_point.gc()),
            FilterValue::PropGcb(gcb) => gcb.contains(&self.code_point.gcb()),
            // FilterValue::PropHst(hst) => hst.contains(&self.code_point.hst()),
            // FilterValue::PropIncb(incb) => incb.contains(&self.code_point.incb()),
            FilterValue::PropInsc(insc) => insc.contains(&self.code_point.insc()),
            FilterValue::PropSc(sc) => sc.contains(&self.code_point.sc()),
            // FilterValue::PropString(s) => match fc.func {
            //     "dm" => s.contains(&self.code_point.dm()),
            //     "na" => s.contains(&self.code_point.na()),
            //     _ => false,
            // },
            FilterValue::PropU32(id) => id.contains(&self.code_point.to_u32()),
            // FilterValue::PropWb(wb) => wb.contains(&self.code_point.wb()),
        };
        m == fc.include
    }

    pub fn matches_filter_condition_op(&self, f: &dyn Any) -> bool {
        if let Some(fb) = f.downcast_ref::<Box<dyn Filterable>>() {
            if let Some(fc) = (&**fb as &dyn Any).downcast_ref::<FilterCondition>() {
                self.matches_filter_condition(fc)
            } else if let Some(fc_op) = (&**fb as &dyn Any).downcast_ref::<FilterConditionOp>() {
                match fc_op {
                    FilterConditionOp::And(fcs) => {
                        fcs.iter().all(|fc| self.matches_filter_condition_op(fc))
                    }
                    FilterConditionOp::Or(fcs) => {
                        fcs.iter().any(|fc| self.matches_filter_condition_op(fc))
                    }
                    FilterConditionOp::Not(fc) => !self.matches_filter_condition_op(fc),
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub fn update_char_status(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

static NON_DEPRECATED_CHARACTERS: FilterCondition = FilterCondition {
    func: "dep",
    values: FilterValue::PropBinary(false),
    include: true,
};

static CATEGORY_EXCLUSIONS: FilterCondition = FilterCondition {
    func: "gc",
    values: FilterValue::PropGc(&[
        Gc::Cc,
        Gc::Cf,
        Gc::Cn,
        Gc::Co,
        Gc::Cs,
        Gc::Mc,
        Gc::Mn,
        Gc::Zl,
        Gc::Zp,
    ]),
    include: false,
};

static BIDI_CLASS_EXCLUSIONS: FilterCondition = FilterCondition {
    func: "bc",
    values: FilterValue::PropBc(&[
        Bc::AL,
        Bc::AN,
        Bc::B,
        Bc::BN,
        Bc::FSI,
        Bc::LRE,
        Bc::LRI,
        Bc::LRO,
        Bc::NSM,
        Bc::PDF,
        Bc::PDI,
        Bc::R,
        Bc::RLE,
        Bc::RLI,
        Bc::RLO,
        Bc::S,
    ]),
    include: false,
};

static GRAPHEME_CLUSTER_BREAK_EXCLUSIONS: FilterCondition = {
    FilterCondition {
        func: "gcb",
        values: FilterValue::PropGcb(&[Gcb::PP]),
        include: false,
    }
};

static BLOCK_EXCLUSIONS: FilterCondition = FilterCondition {
    func: "blk",
    values: FilterValue::PropBlk(&[
        Blk::CjkCompatIdeographsSup,
        Blk::CjkExtB,
        Blk::CjkExtC,
        Blk::CjkExtD,
        Blk::CjkExtE,
        Blk::CjkExtF,
        Blk::CjkExtG,
        Blk::CjkExtH,
        Blk::CjkExtI,
        Blk::CyproMinoan,
        Blk::CyrillicExtD,
        Blk::DivesAkuru,
        Blk::EthiopicExtB,
        Blk::EgyptianHieroglyphFormatControls,
        Blk::EgyptianHieroglyphsExtA,
        Blk::Garay,
        Blk::GurungKhema,
        Blk::KaktovikNumerals,
        Blk::KanaExtA,
        Blk::KanaExtB,
        Blk::KanaSup,
        Blk::Kawi,
        Blk::KhitanSmallScript,
        Blk::KiratRai,
        Blk::LatinExtF,
        Blk::LatinExtG,
        Blk::Makasar,
        Blk::MyanmarExtC,
        Blk::NagMundari,
        Blk::Nandinagari,
        Blk::OlOnal,
        Blk::SmallKanaExt,
        Blk::Specials,
        Blk::Sunuwar,
        Blk::SymbolsForLegacyComputingSup,
        Blk::Tangsa,
        Blk::TangutSup,
        Blk::Todhri,
        Blk::Toto,
        Blk::TuluTigalari,
        Blk::UcasExtA,
        Blk::Vithkuqi,
        Blk::ZnamennyMusic,
    ]),
    include: false,
};

static INDIC_SYLLABIC_CATEGORY_EXCLUSIONS: FilterCondition = FilterCondition {
    func: "insc",
    values: FilterValue::PropInsc(&[Insc::Consonant_With_Stacker]),
    include: false,
};

fn balinese_character_exclusions() -> FilterConditionOp {
    let conditions: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition {
            func: "sc",
            values: FilterValue::PropBlk(&[Blk::Balinese]),
            include: true,
        }),
        Box::new(FilterCondition {
            func: "age",
            values: FilterValue::PropAge(&[Age::V5_0]),
            include: false,
        }),
    ];
    FilterConditionOp::Not(Box::new(FilterConditionOp::And(Box::leak(
        conditions.into_boxed_slice(),
    ))))
}

fn brahmi_character_exclusions() -> FilterConditionOp {
    let conditions: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition {
            func: "blk",
            values: FilterValue::PropBlk(&[Blk::Brahmi]),
            include: true,
        }),
        Box::new(FilterCondition {
            func: "age",
            values: FilterValue::PropAge(&[Age::V6_0]),
            include: false,
        }),
    ];
    FilterConditionOp::Not(Box::new(FilterConditionOp::And(Box::leak(
        conditions.into_boxed_slice(),
    ))))
}

fn chakma_character_exclusions() -> FilterConditionOp {
    let conditions: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition {
            func: "blk",
            values: FilterValue::PropBlk(&[Blk::Chakma]),
            include: true,
        }),
        Box::new(FilterCondition {
            func: "age",
            values: FilterValue::PropAge(&[Age::V6_1, Age::V11_0]),
            include: false,
        }),
    ];
    FilterConditionOp::Not(Box::new(FilterConditionOp::And(Box::leak(
        conditions.into_boxed_slice(),
    ))))
}

fn takri_character_exclusions() -> FilterConditionOp {
    let conditions: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition {
            func: "blk",
            values: FilterValue::PropBlk(&[Blk::Chakma]),
            include: true,
        }),
        Box::new(FilterCondition {
            func: "age",
            values: FilterValue::PropAge(&[Age::V6_1, Age::V12_0]),
            include: false,
        }),
    ];
    FilterConditionOp::Not(Box::new(FilterConditionOp::And(Box::leak(
        conditions.into_boxed_slice(),
    ))))
}

fn ahom_character_exclusions() -> FilterConditionOp {
    let conditions: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition {
            func: "blk",
            values: FilterValue::PropBlk(&[Blk::Chakma]),
            include: true,
        }),
        Box::new(FilterCondition {
            func: "age",
            values: FilterValue::PropAge(&[Age::V8_0, Age::V11_0]),
            include: false,
        }),
    ];
    FilterConditionOp::Not(Box::new(FilterConditionOp::And(Box::leak(
        conditions.into_boxed_slice(),
    ))))
}

fn egyptian_hieroglyphs_character_exclusions() -> FilterConditionOp {
    let conditions: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition {
            func: "blk",
            values: FilterValue::PropBlk(&[Blk::EgyptianHieroglyphs]),
            include: true,
        }),
        Box::new(FilterCondition {
            func: "age",
            values: FilterValue::PropAge(&[Age::V5_2]),
            include: false,
        }),
    ];
    FilterConditionOp::Not(Box::new(FilterConditionOp::And(Box::leak(
        conditions.into_boxed_slice(),
    ))))
}

fn tangut_components_character_exclusions() -> FilterConditionOp {
    let conditions: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition {
            func: "blk",
            values: FilterValue::PropBlk(&[Blk::TangutComponents]),
            include: true,
        }),
        Box::new(FilterCondition {
            func: "age",
            values: FilterValue::PropAge(&[Age::V9_0]),
            include: false,
        }),
    ];
    FilterConditionOp::Not(Box::new(FilterConditionOp::And(Box::leak(
        conditions.into_boxed_slice(),
    ))))
}

fn music_character_exclusions() -> FilterConditionOp {
    let conditions: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition {
            func: "blk",
            values: FilterValue::PropBlk(&[Blk::TangutComponents]),
            include: true,
        }),
        Box::new(FilterCondition {
            func: "age",
            values: FilterValue::PropAge(&[Age::V3_1, Age::V8_0]),
            include: false,
        }),
    ];
    FilterConditionOp::Not(Box::new(FilterConditionOp::And(Box::leak(
        conditions.into_boxed_slice(),
    ))))
}

fn enclosed_alphanum_sup_character_exclusions() -> FilterConditionOp {
    let conditions: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition {
            func: "blk",
            values: FilterValue::PropBlk(&[Blk::EnclosedAlphanumSup]),
            include: true,
        }),
        Box::new(FilterCondition {
            func: "age",
            values: FilterValue::PropAge(&[
                Age::V5_2,
                Age::V6_0,
                Age::V6_1,
                Age::V7_0,
                Age::V9_0,
                Age::V11_0,
                Age::V12_0,
            ]),
            include: false,
        }),
    ];
    FilterConditionOp::Not(Box::new(FilterConditionOp::And(Box::leak(
        conditions.into_boxed_slice(),
    ))))
}

fn enclosed_ideographic_sup_character_exclusions() -> FilterConditionOp {
    let conditions: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition {
            func: "blk",
            values: FilterValue::PropBlk(&[Blk::EnclosedIdeographicSup]),
            include: true,
        }),
        Box::new(FilterCondition {
            func: "age",
            values: FilterValue::PropAge(&[Age::V5_2, Age::V6_0, Age::V9_0]),
            include: false,
        }),
    ];
    FilterConditionOp::Not(Box::new(FilterConditionOp::And(Box::leak(
        conditions.into_boxed_slice(),
    ))))
}

fn sup_arrows_c_character_exclusions() -> FilterConditionOp {
    let conditions: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition {
            func: "blk",
            values: FilterValue::PropBlk(&[Blk::SupArrowsC]),
            include: true,
        }),
        Box::new(FilterCondition {
            func: "age",
            values: FilterValue::PropAge(&[Age::V7_0, Age::V13_0]),
            include: false,
        }),
    ];
    FilterConditionOp::Not(Box::new(FilterConditionOp::And(Box::leak(
        conditions.into_boxed_slice(),
    ))))
}

fn symbols_for_legacy_computing_character_exclusions() -> FilterConditionOp {
    let conditions: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition {
            func: "blk",
            values: FilterValue::PropBlk(&[Blk::SupArrowsC]),
            include: true,
        }),
        Box::new(FilterCondition {
            func: "age",
            values: FilterValue::PropAge(&[Age::V13_0]),
            include: false,
        }),
    ];
    FilterConditionOp::Not(Box::new(FilterConditionOp::And(Box::leak(
        conditions.into_boxed_slice(),
    ))))
}

fn idc_character_exclusions() -> FilterConditionOp {
    let conditions: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition {
            func: "blk",
            values: FilterValue::PropBlk(&[Blk::Idc]),
            include: true,
        }),
        Box::new(FilterCondition {
            func: "age",
            values: FilterValue::PropAge(&[Age::V13_0]),
            include: false,
        }),
    ];
    FilterConditionOp::Not(Box::new(FilterConditionOp::And(Box::leak(
        conditions.into_boxed_slice(),
    ))))
}

fn bopomofo_ext_character_exclusions() -> FilterConditionOp {
    let conditions: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition {
            func: "blk",
            values: FilterValue::PropBlk(&[Blk::BopomofoExt]),
            include: true,
        }),
        Box::new(FilterCondition {
            func: "age",
            values: FilterValue::PropAge(&[Age::V3_0, Age::V6_0]),
            include: false,
        }),
    ];
    FilterConditionOp::Not(Box::new(FilterConditionOp::And(Box::leak(
        conditions.into_boxed_slice(),
    ))))
}

fn cjk_strokes_character_exclusions() -> FilterConditionOp {
    let conditions: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition {
            func: "blk",
            values: FilterValue::PropBlk(&[Blk::CjkStrokes]),
            include: true,
        }),
        Box::new(FilterCondition {
            func: "age",
            values: FilterValue::PropAge(&[Age::V4_1, Age::V5_1]),
            include: false,
        }),
    ];
    FilterConditionOp::Not(Box::new(FilterConditionOp::And(Box::leak(
        conditions.into_boxed_slice(),
    ))))
}

fn cjk_compat_ideographs_character_exclusions() -> FilterConditionOp {
    let conditions: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition {
            func: "blk",
            values: FilterValue::PropBlk(&[Blk::CjkCompatIdeographs]),
            include: true,
        }),
        Box::new(FilterCondition {
            func: "age",
            values: FilterValue::PropAge(&[Age::V1_1, Age::V3_2, Age::V5_2, Age::V6_1]),
            include: false,
        }),
    ];
    FilterConditionOp::Not(Box::new(FilterConditionOp::And(Box::leak(
        conditions.into_boxed_slice(),
    ))))
}

fn arabic_pf_a_character_exclusions() -> FilterConditionOp {
    let conditions: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition {
            func: "blk",
            values: FilterValue::PropBlk(&[Blk::ArabicPfA]),
            include: true,
        }),
        Box::new(FilterCondition {
            func: "age",
            values: FilterValue::PropAge(&[Age::V1_1, Age::V4_0]),
            include: false,
        }),
    ];
    FilterConditionOp::Not(Box::new(FilterConditionOp::And(Box::leak(
        conditions.into_boxed_slice(),
    ))))
}

fn latin_ext_d_character_exclusions() -> FilterConditionOp {
    let subset_1: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition {
            func: "age",
            values: FilterValue::PropAge(&[Age::V14_0]),
            include: true,
        }),
        Box::new(FilterCondition {
            func: "gc",
            values: FilterValue::PropGc(&[Gc::Lm]),
            include: true,
        }),
    ];

    let subset_2: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition {
            func: "age",
            values: FilterValue::PropAge(&[
                Age::V5_0,
                Age::V5_1,
                Age::V6_0,
                Age::V6_1,
                Age::V7_0,
                Age::V8_0,
                Age::V9_0,
                Age::V11_0,
                Age::V12_0,
                Age::V13_0,
            ]),
            include: true,
        }),
        Box::new(FilterConditionOp::And(Box::leak(
            subset_1.into_boxed_slice(),
        ))),
    ];

    let subset_3: Vec<Box<dyn Filterable>> = vec![
        Box::new(FilterCondition {
            func: "blk",
            values: FilterValue::PropBlk(&[Blk::LatinExtD]),
            include: true,
        }),
        Box::new(FilterConditionOp::Not(Box::new(FilterConditionOp::Or(
            Box::leak(subset_2.into_boxed_slice()),
        )))),
    ];

    FilterConditionOp::Not(Box::new(FilterConditionOp::And(Box::leak(
        subset_3.into_boxed_slice(),
    ))))
}

static GENERAL_CHARACTER_EXCLUSIONS: FilterCondition = FilterCondition {
    func: "u32",
    values: FilterValue::PropU32(&[0x332c, 0x1f908, 0x1f909, 0x1f90a, 0x1f90b]),
    include: false,
};

pub fn generate_all_code_points() -> Vec<CodePointConfig> {
    let filters_vec: Vec<Box<dyn Filterable>> = vec![
        Box::new(CATEGORY_EXCLUSIONS),
        Box::new(GRAPHEME_CLUSTER_BREAK_EXCLUSIONS),
        Box::new(NON_DEPRECATED_CHARACTERS),
        Box::new(BIDI_CLASS_EXCLUSIONS),
        Box::new(BLOCK_EXCLUSIONS),
        Box::new(INDIC_SYLLABIC_CATEGORY_EXCLUSIONS),
        Box::new(GENERAL_CHARACTER_EXCLUSIONS),
        Box::new(balinese_character_exclusions()),
        Box::new(brahmi_character_exclusions()),
        Box::new(chakma_character_exclusions()),
        Box::new(takri_character_exclusions()),
        Box::new(ahom_character_exclusions()),
        Box::new(egyptian_hieroglyphs_character_exclusions()),
        Box::new(tangut_components_character_exclusions()),
        Box::new(music_character_exclusions()),
        Box::new(enclosed_alphanum_sup_character_exclusions()),
        Box::new(enclosed_ideographic_sup_character_exclusions()),
        Box::new(sup_arrows_c_character_exclusions()),
        Box::new(symbols_for_legacy_computing_character_exclusions()),
        Box::new(idc_character_exclusions()),
        Box::new(bopomofo_ext_character_exclusions()),
        Box::new(cjk_strokes_character_exclusions()),
        Box::new(cjk_compat_ideographs_character_exclusions()),
        Box::new(arabic_pf_a_character_exclusions()),
        Box::new(latin_ext_d_character_exclusions()),
    ];

    (char::MIN as u32..=char::MAX as u32)
        .flat_map(|i| CodePoint::new(i).ok())
        .filter_map(|cp| {
            let cpc = CodePointConfig {
                enabled: true,
                code_point: cp,
            };
            filters_vec
                .iter()
                .all(|f| cpc.matches_filter_condition_op(f))
                .then(|| cpc)
        })
        .collect()
}
