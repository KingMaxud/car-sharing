CREATE TABLE users
(
    id          uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    role        VARCHAR (50) NOT NULL DEFAULT 'user',
    created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
