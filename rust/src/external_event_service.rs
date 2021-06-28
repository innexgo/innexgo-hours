use super::db_types::*;
use super::utils::current_time_millis;
use tokio_postgres::GenericClient;
use innexgo_hours_api::request;

impl From<tokio_postgres::row::Row> for ExternalEvent {
  // select * from external_event order only, otherwise it will fail
  fn from(row: tokio_postgres::row::Row) -> ExternalEvent {
    ExternalEvent {
      external_event_id: row.get("external_event_id"),
      creation_time: row.get("creation_time"),
      creator_user_id: row.get("creator_user_id"),
    }
  }
}

pub async fn add(
  con: &mut impl GenericClient,
  creator_user_id: i64,
) -> Result<ExternalEvent, tokio_postgres::Error> {
  let creation_time = current_time_millis();

  let external_event_id = con
    .query_one(
      "INSERT INTO
       external_event(
           creation_time,
           creator_user_id
       )
       VALUES($1, $2)
       RETURNING external_event_id
      ",
      &[&creation_time, &creator_user_id],
    ).await?
    .get(0);

  // return external_event
  Ok(ExternalEvent {
    external_event_id,
    creation_time,
    creator_user_id,
  })
}

pub async fn get_by_external_event_id(
  con: &mut impl GenericClient,
  external_event_id: i64,
) -> Result<Option<ExternalEvent>, tokio_postgres::Error> {
  let result = con
    .query_opt(
      "SELECT * FROM external_event WHERE external_event_id=$1",
      &[&external_event_id],
    ).await?
    .map(|x| x.into());

  Ok(result)
}

pub async fn query(
  con: &mut impl GenericClient,
  props: request::ExternalEventViewProps,
) -> Result<Vec<ExternalEvent>, tokio_postgres::Error> {
  let results = con
    .query(
      " SELECT ee.* FROM external_event ee WHERE 1 = 1
        AND ($1::bigint IS NULL OR ee.external_event_id = $1)
        AND ($2::bigint IS NULL OR ee.creation_time >= $2)
        AND ($3::bigint IS NULL OR ee.creation_time <= $3)
        AND ($4::bigint IS NULL OR ee.creator_user_id = $4)
        ORDER BY ee.external_event_id
        LIMIT $5
        OFFSET $6
      ",
      &[
        &props.external_event_id,
        &props.min_creation_time,
        &props.max_creation_time,
        &props.creator_user_id,
        &props.count.unwrap_or(100),
        &props.offset.unwrap_or(0),
      ],
    ).await?
    .into_iter()
    .map(|row| row.into())
    .collect();

  Ok(results)
}
