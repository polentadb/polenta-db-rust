pub(crate) struct ColumnDefinition {
    column_type: String,
    size:        i32,
    precision:   i32,
    primary_key: bool,
}

pub(crate) struct CollectionDefinition  {
    collection_type: String,
    //Columns:        map[string]ColumnDefinition
    sequence:       i64,
}

pub(crate) fn create_user(user_name: &str) {
}