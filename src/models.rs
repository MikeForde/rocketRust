use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct IPSModel {
    pub package_uuid: String,
    pub time_stamp: NaiveDateTime,
    pub patient: Patient,
    pub medications: Vec<Medication>,
    pub allergies: Vec<Allergy>,
    pub conditions: Vec<Condition>,
    pub observations: Vec<Observation>,
    pub immunizations: Vec<Immunization>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Patient {
    pub name: String,
    pub given: String,
    pub dob: NaiveDateTime,
    pub gender: Option<String>,
    pub nation: String,
    pub practitioner: String,
    pub organization: Option<String>,
    pub identifier: Option<String>,
    pub identifier2: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Medication {
    pub name: String,
    pub date: NaiveDateTime,
    pub dosage: String,
    pub system: String,
    pub code: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Allergy {
    pub name: String,
    pub criticality: String,
    pub date: NaiveDateTime,
    pub system: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Condition {
    pub name: String,
    pub date: NaiveDateTime,
    pub system: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Observation {
    pub name: String,
    pub date: NaiveDateTime,
    pub value: String,
    pub system: String,
    pub code: String,
    pub value_code: String,
    pub body_site: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Immunization {
    pub name: String,
    pub system: String,
    pub date: NaiveDateTime,
    pub code: String,
    pub status: String,
}
