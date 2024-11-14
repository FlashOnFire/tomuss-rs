use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use serde::Deserialize;
use std::collections::HashMap;

use crate::tomuss_deser_utils::{deser_grades_vec, deser_int_as_bool, WrappedStrF32};

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TomussData {
    explanation: String,
    message: String,
    more_on_suivi: String,
    logo: String,
    go_home: i32,
    grp_messages: Vec<String>,
    advertising: bool,
    #[serde(rename = "DA")]
    da: Vec<String>,
    abjs: Vec<JustifiedAbsence>,
    pub(crate) login: String,
    #[serde(rename = "RdV")]
    rdv: String,
    compte: String,
    charte: String,
    semesters: HashMap<String, String>,
    signature: String,
    #[serde(rename = "FST")]
    fst: serde_json::Value,
    #[serde(rename = "BilanAPOGEE")]
    bilan_apogee: String,
    set_referent: String,
    bilan: String,
    #[serde(rename = "DateDeNaissance")]
    birth_date: String,
    #[serde(rename = "EDT")]
    edt: String,
    //ACLS null
    picture_upload: bool,
    preferences: HashMap<String, i32>,
    civilite: String,
    names: Person,
    member_of: Vec<serde_json::Value>,

    #[serde(rename = "IA_scol")]
    ia_scol: String,
    tables: String,
    notes: String,
    students: String,

    #[serde(rename = "FFSU")]
    ffsu: String,

    #[serde(rename = "TT")]
    tt: String,

    #[serde(rename = "RSS")]
    rss: String,

    #[serde(rename = "RSSStream")]
    rss_stream: Vec<serde_json::Value>,

    questionnaire: String,

    #[serde(rename = "UETree")]
    ue_tree: HashMap<String, String>,

    #[serde(rename = "ReferentNP")]
    referent_np: String,

    referent: serde_json::Value,

    mails: String,

    #[serde(rename = "choix_TVL")]
    choix_tvl: String,

    #[serde(rename = "IPAnnuelle")]
    ip_annuelle: HashMap<String, serde_json::Value>,

    #[serde(rename = "P_template")]
    p_template: Vec<serde_json::Value>,

    #[serde(deserialize_with = "deser_grades_vec")]
    pub(crate) grades: Vec<Grade>,

    profiling: HashMap<String, i32>,
}

impl TomussData {
    pub fn new(source_json: &str) -> TomussData {
        let data: Vec<Vec<serde_json::Value>> = serde_json::from_str(&source_json).unwrap();

        let data: HashMap<String, serde_json::Value> = data
            .into_par_iter()
            .map(|inner_vec| {
                (
                    serde_json::from_value::<String>(inner_vec.first().unwrap().clone()).unwrap(),
                    inner_vec.get(1).unwrap().clone(),
                )
            })
            .collect();

        serde_json::to_value(data)
            .ok()
            .and_then(|x| serde_path_to_error::deserialize(x).unwrap())
            .unwrap()
    }
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Grade {
    masters: Vec<Person>,
    rounding: i32,
    contains_users: i32,
    default_sort_column: Vec<i32>,
    competence: String,
    table_title: String,
    popup_on_red_line: i32,
    managers: Vec<String>,
    code: i32,
    bookmark: i32,
    dates: Vec<f32>,
    official_ue: i32,
    ue: String,
    year: i32,
    columns: Vec<GradeColumn>,
    line_id: String,
    line: Vec<serde_json::Value>,
    stats: serde_json::Value,
}

#[derive(Debug, Clone)]
pub(crate) struct Person {
    pub(crate) name: String,
    pub(crate) surname: String,
    pub(crate) mail: String,
}

#[derive(Deserialize, Debug, Clone, Default)]
enum Freezed {
    #[default]
    None,
    F,
    C,
}

#[derive(Deserialize, Debug, Clone)]
struct GradeColumn {
    #[serde(rename = "type")]
    type_: GradeType,
    author: String,
    #[serde(default)]
    freezed: Freezed,
    position: f32,
    title: String,
    width: Option<i32>,
    comment: Option<String>,
    the_id: String,
    #[serde(deserialize_with = "deser_int_as_bool", default)]
    hidden: bool,
    repetition: Option<i32>,
    weight: Option<WrappedStrF32>,
    visibility_date: Option<String>,
    green: Option<String>,
    red: Option<String>,
    enumeration: Option<String>,
    cell_writable: Option<String>,
    grade_type: Option<i32>,
}

#[derive(Deserialize, Debug, Clone)]
enum GradeType {
    Text,
    Note,
    Login,
    Moy,
    Prst,
    Enumeration,
    Upload,
    Max,
}

#[derive(Debug, Clone)]
pub(crate) struct JustifiedAbsence {
    pub(crate) start: String,
    pub(crate) end: String,
    pub(crate) comment: String,
}