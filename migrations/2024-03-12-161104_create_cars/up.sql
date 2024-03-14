CREATE TABLE cars
(
    id              uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    name            VARCHAR (50) NOT NULL,
    hourly_rate     INTEGER NOT NULL,
    daily_rate      INTEGER NOT NULL,
    weekly_rate     INTEGER NOT NULL,
    photos          TEXT[]
);
