use pretty_assertions::assert_eq;
use serde_json::Value;
use vector_config_common::schema::RootSchema;

pub fn as_schema(value: Value) -> RootSchema {
    serde_json::from_value(value).expect("should not fail to deserialize schema")
}

#[track_caller]
pub fn assert_schemas_eq(expected: RootSchema, actual: RootSchema) {
    let expected_json = serde_json::to_string_pretty(&expected).expect("should not fail");
    let actual_json = serde_json::to_string_pretty(&actual).expect("should not fail");

    assert_eq!(expected_json, actual_json);
}
