// Do not use this strategy in production.
// Code to initailize database

CREATE KEYSPACE IF NOT EXISTS user_auth WITH REPLICATION =
{ 'class' : 'SimpleStrategy', 'replication_factor' : 1 };

CREATE TABLE IF NOT EXISTS user_auth.users_list (
  user_id uuid,
  email text,
  password_hash text,
  PRIMARY KEY (user_id)
);

CREATE INDEX ON user_auth.users_list (email);
