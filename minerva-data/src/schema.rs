table! {
    USER (id) {
        id -> Int4,
        login -> Varchar,
        name -> Varchar,
        email -> Nullable<Varchar>,
        pwhash -> Bytea,
    }
}

table! {
    address (id) {
        id -> Int4,
        client_id -> Int4,
        #[sql_name = "type"]
        type_ -> Int2,
        address -> Varchar,
        number -> Varchar,
        complement -> Nullable<Varchar>,
        district -> Varchar,
        state -> Bpchar,
        city -> Varchar,
        country -> Bpchar,
    }
}

table! {
    client (id) {
        id -> Int4,
        #[sql_name = "type"]
        type_ -> Int2,
        name -> Varchar,
        document -> Varchar,
        active -> Bool,
    }
}

table! {
    product (id) {
        id -> Int4,
        description -> Varchar,
        unit -> Bpchar,
        price -> Numeric,
    }
}

table! {
    stock (product_id) {
        product_id -> Int4,
        amount -> Numeric,
        cost -> Numeric,
    }
}

table! {
    stock_mov (id) {
        id -> Int4,
        product_id -> Int4,
        document -> Varchar,
        amount -> Numeric,
        cost -> Numeric,
        shipping_cost -> Numeric,
        date -> Timestamptz,
    }
}

table! {
    syslog (id) {
        id -> Int4,
        service -> Varchar,
        USER -> Varchar,
        operation -> Int2,
        datetime -> Timestamptz,
        description -> Nullable<Varchar>,
    }
}

joinable!(address -> client (client_id));

allow_tables_to_appear_in_same_query!(
    USER,
    address,
    client,
    product,
    stock,
    stock_mov,
    syslog,
);
