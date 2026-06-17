use super::{infer_value_type, Error};
use googleapis_tonic_google_bigtable_v2::google::bigtable::v2::{r#type, value, Value};

fn make_val(kind: value::Kind) -> Value {
    Value {
        kind: Some(kind),
        r#type: None,
    }
}

#[test]
fn infer_type_bytes() {
    let t = infer_value_type("p", &make_val(value::Kind::BytesValue(vec![]))).unwrap();
    assert!(matches!(t.kind, Some(r#type::Kind::BytesType(_))));
}

#[test]
fn infer_type_string() {
    let t = infer_value_type("p", &make_val(value::Kind::StringValue(String::new()))).unwrap();
    assert!(matches!(t.kind, Some(r#type::Kind::StringType(_))));
}

#[test]
fn infer_type_int() {
    let t = infer_value_type("p", &make_val(value::Kind::IntValue(0))).unwrap();
    assert!(matches!(t.kind, Some(r#type::Kind::Int64Type(_))));
}

#[test]
fn infer_type_bool() {
    let t = infer_value_type("p", &make_val(value::Kind::BoolValue(false))).unwrap();
    assert!(matches!(t.kind, Some(r#type::Kind::BoolType(_))));
}

#[test]
fn infer_type_timestamp() {
    let t = infer_value_type(
        "p",
        &make_val(value::Kind::TimestampValue(Default::default())),
    )
    .unwrap();
    assert!(matches!(t.kind, Some(r#type::Kind::TimestampType(_))));
}

#[test]
fn infer_type_date() {
    let t =
        infer_value_type("p", &make_val(value::Kind::DateValue(Default::default()))).unwrap();
    assert!(matches!(t.kind, Some(r#type::Kind::DateType(_))));
}

#[test]
fn float_requires_explicit_type() {
    let err = infer_value_type("p", &make_val(value::Kind::FloatValue(0.0))).unwrap_err();
    // Assuming Error has an InvalidArgument variant. Check what your Error has!
    // We'll use a string match or general match based on what was there.
    assert!(err.to_string().contains("cannot infer float type"));
}

#[test]
fn array_requires_explicit_type() {
    let err = infer_value_type("p", &make_val(value::Kind::ArrayValue(Default::default())))
        .unwrap_err();
    assert!(err.to_string().contains("cannot infer array element type"));
}

#[test]
fn raw_value_rejected() {
    let err = infer_value_type("p", &make_val(value::Kind::RawValue(vec![]))).unwrap_err();
    assert!(err.to_string().contains("raw values cannot be used"));
}

#[test]
fn null_requires_explicit_type() {
    let v = Value {
        kind: None,
        r#type: None,
    };
    let err = infer_value_type("p", &v).unwrap_err();
    assert!(err.to_string().contains("NULL value requires an explicit"));
}
