use JsonValue;

fn generate_string(s: &String) -> String {
    s.clone()
}

fn generate_array(v: &Vec<JsonValue>) -> String {
    let mut s = v.iter().fold(String::new(), |acc, e| acc + &generate(e) + ",");
    s.pop();  // Remove trailing comma
    s
}

fn generate_object(m: &HashMap<String, JsonValue>) -> String {
    let mut s = '{'.to_string();
    for (k, v) in m {
        s = s + &generate_string(k) + ":" + &generate(v) + ",";
    }
    s.pop();  // Remove trailing comma
    s.push('}');
    s
}

pub fn generate(v: &JsonValue) -> String {
    match v {
        &JsonValue::Number(n) => n.to_string(),
        &JsonValue::Boolean(b) => b.to_string(),
        &JsonValue::String(ref s) => generate_string(&s),
        &JsonValue::Null => "null".to_string(),
        &JsonValue::Array(ref a) => generate_array(&a),
        &JsonValue::Object(ref o) => generate_object(&o),
    }
}

impl JsonValue {
    pub fn to_string(&self) -> String {
        generate(self)
    }
}

