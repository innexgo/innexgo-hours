use super::db_types::*;
use super::utils::current_time_millis;
use std::convert::From;
use tokio_postgres::GenericClient;

impl From<tokio_postgres::row::Row> for SchoolData {
  // select * from school_data order only, otherwise it will fail
  fn from(row: tokio_postgres::Row) -> SchoolData {
    SchoolData {
      school_data_id: row.get("school_data_id"),
      creation_time: row.get("creation_time"),
      creator_user_id: row.get("creator_user_id"),
      school_id: row.get("school_id"),
      name: row.get("name"),
      description: row.get("description"),
      active: row.get("active"),
    }
  }
}

pub async fn add(
  con: &mut impl GenericClient,
  creator_user_id: i64,
  school_id: i64,
  name: String,
  description: String,
  active: bool,
) -> Result<SchoolData, tokio_postgres::Error> {
  let creation_time = current_time_millis();

  let school_data_id = con
    .query_one(
      "INSERT INTO
       school_data_t(
           creation_time,
           creator_user_id,
           school_id,
           name,
           description,
           active
       )
       VALUES ($1, $2, $3, $4, $5, $6)
       RETURNING school_data_id
      ",
      &[
        &creation_time,
        &creator_user_id,
        &school_id,
        &name,
        &description,
        &active,
      ],
    )
    .await?
    .get(0);

  Ok(SchoolData {
    school_data_id,
    creation_time,
    creator_user_id,
    school_id,
    name,
    description,
    active,
  })
}

#[allow(unused)]
pub async fn get_by_school_data_id(
  con: &mut impl GenericClient,
  school_data_id: &i64,
) -> Result<Option<SchoolData>, tokio_postgres::Error> {
  let result = con
    .query_opt(
      "SELECT * FROM school_data_t WHERE school_data_id=$1",
      &[&school_data_id],
    )
    .await?
    .map(|x| x.into());

  Ok(result)
}

pub async fn get_by_school_id(
  con: &mut impl GenericClient,
  school_id: i64,
) -> Result<Option<SchoolData>, tokio_postgres::Error> {
  let result = con
    .query_opt(
      "
      SELECT sd.* FROM recent_school_data_v sd
      WHERE sd.school_id = $1
      ",
      &[&school_id],
    )
    .await?
    .map(|x| x.into());
  Ok(result)
}

pub async fn is_active_by_school_id(
  con: &mut impl GenericClient,
  school_id: i64,
) -> Result<bool, tokio_postgres::Error> {
  let result = matches!(
    get_by_school_id(con, school_id).await?,
    Some(SchoolData { active: true, .. })
  );

  Ok(result)
}

pub async fn query(
  con: &mut impl GenericClient,
  props: innexgo_hours_api::request::SchoolDataViewProps,
) -> Result<Vec<SchoolData>, tokio_postgres::Error> {
  let sql = [
    if props.only_recent {
      "SELECT sd.* FROM recent_school_data_v sd"
    } else {
      "SELECT sd.* FROM school_data_t sd"
    },
    " WHERE 1 = 1",
    " AND ($1::bigint[] IS NULL OR sd.school_data_id = ANY($1))",
    " AND ($2::bigint   IS NULL OR sd.creation_time >= $2)",
    " AND ($3::bigint   IS NULL OR sd.creation_time <= $3)",
    " AND ($4::bigint[] IS NULL OR sd.creator_user_id = ANY($4))",
    " AND ($5::bigint[] IS NULL OR sd.school_id = ANY($5))",
    " AND ($6::text[]   IS NULL OR sd.name = ANY($6))",
    " AND ($7::text     IS NULL OR sd.name LIKE CONCAT('%',$7,'%'))",
    " AND ($8::text[]   IS NULL OR sd.description = ANY($8))",
    " AND ($9::text     IS NULL OR sd.description LIKE CONCAT('%',$9,'%'))",
    " AND ($10::bool    IS NULL OR sd.active = $10)",
    " ORDER BY sd.school_data_id",
  ]
  .join("\n");

  let stmnt = con.prepare(&sql).await?;

  let results = con
    .query(
      &stmnt,
      &[
        &props.school_data_id,
        &props.min_creation_time,
        &props.max_creation_time,
        &props.creator_user_id,
        &props.school_id,
        &props.name,
        &props.partial_name,
        &props.description,
        &props.partial_description,
        &props.active,
      ],
    )
    .await?
    .into_iter()
    .map(|row| row.into())
    .collect();

  Ok(results)
}
