use super::db_types::*;
use super::utils::current_time_millis;
use innexgo_hours_api::request;
use std::convert::TryInto;
use tokio_postgres::GenericClient;

impl From<tokio_postgres::row::Row> for CourseMembership {
  // select * from course_membership_t order only, otherwise it will fail
  fn from(row: tokio_postgres::Row) -> CourseMembership {
    CourseMembership {
      course_membership_id: row.get("course_membership_id"),
      creation_time: row.get("creation_time"),
      creator_user_id: row.get("creator_user_id"),
      user_id: row.get("user_id"),
      course_id: row.get("course_id"),
      course_membership_kind: (row.get::<_, i64>("course_membership_kind") as u8)
        .try_into()
        .unwrap(),
      course_key_key: row.get("course_key_key"),
    }
  }
}

pub async fn add(
  con: &mut impl GenericClient,
  creator_user_id: i64,
  user_id: i64,
  course_id: i64,
  course_membership_kind: request::CourseMembershipKind,
  course_key_key: Option<String>,
) -> Result<CourseMembership, tokio_postgres::Error> {
  let creation_time = current_time_millis();

  let course_membership_id = con
    .query_one(
      "INSERT INTO
       course_membership_t(
           creation_time,
           creator_user_id,
           user_id,
           course_id,
           course_membership_kind,
           course_key_key
       )
       VALUES ($1, $2, $3, $4, $5, $6)
       RETURNING course_membership_id
      ",
      &[
        &creation_time,
        &creator_user_id,
        &user_id,
        &course_id,
        &(course_membership_kind.clone() as i64),
        &course_key_key,
      ],
    )
    .await?
    .get(0);

  // return course_membership
  Ok(CourseMembership {
    course_membership_id,
    creation_time,
    creator_user_id,
    user_id,
    course_id,
    course_membership_kind,
    course_key_key,
  })
}

pub async fn get_by_course_membership_id(
  con: &mut impl GenericClient,
  course_membership_id: i64,
) -> Result<Option<CourseMembership>, tokio_postgres::Error> {
  let result = con
    .query_opt(
      "SELECT * FROM course_membership_t WHERE course_membership_id=$1",
      &[&course_membership_id],
    )
    .await?
    .map(|x| x.into());

  Ok(result)
}

pub async fn get_by_course_key_key(
  con: &mut impl GenericClient,
  course_key_key: &str,
) -> Result<Vec<CourseMembership>, tokio_postgres::Error> {
  let result = con
    .query(
      "SELECT * FROM course_membership_t WHERE course_key_key=$1",
      &[&course_key_key],
    )
    .await?
    .into_iter()
    .map(|x| x.into())
    .collect();

  Ok(result)
}

pub async fn count_course_key_uses(
  con: &mut impl GenericClient,
  course_key_key: &str,
) -> Result<i64, tokio_postgres::Error> {
  Ok(get_by_course_key_key(con, course_key_key).await?.len() as i64)
}

pub async fn get_by_user_id_course_id(
  con: &mut impl GenericClient,
  user_id: i64,
  course_id: i64,
) -> Result<Option<CourseMembership>, tokio_postgres::Error> {
  let result = con
    .query_opt(
      "
      SELECT cm.* FROM recent_course_membership_v cm
      WHERE 1 = 1
      AND cm.user_id = $1
      AND cm.course_id = $2
      ",
      &[&user_id, &course_id],
    )
    .await?
    .map(|x| x.into());

  Ok(result)
}

pub async fn is_student(
  con: &mut impl GenericClient,
  user_id: i64,
  course_id: i64,
) -> Result<bool, tokio_postgres::Error> {
  let result = matches!(
    get_by_user_id_course_id(con, user_id, course_id).await?,
    Some(CourseMembership {
      course_membership_kind: request::CourseMembershipKind::Student,
      ..
    })
  );

  Ok(result)
}

pub async fn is_instructor(
  con: &mut impl GenericClient,
  user_id: i64,
  course_id: i64,
) -> Result<bool, tokio_postgres::Error> {
  let result = matches!(
    get_by_user_id_course_id(con, user_id, course_id).await?,
    Some(CourseMembership {
      course_membership_kind: request::CourseMembershipKind::Instructor,
      ..
    })
  );
  Ok(result)
}

// if the user id is an instructor at the given location
pub async fn is_instructor_at(
  con: &mut impl GenericClient,
  user_id: i64,
  location_id: i64,
) -> Result<bool, tokio_postgres::Error> {
  let result: i64 = con
    .query_one(
      "
      SELECT count(cm.*)
      FROM recent_course_membership_v cm
      JOIN recent_course_data_v cd ON cd.course_id = cm.course_id
      WHERE 1 = 1
      AND cm.user_id = $1
      AND cd.location_id = $2
      AND cm.course_membership_kind = $3
      AND cd.active
      ",
      &[
        &user_id,
        &location_id,
        &(request::CourseMembershipKind::Instructor as i64),
      ],
    )
    .await?
    .get(0);

  Ok(result > 0)
}

pub async fn is_member(
  con: &mut impl GenericClient,
  user_id: i64,
  course_id: i64,
) -> Result<bool, tokio_postgres::Error> {
  let result = matches!(
    get_by_user_id_course_id(con, user_id, course_id).await?,
    Some(CourseMembership {
      course_membership_kind: request::CourseMembershipKind::Instructor
        | request::CourseMembershipKind::Student,
      ..
    })
  );

  Ok(result)
}

pub async fn get_by_course_id(
  con: &mut impl GenericClient,
  course_id: i64,
) -> Result<Vec<CourseMembership>, tokio_postgres::Error> {
  let result = con
    .query(
      "
      SELECT cm.* FROM recent_course_membership_v cm
      WHERE 1 = 1
      AND cm.course_id = $1
      ",
      &[&course_id],
    )
    .await?
    .into_iter()
    .map(|x| x.into())
    .collect();

  Ok(result)
}

pub async fn count_instructors(
  con: &mut impl GenericClient,
  course_id: i64,
) -> Result<i64, tokio_postgres::Error> {
  let result = get_by_course_id(con, course_id)
    .await?
    .into_iter()
    .filter(|x| {
      matches!(
        x,
        CourseMembership {
          course_membership_kind: request::CourseMembershipKind::Instructor,
          ..
        },
      )
    })
    .count();

  Ok(result as i64)
}

pub async fn query(
  con: &mut impl GenericClient,
  props: innexgo_hours_api::request::CourseMembershipViewProps,
) -> Result<Vec<CourseMembership>, tokio_postgres::Error> {
  let sql = [
    if props.only_recent {
      "SELECT cm.* FROM recent_course_membership_v cm"
    } else {
      "SELECT cm.* FROM course_membership_t cm"
    },
    " WHERE 1 = 1",
    " AND ($1::bigint[] IS NULL OR cm.course_membership_id = ANY($1))",
    " AND ($2::bigint   IS NULL OR cm.creation_time >= $2)",
    " AND ($3::bigint   IS NULL OR cm.creation_time <= $3)",
    " AND ($4::bigint[] IS NULL OR cm.creator_user_id = ANY($4))",
    " AND ($5::bigint[] IS NULL OR cm.user_id = ANY($5))",
    " AND ($6::bigint[] IS NULL OR cm.course_id = ANY($6))",
    " AND ($7::bigint[] IS NULL OR cm.course_membership_kind = ANY($7))",
    " AND ($8::bool     IS NULL OR cm.course_key_key IS NOT NULL = $8)",
    " AND ($9::text[]   IS NULL OR cm.course_key_key = ANY($9))",
    " ORDER BY cm.course_membership_id",
  ]
  .join("\n");

  let stmnt = con.prepare(&sql).await?;

  let results = con
    .query(
      &stmnt,
      &[
        &props.course_membership_id,
        &props.min_creation_time,
        &props.max_creation_time,
        &props.creator_user_id,
        &props.user_id,
        &props.course_id,
        &props
          .course_membership_kind
          .map(|v| v.into_iter().map(|x| x as i64).collect::<Vec<i64>>()),
        &props.course_membership_from_key,
        &props.course_key_key,
      ],
    )
    .await?
    .into_iter()
    .map(|row| row.into())
    .collect();

  Ok(results)
}
