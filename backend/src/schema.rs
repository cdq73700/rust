// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Binary,
        email -> Varchar,
        name -> Varchar,
        password -> Varchar,
        created_at -> Datetime,
        created_by -> Varchar,
        updated_at -> Datetime,
        updated_by -> Varchar,
        deleted_at -> Nullable<Datetime>,
        deleted_by -> Nullable<Varchar>,
    }
}
