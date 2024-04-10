CREATE TABLE sessions
(
    session_token BYTEA PRIMARY KEY,
    user_id       uuid REFERENCES users (id) ON DELETE CASCADE
);
