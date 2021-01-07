-- Root user, not meant to log in
INSERT INTO verification_challenge VALUES(
  'root_challenge',  -- verification_challenge_key_hash
  1, -- creation_time
  'root', -- name
  'root@example.com', -- email
  '$2a$10$kteCMggOjaT1lJWybiFwMewtFvec7QB35lo6Rjk7IjNJFVJBoyDQ.' -- password_hash
);

INSERT INTO user VALUES(
  0, -- user_id
  1, -- creation_time
  'root',  -- name
  'root@example.com',  -- email
  'root_challenge' -- verification_challenge_key_hash
);


-- The school everybody attends
INSERT INTO school VALUES(0, 1, 0, 'Squidward Community College', 'sqcc');

INSERT INTO user VALUES(
  1, -- user_id
  1, -- creation_time
  'BOB JOHNSON', --name
  'bob@example.com', -- email
  'root_challenge' -- verification_challenge_key_hash
);
INSERT INTO user VALUES(
  2, -- user_id
  1, -- creation_time
  'SARAH DOE', --name
  'sarah@example.com', -- email
  'root_challenge' -- verification_challenge_key_hash
);
INSERT INTO user VALUES(
  3, -- user_id
  1, -- creation_time
  'JOE SMITH', --name
  'joe@example.com', -- email
  'root_challenge' -- verification_challenge_key_hash
);
INSERT INTO user VALUES(
  4, -- user_id
  1, -- creation_time
  'ALICE BROWN', --name
  'alice@example.com', -- email
  'root_challenge' -- verification_challenge_key_hash
);
INSERT INTO user VALUES(
  5, -- user_id
  1, -- creation_time
  'BILLY FLETCHER', --name
  'billy@example.com', -- email
  'root_challenge' -- verification_challenge_key_hash
);
INSERT INTO user VALUES(
  6, -- user_id
  1, -- creation_time
  'CARSON WILSON', --name
  'carson@example.com', -- email
  'root_challenge' -- verification_challenge_key_hash
);
INSERT INTO user VALUES(
  7, -- user_id
  1, -- creation_time
  'GEORGE OHARE', --name
  'george@example.com', -- email
  'root_challenge' -- verification_challenge_key_hash
);
INSERT INTO user VALUES(
  8, -- user_id
  1, -- creation_time
  'WILLIAM DOE', --name
  'william@example.com', -- email
  'root_challenge' -- verification_challenge_key_hash
);
INSERT INTO user VALUES(
  9, -- user_id
  1, -- creation_time
  'ROBERT MCPHILLIP', --name
  'robert@example.com', -- email
  'root_challenge' -- verification_challenge_key_hash
);



-- Dummy location LMAO to prevent errors
INSERT INTO location VALUES(
  0, -- location_id
  1, -- creation_time
  0, -- creator_user_id
  0, -- school_id
  'Dummy Location',  -- name
  'Virtually, at Squidward Community College', -- description
  true
);

-- We want to create an alternate school system that we don't want to see leakage between

-- create alternate root
INSERT INTO verification_challenge VALUES(
  'other_root_challenge',  -- verification_challenge_key_hash
  1, -- creation_time
  'other_root', -- name
  'other_root@example.com', -- email
  '$2a$10$kteCMggOjaT1lJWybiFwMewtFvec7QB35lo6Rjk7IjNJFVJBoyDQ.' -- password_hash
);

INSERT INTO user VALUES(
  42, -- user_id
  1, -- creation_time
  'other_root',  -- name
  'other_root@example.com',  -- email
  'other_root_challenge' -- verification_challenge_key_hash
);

-- A different school that we don't want to accidentally see showing up on any query
INSERT INTO school VALUES(
  1, -- school_id
  1, -- creation_time
  42, -- creator_user_id
  'Other School', -- name
  'os' -- abbreviation
);

-- The admin of this other school is other_root
INSERT INTO adminship VALUES(
  42, -- adminship_id
  1, -- creation_time
  42, -- creator_user_id
  42, -- user_id
  1, -- school_id
  0 -- adminship_kind
);

INSERT INTO verification_challenge VALUES(
  'OTHERBOB JOHNSON CHALLENGE',  -- verification_challenge_key_hash
  1, -- creation_time
  'OTHERBOB JOHNSON', -- name
  'otherbob@example.com', -- email
  '$2a$10$kteCMggOjaT1lJWybiFwMewtFvec7QB35lo6Rjk7IjNJFVJBoyDQ.' -- password_hash
);


INSERT INTO user VALUES(
  43, -- user_id 
  1, -- creation_time
  'OTHERBOB JOHNSON', -- name
  'otherbob@example.com', -- email
  'OTHERBOB JOHNSON CHALLENGE' -- verification_challenge_key_hash
);

INSERT INTO password VALUES(
  43, -- password_id
  1, -- creation_time
  43, -- creator_user_id
  43, -- user_id
  0, -- password_kind
 '$2a$10$kteCMggOjaT1lJWybiFwMewtFvec7QB35lo6Rjk7IjNJFVJBoyDQ.', -- password_hash
  '' -- password_reset_key_hash
);

-- Create a course
INSERT INTO course VALUES(
  42, -- course_id
  1, -- creation_time
  43, -- creator_user_id
  1, -- school_id
  'OTHERMATH', -- name
  'A math class from OTHER SCHOOL' -- description
);

-- Otherbob johnson is an instructor
INSERT INTO course_membership VALUES(
  42, -- course_membership_id
  1, -- creation_time
  43, -- creator_user_id
  43, -- user_id
  42, -- course_id
  1 -- course_membership_kind
);

-- A student in this different school we don't want to show up at all

INSERT INTO verification_challenge VALUES(
  'OTHERBILLY FLETCHER CHALLENGE',  -- verification_challenge_key_hash
  1, -- creation_time
  'OTHERBILLY FLETCHER', -- name
  'otherbilly@example.com', -- email
  '$2a$10$kteCMggOjaT1lJWybiFwMewtFvec7QB35lo6Rjk7IjNJFVJBoyDQ.' -- password_hash
);

INSERT INTO user VALUES(
  44, -- user_id 
  1, -- creation_time
  'OTHERBILLY FLETCHER', -- name
  'otherbilly@example.com', -- email
  'OTHERBILLY FLETCHER CHALLENGE' -- verification_challenge_key_hash
);

INSERT INTO password VALUES(
  44, -- password_id
  1, -- creation_time
  44, -- creator_user_id
  44, -- user_id
  0, -- password_kind
 '$2a$10$kteCMggOjaT1lJWybiFwMewtFvec7QB35lo6Rjk7IjNJFVJBoyDQ.', -- password_hash
  '' -- password_reset_key_hash
);

-- Otherbilly fletcher is a student
INSERT INTO course_membership VALUES(
  44, -- course_membership_id
  1, -- creation_time
  44, -- creator_user_id
  44, -- user_id
  42, -- course_id
  0 -- course_membership_kind
);
