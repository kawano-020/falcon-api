table! {
    user (id) {
        id -> Unsigned<Bigint>,
        first_name -> Varchar,
        last_name -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}
