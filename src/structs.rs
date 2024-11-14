use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use serde::Deserialize;
use std::collections::HashMap;

use crate::tomuss_deser_utils::deser_grades_vec;

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
    abjs: Vec<serde_json::Value>,
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
    names: Vec<String>,
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
    pub(crate) grades: Vec<serde_json::Value>,

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
            .and_then(|x| TomussData::deserialize(x).ok())
            .unwrap()
    }
}

