use serde::de;
use serde::de::Error;

pub(crate) fn deser_grades_vec<'de, D>(deserializer: D) -> Result<Vec<serde_json::Value>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let vec: Vec<Vec<serde_json::Value>> = serde::Deserialize::deserialize(deserializer)?;
    if vec.is_empty() || vec.len() != 2 || !vec.get(1).unwrap().is_empty() {
        return Err(D::Error::custom("Grades deserializing error"));
    }
    
    Ok(vec.iter().flatten().cloned().collect::<Vec<_>>())
}