use std::collections::HashMap;

// example query string
// a=1&b===&c=1&c=2

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>), // heap allocated (can grow unlike a normal array which requires a size to be specified)
}

impl<'buf> QueryString<'buf> {
    // Rust automatically infers that the returned reference is tied to `&self`
    pub fn get(&self, key: &str) -> Option<&Value<'buf>> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";

            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            data.entry(key)
                .and_modify(|existing| match existing {
                    Value::Single(prev_val) => *existing = Value::Multiple(vec![prev_val, val]),
                    Value::Multiple(vec) => vec.push(val),
                })
                .or_insert(Value::Single(val));
        }

        QueryString { data }
    }
}
