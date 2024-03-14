// @generated automatically by Diesel CLI.

diesel::table! {
    cars (id) {
        id -> Uuid,
        #[max_length = 50]
        name -> Varchar,
        hourly_rate -> Int4,
        daily_rate -> Int4,
        weekly_rate -> Int4,
        photos -> Nullable<Array<Nullable<Text>>>,
        #[max_length = 30]
        status -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    orders (id) {
        id -> Uuid,
        user_id -> Uuid,
        car_id -> Uuid,
        start_rent_time -> Nullable<Timestamp>,
        end_rent_time -> Nullable<Timestamp>,
        #[max_length = 30]
        status -> Varchar,
        paid -> Bool,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 50]
        role -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(orders -> cars (car_id));
diesel::joinable!(orders -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    cars,
    orders,
    users,
);
