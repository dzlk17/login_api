// Do not use this strategy in production.
CREATE KEYSPACE IF NOT EXISTS user_auth WITH REPLICATION =
{ 'class' : 'SimpleStrategy', 'replication_factor' : 2 };

CREATE TABLE IF NOT EXISTS user_auth.users_list (
  user_id uuid,
  email text,
  password_hash text,
  PRIMARY KEY (user_id)
);

INSERT INTO user_auth.users_list (user_id, email, password_hash)
    VALUES (uuid(), 'jannowak@gmail.com', '1234');