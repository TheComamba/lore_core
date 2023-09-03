// @generated automatically by Diesel CLI.

diesel::table! {
    entities (label, descriptor) {
        label -> Text,
        descriptor -> Text,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    history_items (label) {
        label -> Text,
        year -> Integer,
        day -> Nullable<Integer>,
        content -> Text,
        properties -> Nullable<Text>,
    }
}

diesel::table! {
    relationships (parent, child, role) {
        parent -> Text,
        child -> Text,
        role -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(entities, history_items, relationships,);
