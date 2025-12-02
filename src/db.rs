use chrono::NaiveDateTime;
use sqlx::{FromRow, MySql, Pool};

use crate::models::{
    Allergy, Condition, IPSModel, Immunization, Medication, Observation, Patient,
};

pub type MySqlPool = Pool<MySql>;

/// Row as stored in the `ipsAlt` table (flat, like your Sequelize IPSModel)
#[derive(Debug, FromRow)]
struct IPSRow {
    #[sqlx(rename = "id")]
    id: i64, // `id` is the PK (AUTO_INCREMENT) on ipsAlt

    #[sqlx(rename = "packageUUID")]
    package_uuid: String,

    #[sqlx(rename = "timeStamp")]
    time_stamp: NaiveDateTime,

    #[sqlx(rename = "patientName")]
    patient_name: String,

    #[sqlx(rename = "patientGiven")]
    patient_given: String,

    #[sqlx(rename = "patientDob")]
    patient_dob: NaiveDateTime,

    #[sqlx(rename = "patientGender")]
    patient_gender: Option<String>,

    #[sqlx(rename = "patientNation")]
    patient_nation: String,

    #[sqlx(rename = "patientPractitioner")]
    patient_practitioner: String,

    #[sqlx(rename = "patientOrganization")]
    patient_organization: Option<String>,

    #[sqlx(rename = "patientIdentifier")]
    patient_identifier: Option<String>,

    #[sqlx(rename = "patientIdentifier2")]
    patient_identifier2: Option<String>,
}

/// Row for child tables
#[derive(Debug, FromRow)]
struct MedicationRow {
    #[sqlx(rename = "med_name")]
    name: String,

    #[sqlx(rename = "med_date")]
    date: NaiveDateTime,

    #[sqlx(rename = "med_dosage")]
    dosage: String,

    // nullable in DB -> Option<String>
    #[sqlx(rename = "med_system")]
    system: Option<String>,

    #[sqlx(rename = "med_code")]
    code: Option<String>,

    #[sqlx(rename = "med_status")]
    status: Option<String>,
}

#[derive(Debug, FromRow)]
struct AllergyRow {
    #[sqlx(rename = "all_name")]
    name: String,

    #[sqlx(rename = "all_criticality")]
    criticality: Option<String>,

    #[sqlx(rename = "all_date")]
    date: NaiveDateTime,

    #[sqlx(rename = "all_system")]
    system: Option<String>,

    #[sqlx(rename = "all_code")]
    code: Option<String>,
}

#[derive(Debug, FromRow)]
struct ConditionRow {
    #[sqlx(rename = "con_name")]
    name: String,

    #[sqlx(rename = "con_date")]
    date: NaiveDateTime,

    #[sqlx(rename = "con_system")]
    system: Option<String>,

    #[sqlx(rename = "con_code")]
    code: Option<String>,
}

#[derive(Debug, FromRow)]
struct ObservationRow {
    #[sqlx(rename = "ob_name")]
    name: String,

    #[sqlx(rename = "ob_date")]
    date: NaiveDateTime,

    #[sqlx(rename = "ob_value")]
    value: Option<String>,

    #[sqlx(rename = "ob_system")]
    system: Option<String>,

    #[sqlx(rename = "ob_code")]
    code: Option<String>,

    #[sqlx(rename = "valueCode")]
    value_code: Option<String>,

    #[sqlx(rename = "bodySite")]
    body_site: Option<String>,

    #[sqlx(rename = "ob_status")]
    status: Option<String>,
}

#[derive(Debug, FromRow)]
struct ImmunizationRow {
    #[sqlx(rename = "imm_name")]
    name: String,

    #[sqlx(rename = "imm_system")]
    system: Option<String>,

    #[sqlx(rename = "imm_date")]
    date: NaiveDateTime,

    #[sqlx(rename = "imm_code")]
    code: Option<String>,

    #[sqlx(rename = "imm_status")]
    status: Option<String>,
}

pub async fn load_ips(
    package_uuid: &str,
    pool: &MySqlPool,
) -> Result<Option<IPSModel>, sqlx::Error> {
    // 1. Fetch main IPS row
    let ips_row: Option<IPSRow> =
        sqlx::query_as::<_, IPSRow>(r#"SELECT * FROM `ipsAlt` WHERE `packageUUID` = ? LIMIT 1"#)
            .bind(package_uuid)
            .fetch_optional(pool)
            .await?;

    let Some(ips_row) = ips_row else {
        return Ok(None);
    };

    // 2. Fetch child records using the FK (assumed IPSModelId)
    let medications_rows: Vec<MedicationRow> = sqlx::query_as::<_, MedicationRow>(
        r#"SELECT
               `name`   AS med_name,
               `date`   AS med_date,
               `dosage` AS med_dosage,
               `system` AS med_system,
               `code`   AS med_code,
               `status` AS med_status
           FROM `Medications`
           WHERE `IPSModelId` = ?"#,
    )
    .bind(ips_row.id)
    .fetch_all(pool)
    .await?;

    let allergies_rows: Vec<AllergyRow> = sqlx::query_as::<_, AllergyRow>(
        r#"SELECT
               `name`        AS all_name,
               `criticality` AS all_criticality,
               `date`        AS all_date,
               `system`      AS all_system,
               `code`        AS all_code
           FROM `Allergies`
           WHERE `IPSModelId` = ?"#,
    )
    .bind(ips_row.id)
    .fetch_all(pool)
    .await?;

    let conditions_rows: Vec<ConditionRow> = sqlx::query_as::<_, ConditionRow>(
        r#"SELECT
               `name`   AS con_name,
               `date`   AS con_date,
               `system` AS con_system,
               `code`   AS con_code
           FROM `Conditions`
           WHERE `IPSModelId` = ?"#,
    )
    .bind(ips_row.id)
    .fetch_all(pool)
    .await?;

    let observations_rows: Vec<ObservationRow> = sqlx::query_as::<_, ObservationRow>(
        r#"SELECT
               `name`      AS ob_name,
               `date`      AS ob_date,
               `value`     AS ob_value,
               `system`    AS ob_system,
               `code`      AS ob_code,
               `valueCode` AS valueCode,
               `bodySite`  AS bodySite,
               `status`    AS ob_status
           FROM `Observations`
           WHERE `IPSModelId` = ?"#,
    )
    .bind(ips_row.id)
    .fetch_all(pool)
    .await?;

    let immunizations_rows: Vec<ImmunizationRow> = sqlx::query_as::<_, ImmunizationRow>(
        r#"SELECT
               `name`   AS imm_name,
               `system` AS imm_system,
               `date`   AS imm_date,
               `code`   AS imm_code,
               `status` AS imm_status
           FROM `Immunizations`
           WHERE `IPSModelId` = ?"#,
    )
    .bind(ips_row.id)
    .fetch_all(pool)
    .await?;

    // 3. Map rows into API structs
    let patient = Patient {
        name: ips_row.patient_name,
        given: ips_row.patient_given,
        dob: ips_row.patient_dob,
        gender: ips_row.patient_gender,
        nation: ips_row.patient_nation,
        practitioner: ips_row.patient_practitioner,
        organization: ips_row.patient_organization,
        identifier: ips_row.patient_identifier,
        identifier2: ips_row.patient_identifier2,
    };

    let medications = medications_rows
        .into_iter()
        .map(|m| Medication {
            name: m.name,
            date: m.date,
            dosage: m.dosage,
            system: m.system.unwrap_or_default(),
            code: m.code.unwrap_or_default(),
            status: m.status.unwrap_or_default(),
        })
        .collect();

    let allergies = allergies_rows
        .into_iter()
        .map(|a| Allergy {
            name: a.name,
            criticality: a.criticality.unwrap_or_default(),
            date: a.date,
            system: a.system.unwrap_or_default(),
            code: a.code.unwrap_or_default(),
        })
        .collect();

    let conditions = conditions_rows
        .into_iter()
        .map(|c| Condition {
            name: c.name,
            date: c.date,
            system: c.system.unwrap_or_default(),
            code: c.code.unwrap_or_default(),
        })
        .collect();

    let observations = observations_rows
        .into_iter()
        .map(|o| Observation {
            name: o.name,
            date: o.date,
            value: o.value.unwrap_or_default(),
            system: o.system.unwrap_or_default(),
            code: o.code.unwrap_or_default(),
            value_code: o.value_code.unwrap_or_default(),
            body_site: o.body_site.unwrap_or_default(),
            status: o.status.unwrap_or_default(),
        })
        .collect();

    let immunizations = immunizations_rows
        .into_iter()
        .map(|i| Immunization {
            name: i.name,
            system: i.system.unwrap_or_default(),
            date: i.date,
            code: i.code.unwrap_or_default(),
            status: i.status.unwrap_or_default(),
        })
        .collect();

    let ips_model = IPSModel {
        package_uuid: ips_row.package_uuid,
        time_stamp: ips_row.time_stamp,
        patient,
        medications,
        allergies,
        conditions,
        observations,
        immunizations,
    };

    Ok(Some(ips_model))
}
