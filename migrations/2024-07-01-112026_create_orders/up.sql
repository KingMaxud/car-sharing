CREATE TABLE orders
(
    id              uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id         uuid NOT NULL REFERENCES users(id),
    car_id          uuid NOT NULL REFERENCES cars(id),
    start_rent_time TIMESTAMP,
    end_rent_time   TIMESTAMP,
    status          VARCHAR (30) NOT NULL DEFAULT 'awaits_confirmation',
    paid            bool NOT NULL DEFAULT false,
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP
);
