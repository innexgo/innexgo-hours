use super::db_types::*;
use super::utils::current_time_millis;
use innexgo_hours_api::request;
use tokio_postgres::GenericClient;

impl From<tokio_postgres::row::Row> for SchoolKey {
  // select * from school_key order only, otherwise it will fail
  fn from(row: tokio_postgres::row::Row) -> SchoolKey {
    SchoolKey {
      school_key_key: row.get("school_key_key"),
      creation_time: row.get("creation_time"),
      creator_user_id: row.get("creator_user_id"),
      school_id: row.get("school_id"),
      start_time: row.get("start_time"),
      end_time: row.get("end_time"),
    }
  }
}

pub async fn add(
  con: &mut impl GenericClient,
  school_key_key_str: &str,
  creator_user_id: i64,
  school_id: i64,
  start_time: i64,
  end_time: i64,
) -> Result<SchoolKey, tokio_postgres::Error> {
  let creation_time = current_time_millis();

  let school_key_key = con
    .query_one(
      "INSERT INTO
       school_key_t(
           school_key_key,
           creation_time,
           creator_user_id,
           school_id,
           start_time,
           end_time
       )
       VALUES($1, $2, $3, $4, $5, $6)
       RETURNING school_key_key
      ",
      &[
        &school_key_key_str,
        &creation_time,
        &creator_user_id,
        &school_id,
        &start_time,
        &end_time,
      ],
    )
    .await?
    .get(0);

  // return school_key
  Ok(SchoolKey {
    school_key_key,
    creation_time,
    creator_user_id,
    school_id,
    start_time,
    end_time
  })
}

pub async fn get_by_school_key_key(
  con: &mut impl GenericClient,
  school_key_key: &str,
) -> Result<Option<SchoolKey>, tokio_postgres::Error> {
  let result = con
    .query_opt(
      "SELECT * FROM school_key_t WHERE school_key_key=$1",
      &[&school_key_key],
    )
    .await?
    .map(|x| x.into());

  Ok(result)
}

pub async fn query(
  con: &mut impl GenericClient,
  props: request::SchoolKeyViewProps,
) -> Result<Vec<SchoolKey>, tokio_postgres::Error> {

  let sql = "
    SELECT sk.* FROM school_key_t sk
    WHERE 1 = 1
    AND ($1::text[]   IS NULL OR sk.school_key_key = ANY($1))
    AND ($2::bigint   IS NULL OR sk.creation_time >= $2)
    AND ($3::bigint   IS NULL OR sk.creation_time <= $3)
    AND ($4::bigint[] IS NULL OR sk.creator_user_id = ANY($4))
    AND ($5::bigint[] IS NULL OR sk.school_id = ANY($5))
    AND ($6::bigint   IS NULL OR sk.start_time >= $6)
    AND ($7::bigint   IS NULL OR sk.start_time <= $7)
    AND ($8::bigint   IS NULL OR sk.end_time >= $8)
    AND ($9::bigint  IS NULL OR sk.end_time <= $9)
    ORDER BY sk.school_key_key
  ";

  let stmnt = con.prepare(sql).await?;

  let results = con
    .query(
      &stmnt,
      &[
        &props.school_key_key,
        &props.min_creation_time,
        &props.max_creation_time,
        &props.creator_user_id,
        &props.school_id,
        &props.min_start_time,
        &props.max_start_time,
        &props.min_end_time,
        &props.max_end_time,
      ],
    )
    .await?
    .into_iter()
    .map(|x| x.into())
    .collect();

  Ok(results)
}
