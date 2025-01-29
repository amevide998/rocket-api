// @generated automatically by Diesel CLI.

diesel::table! {
    rustaceans (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        create_at -> Timestamp,
    }
}
