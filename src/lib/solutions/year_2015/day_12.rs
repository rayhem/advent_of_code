use crate::utils::solution::Solution;
pub struct Day12 {}

impl Solution for Day12 {
    fn part_one(&self, input: &str) -> Option<String> {
        Some(total_json(input, |_| true).to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        Some(
            total_json(input, |value| {
                *value != serde_json::Value::String("red".to_string())
            })
            .to_string(),
        )
    }
}

fn total_json(input: &str, is_valid: fn(&serde_json::Value) -> bool) -> i64 {
    let json: serde_json::Value = serde_json::from_str(input).unwrap();

    let mut stack = json.as_object().unwrap().values().collect::<Vec<_>>();
    let mut total = 0;

    while let Some(item) = stack.pop() {
        match item {
            serde_json::Value::Null => continue,
            serde_json::Value::Bool(_) => continue,
            serde_json::Value::Number(n) => total += n.as_i64().unwrap(),
            serde_json::Value::String(_) => continue,
            serde_json::Value::Array(a) => stack.extend(a),
            serde_json::Value::Object(obj) => {
                if obj.values().all(is_valid) {
                    stack.extend(obj.values());
                }
            }
        }
    }

    total
}

crate::verify!(Day12, crate::my_input!("2015", "12"), "119433", "68466");
