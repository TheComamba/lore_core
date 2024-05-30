// @generated automatically by Diesel CLI.

diesel::table! {
    entities (label, descriptor) {
        label -> Text,
        descriptor -> Text,
        description -> Text,
    }
}

diesel::table! {
    history_items (timestamp) {
        timestamp -> BigInt,
        year -> Integer,
        day -> Integer,
        content -> Text,
        properties -> Text,
    }
}

diesel::table! {
    relationships (parent, child, role) {
        parent -> Text,
        child -> Text,
        role -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(entities, history_items, relationships,);
