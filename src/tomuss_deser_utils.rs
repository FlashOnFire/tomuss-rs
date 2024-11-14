use crate::structs::{Grade, JustifiedAbsence, Person};
use serde::de::{Error, Unexpected};
use serde::{de, Deserialize, Deserializer};
use std::str::FromStr;

pub(crate) fn deser_grades_vec<'de, D>(deserializer: D) -> Result<Vec<Grade>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let vec: Vec<Vec<_>> = serde::Deserialize::deserialize(deserializer)?;
    if vec.is_empty() || vec.len() != 2 || !vec.get(1).unwrap().is_empty() {
        return Err(D::Error::custom("Grades deserializing error"));
    }

    Ok(vec.iter().flatten().cloned().collect())
}

impl<'de> Deserialize<'de> for Person {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let vec: Vec<String> = serde::Deserialize::deserialize(deserializer)?;
        if vec.len() != 3 {
            return Err(D::Error::invalid_length(vec.len(), &"3"));
        }
        let mail = vec.get(2).unwrap();
        if !mail.contains("@") {
            return Err(D::Error::invalid_value(Unexpected::Str(mail), &"Missing @"));
        }

        Ok(Self {
            name: vec.get(0).unwrap().clone(),
            surname: vec.get(1).unwrap().clone(),
            mail: mail.clone(),
        })
    }
}

pub(crate) fn deser_int_as_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let n: i32 = serde::Deserialize::deserialize(deserializer)?;
    match n {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(D::Error::invalid_value(
            Unexpected::Signed(n.into()),
            &"zero or one",
        )),
    }
}

#[derive(Clone, Debug)]
pub struct WrappedStrF32(f32);
impl<'de> Deserialize<'de> for WrappedStrF32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = String::deserialize(deserializer)?;
        match f32::from_str(str.as_str()) {
            Ok(value) => Ok(Self(value)),
            Err(_) => Err(D::Error::invalid_value(Unexpected::Str(&str), &"f32")),
        }
    }
}

impl<'de> Deserialize<'de> for JustifiedAbsence {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let vec: Vec<String> = serde::Deserialize::deserialize(deserializer)?;
        if vec.len() != 3 {
            return Err(D::Error::invalid_length(vec.len(), &"3"));
        }

        Ok(Self {
            start: vec.get(0).unwrap().clone(),
            end: vec.get(1).unwrap().clone(),
            comment: vec.get(2).unwrap().clone(),
        })
    }
}
