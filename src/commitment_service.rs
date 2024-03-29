use super::db_types::*;
use super::utils::current_time_millis;
use innexgo_hours_api::request;
use tokio_postgres::GenericClient;

impl From<tokio_postgres::row::Row> for Commitment {
  // select * from commitment order only, otherwise it will fail
  fn from(row: tokio_postgres::Row) -> Commitment {
    Commitment {
      commitment_id: row.get("commitment_id"),
      creation_time: row.get("creation_time"),
      creator_user_id: row.get("creator_user_id"),
      attendee_user_id: row.get("attendee_user_id"),
      session_id: row.get("session_id"),
      active: row.get("active"),
    }
  }
}

pub async fn add(
  con: &mut impl GenericClient,
  creator_user_id: i64,
  attendee_user_id: i64,
  session_id: i64,
  active: bool,
) -> Result<Commitment, tokio_postgres::Error> {
  let creation_time = current_time_millis();

  let commitment_id = con
    .query_one(
      "INSERT INTO
       commitment_t(
           creation_time,
           creator_user_id,
           attendee_user_id,
           session_id,
           active
       )
       VALUES($1, $2, $3, $4, $5)
       RETURNING commitment_id
      ",
      &[
        &creation_time,
        &creator_user_id,
        &attendee_user_id,
        &session_id,
        &active,
      ],
    )
    .await?
    .get(0);

  // return commitment
  Ok(Commitment {
    commitment_id,
    creation_time,
    creator_user_id,
    attendee_user_id,
    session_id,
    active,
  })
}

pub async fn get_by_commitment_id(
  con: &mut impl GenericClient,
  commitment_id: i64,
) -> Result<Option<Commitment>, tokio_postgres::Error> {
  let result = con
    .query_opt(
      "SELECT * FROM commitment_t WHERE commitment_id=$1",
      &[&commitment_id],
    )
    .await?
    .map(|x| x.into());

  Ok(result)
}

pub async fn get_by_attendee_user_id_session_id(
  con: &mut impl GenericClient,
  attendee_user_id: i64,
  session_id: i64,
) -> Result<Option<Commitment>, tokio_postgres::Error> {
  let result = con
    .query_opt(
      "SELECT * FROM recent_commitment_v WHERE attendee_user_id=$1 AND session_id=$2",
      &[&attendee_user_id, &session_id],
    )
    .await?
    .map(|x| x.into());

  Ok(result)
}

pub async fn query(
  con: &mut impl GenericClient,
  props: request::CommitmentViewProps,
) -> Result<Vec<Commitment>, tokio_postgres::Error> {
  let sql = [
    if props.only_recent {
      "SELECT c.* FROM recent_commitment_v c"
    } else {
      "SELECT c.* FROM commitment_t c"
    },
    "INNER JOIN session_t ses ON ses.session_id = c.session_id",
    "INNER JOIN recent_session_data_v sesd ON sesd.session_id = c.session_id",
    "LEFT JOIN session_request_response_t srr ON srr.commitment_id = c.commitment_id",
    "WHERE 1 = 1",
    "AND ($1::bigint[] IS NULL OR c.commitment_id = ANY($1))",
    "AND ($2::bigint   IS NULL OR c.creation_time >= $2)",
    "AND ($3::bigint   IS NULL OR c.creation_time <= $3)",
    "AND ($4::bigint[] IS NULL OR c.creator_user_id = ANY($4))",
    "AND ($5::bigint[] IS NULL OR c.attendee_user_id = ANY($5))",
    "AND ($6::bigint[] IS NULL OR c.session_id = ANY($6))",
    "AND ($7::bigint[] IS NULL OR ses.course_id = ANY($7))",
    "AND ($8::bigint   IS NULL OR sesd.start_time >= $8)",
    "AND ($9::bigint   IS NULL OR sesd.start_time <= $9)",
    "AND ($10::bigint  IS NULL OR sesd.end_time >= $10)",
    "AND ($11::bigint  IS NULL OR sesd.end_time <= $11)",
    "AND ($12::bool    IS NULL OR c.active IS NOT NULL = $12)",
    "AND ($13::bool    IS NULL OR srr.commitment_id IS NOT NULL = $13)",
    "ORDER BY c.commitment_id",
  ]
  .join("\n");

  let stmnt = con.prepare(&sql).await?;

  let results = con
    .query(
      &stmnt,
      &[
        &props.commitment_id,
        &props.min_creation_time,
        &props.max_creation_time,
        &props.creator_user_id,
        &props.attendee_user_id,
        &props.session_id,
        &props.course_id,
        &props.min_start_time,
        &props.max_start_time,
        &props.min_end_time,
        &props.max_end_time,
        &props.active,
        &props.from_request_response,
      ],
    )
    .await?
    .into_iter()
    .map(|x| x.into())
    .collect();

  Ok(results)
}
