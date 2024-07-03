CREATE TABLE sessions
(
    session_token BYTEA PRIMARY KEY,
    user_id       uuid NOT NULL REFERENCES users (id) ON DELETE CASCADE
);
