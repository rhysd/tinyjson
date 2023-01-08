use std::collections::HashMap;
use tinyjson::*;

#[test]
fn test_query_create() {
    let mut v = JsonValue::Number(0.0);
    let q = JsonQuery::new(&v);
    assert_eq!(q.find(), Some(&v));
    let q = JsonQueryMut::new(&mut v);
    assert_eq!(q.find(), Some(&mut JsonValue::Number(0.0)));
    let q = JsonQuery::default();
    assert_eq!(q.find(), None);
    let q = JsonQueryMut::default();
    assert_eq!(q.find(), None);
}

#[test]
fn test_query_array_index() {
    let v: JsonValue = "[[[0], []], []]".parse().unwrap();

    assert!(v.query().child(0).find().unwrap().is_array());
    assert!(v.query().child(0).child(0).find().unwrap().is_array());
    assert_eq!(
        v.query().child(0).child(0).child(0).find().unwrap(),
        &JsonValue::Number(0.0),
    );
    assert_eq!(
        v.query().child(0).child(1).find().unwrap(),
        &JsonValue::Array(vec![]),
    );
    assert_eq!(
        v.query().child(1).find().unwrap(),
        &JsonValue::Array(vec![]),
    );

    assert_eq!(v.query().child(0).child(0).child(0).child(0).find(), None);
    assert_eq!(
        v.query()
            .child(0)
            .child(0)
            .child(0)
            .child(0)
            .child(0)
            .find(),
        None,
    );
    assert_eq!(v.query().child(10000).find(), None);
    assert_eq!(v.query().child(10000).child(0).find(), None);
    assert_eq!(v.query().child(0).child(1).child(0).find(), None);

    let q = v.query().child(0);
    let q2 = q.clone();
    assert_eq!(q.find(), q2.find());
}

#[test]
fn test_query_mut_array_index() {
    let mut v: JsonValue = "[[[0], []], []]".parse().unwrap();

    assert!(v.query_mut().child(0).find().unwrap().is_array());
    assert!(v.query_mut().child(0).child(0).find().unwrap().is_array());
    assert_eq!(
        v.query_mut().child(0).child(0).child(0).find().unwrap(),
        &mut JsonValue::Number(0.0),
    );
    assert_eq!(
        v.query_mut().child(0).child(1).find().unwrap(),
        &mut JsonValue::Array(vec![]),
    );
    assert_eq!(
        v.query_mut().child(1).find().unwrap(),
        &mut JsonValue::Array(vec![]),
    );

    assert_eq!(
        v.query_mut().child(0).child(0).child(0).child(0).find(),
        None
    );
    assert_eq!(
        v.query_mut()
            .child(0)
            .child(0)
            .child(0)
            .child(0)
            .child(0)
            .find(),
        None,
    );
    assert_eq!(v.query_mut().child(10000).find(), None);
    assert_eq!(v.query_mut().child(10000).child(0).find(), None);
    assert_eq!(v.query_mut().child(0).child(1).child(0).find(), None);

    *v.query_mut()
        .child(0)
        .child(0)
        .child(0)
        .get::<f64>()
        .unwrap() = 99.0;
    assert_eq!(v.stringify().unwrap(), "[[[99],[]],[]]");
}

#[test]
fn test_query_object_key() {
    let v: JsonValue = r#"{"a":{"b":{"c":0},"d":{}},"e":{}}"#.parse().unwrap();

    assert!(v.query().child("a").find().unwrap().is_object());
    assert!(v.query().child("a").child("b").find().unwrap().is_object());
    assert_eq!(
        v.query().child("a").child("b").child("c").find().unwrap(),
        &JsonValue::Number(0.0),
    );
    assert_eq!(
        v.query().child("a").child("d").find().unwrap(),
        &JsonValue::Object(HashMap::new()),
    );
    assert_eq!(
        v.query().child("e").find().unwrap(),
        &JsonValue::Object(HashMap::new()),
    );

    assert_eq!(
        v.query().child("a").child("b").child("c").child("d").find(),
        None,
    );
    assert_eq!(v.query().child("a").child("b").child("x").find(), None);
    assert_eq!(v.query().child("a").child("x").find(), None);
    assert_eq!(v.query().child("x").find(), None);
    assert_eq!(v.query().child("a").child("d").child("x").find(), None);

    let q = v.query().child("a").child("b");
    let q2 = q.clone();
    assert_eq!(q.find(), q2.find());
}

#[test]
fn test_query_mut_object_key() {
    let mut v: JsonValue = r#"{"a":{"b":{"c":0},"d":{}},"e":{}}"#.parse().unwrap();

    assert!(v.query_mut().child("a").find().unwrap().is_object());
    assert!(v
        .query_mut()
        .child("a")
        .child("b")
        .find()
        .unwrap()
        .is_object());
    assert_eq!(
        v.query_mut()
            .child("a")
            .child("b")
            .child("c")
            .find()
            .unwrap(),
        &mut JsonValue::Number(0.0),
    );
    assert_eq!(
        v.query_mut().child("a").child("d").find().unwrap(),
        &mut JsonValue::Object(HashMap::new()),
    );
    assert_eq!(
        v.query_mut().child("e").find().unwrap(),
        &mut JsonValue::Object(HashMap::new()),
    );

    assert_eq!(
        v.query_mut()
            .child("a")
            .child("b")
            .child("c")
            .child("d")
            .find(),
        None,
    );
    assert_eq!(v.query_mut().child("a").child("b").child("x").find(), None);
    assert_eq!(v.query_mut().child("a").child("x").find(), None);
    assert_eq!(v.query_mut().child("x").find(), None);
    assert_eq!(v.query_mut().child("a").child("d").child("x").find(), None);

    *v.query_mut()
        .child("a")
        .child("b")
        .child("c")
        .get::<f64>()
        .unwrap() = 99.0;
    assert_eq!(
        v.query()
            .child("a")
            .child("b")
            .child("c")
            .get::<f64>()
            .unwrap(),
        &99.0
    );
}

#[test]
fn test_query_value_predicate() {
    let v: JsonValue = r#"[{"a": 0, "b": 1}, 0, 1, 2]"#.parse().unwrap();
    let a: &Vec<_> = v.get().unwrap();
    let m: &HashMap<_, _> = a[0].get().unwrap();

    assert_eq!(v.query().child_by(|v| v.is_object()).get(), Some(m));
    assert_eq!(v.query().child_by(|v| v.is_number()).find(), Some(&a[1]));
    assert_eq!(
        v.query()
            .child_by(|v| matches!(v, JsonValue::Number(n) if *n > 0.0))
            .find(),
        Some(&a[2]),
    );
    assert_eq!(
        v.query()
            .child_by(|v| v.is_object())
            .child_by(|v| matches!(v, JsonValue::Number(n) if *n > 0.0))
            .find(),
        Some(&m["b"]),
    );
    assert_eq!(v.query().child_by(|v| v.is_string()).find(), None);
    assert_eq!(
        v.query()
            .child_by(|v| matches!(v, JsonValue::Number(n) if *n > 10.0))
            .find(),
        None,
    );
    assert_eq!(
        v.query()
            .child_by(|v| v.is_object())
            .child_by(|v| matches!(v, JsonValue::Number(n) if *n > 10.0))
            .find(),
        None,
    );

    let q = v.query().child_by(|v| v.is_object());
    let q2 = q.clone();
    assert_eq!(q.find(), q2.find());
}

#[test]
fn test_query_mut_value_predicate() {
    let mut v: JsonValue = r#"[{"a": 0, "b": 1}, 0, 1, 2]"#.parse().unwrap();

    assert!(v
        .query_mut()
        .child_by(|v| v.is_object())
        .find()
        .unwrap()
        .is_object());
    assert_eq!(
        v.query_mut().child_by(|v| v.is_number()).find(),
        Some(&mut JsonValue::Number(0.0))
    );
    assert_eq!(
        v.query_mut()
            .child_by(|v| matches!(v, JsonValue::Number(n) if *n > 0.0))
            .find(),
        Some(&mut JsonValue::Number(1.0)),
    );
    assert_eq!(
        v.query_mut()
            .child_by(|v| v.is_object())
            .child_by(|v| matches!(v, JsonValue::Number(n) if *n > 0.0))
            .find(),
        Some(&mut JsonValue::Number(1.0)),
    );
    assert_eq!(v.query_mut().child_by(|v| v.is_string()).find(), None);
    assert_eq!(
        v.query_mut()
            .child_by(|v| matches!(v, JsonValue::Number(n) if *n > 10.0))
            .find(),
        None,
    );
    assert_eq!(
        v.query_mut()
            .child_by(|v| v.is_object())
            .child_by(|v| matches!(v, JsonValue::Number(n) if *n > 10.0))
            .find(),
        None,
    );

    *v.query_mut().child_by(|v| v.is_object()).find().unwrap() =
        JsonValue::String("hello".to_string());
    *v.query_mut()
        .child_by(|v| v.is_number())
        .get::<f64>()
        .unwrap() = -11.0;
    *v.query_mut()
        .child_by(|v| matches!(v, JsonValue::Number(n) if *n > 1.0))
        .get::<f64>()
        .unwrap() = 22.0;
    assert_eq!(v.stringify().unwrap(), r#"["hello",-11,1,22]"#,);
}

#[test]
fn test_query_mixed() {
    let mut v: JsonValue = r#"[{"a": 0, "b": [-1, 0, 1]}, 0, 1, 2]"#.parse().unwrap();

    assert_eq!(
        v.query()
            .child(0)
            .child("b")
            .child_by(|v| matches!(v, JsonValue::Number(n) if *n > 0.0))
            .find(),
        Some(&JsonValue::Number(1.0)),
    );
    assert_eq!(
        v.query()
            .child("b")
            .child(0)
            .child_by(|v| matches!(v, JsonValue::Number(n) if *n > 0.0))
            .find(),
        None,
    );
    assert_eq!(
        v.query()
            .child_by(|v| matches!(v, JsonValue::Number(n) if *n > 0.0))
            .child("b")
            .child(0)
            .find(),
        None,
    );

    *v.query_mut()
        .child(0)
        .child("b")
        .child_by(|v| matches!(v, JsonValue::Number(n) if *n > 0.0))
        .get::<f64>()
        .unwrap() = 42.0;

    assert_eq!(
        v.query()
            .child(0)
            .child("b")
            .child_by(|v| matches!(v, JsonValue::Number(n) if *n >= 10.0))
            .find(),
        Some(&JsonValue::Number(42.0)),
    );
}

#[test]
fn test_query_find() {
    let mut v: JsonValue = "[0]".parse().unwrap();
    assert_eq!(v.query().child(0).find(), Some(&JsonValue::Number(0.0)));
    assert_eq!(v.query().child(1).find(), None);
    assert_eq!(
        v.query_mut().child(0).find(),
        Some(&mut JsonValue::Number(0.0)),
    );
    assert_eq!(v.query_mut().child(1).find(), None);
}

#[test]
fn test_query_get() {
    let mut v: JsonValue = "[0]".parse().unwrap();
    assert_eq!(v.query().child(0).get::<f64>(), Some(&0.0));
    assert_eq!(v.query().child(1).get::<f64>(), None);
    assert_eq!(v.query().child(0).get::<String>(), None);
    assert_eq!(v.query().child(1).get::<String>(), None);
    assert_eq!(v.query_mut().child(0).get::<f64>(), Some(&mut 0.0));
    assert_eq!(v.query_mut().child(1).get::<f64>(), None);
    assert_eq!(v.query_mut().child(0).get::<String>(), None);
    assert_eq!(v.query_mut().child(1).get::<String>(), None);
}

#[test]
fn test_query_exists() {
    let mut v: JsonValue = "[0]".parse().unwrap();
    assert!(v.query().exists());
    assert!(v.query().child(0).exists());
    assert!(!v.query().child(1).exists());
    assert!(!v.query().child(0).child(0).exists());
    assert!(!v.query().child("foo").exists());
    assert!(JsonQuery::new(&v).exists());
    assert!(!JsonQuery::default().exists());
}
