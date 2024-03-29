
use super::db_types::*;
use super::utils::current_time_millis;
use innexgo_hours_api::request;
use tokio_postgres::GenericClient;

impl From<tokio_postgres::row::Row> for Session {
  // select * from session order only, otherwise it will fail
  fn from(row: tokio_postgres::row::Row) -> Session {
    Session {
      session_id: row.get("session_id"),
      creation_time: row.get("creation_time"),
      creator_user_id: row.get("creator_user_id"),
      course_id: row.get("course_id"),
    }
  }
}

pub async fn add(
  con: &mut impl GenericClient,
  creator_user_id: i64,
  course_id: i64,
) -> Result<Session, tokio_postgres::Error> {
  let creation_time = current_time_millis();

  let session_id = con
    .query_one(
      "INSERT INTO
       session_t(
           creation_time,
           creator_user_id,
           course_id
       )
       VALUES($1, $2, $3)
       RETURNING session_id
      ",
      &[&creation_time, &creator_user_id, &course_id],
    )
    .await?
    .get(0);

  // return session
  Ok(Session {
    session_id,
    creation_time,
    creator_user_id,
    course_id,
  })
}

pub async fn get_by_session_id(
  con: &mut impl GenericClient,
  session_id: i64,
) -> Result<Option<Session>, tokio_postgres::Error> {
  let result = con
    .query_opt("SELECT * FROM session_t WHERE session_id=$1", &[&session_id])
    .await?
    .map(|x| x.into());

  Ok(result)
}

pub async fn query(
  con: &mut impl GenericClient,
  props: request::SessionViewProps,
) -> Result<Vec<Session>, tokio_postgres::Error> {
  let results = con
    .query(
      "
      SELECT ses.* FROM session_t ses WHERE 1 = 1
      AND ($1::bigint[] IS NULL OR ses.session_id = ANY $1)
      AND ($2::bigint   IS NULL OR ses.creation_time >= $2)
      AND ($3::bigint   IS NULL OR ses.creation_time <= $3)
      AND ($4::bigint[] IS NULL OR ses.creator_user_id = ANY($4))
      AND ($5::bigint[] IS NULL OR ses.course_id = ANY($5))
      ORDER BY ses.session_id
      ",
      &[
        &props.session_id,
        &props.min_creation_time,
        &props.max_creation_time,
        &props.creator_user_id,
        &props.course_id,
      ],
    )
    .await?
    .into_iter()
    .map(|row| row.into())
    .collect();

  Ok(results)
}
