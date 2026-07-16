use std::collections::HashMap;

// example query string
// a=1&b===&c=1&c=2

pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>), // heap allocated (can grow unlike a normal array which requires a size to be specified)
}

impl<'buf> QueryString<'buf> {
    pub fn get(&'_ self, key: &str) -> Option<&'_ Value<'_>> {
        self.data.get(key)
    }
}
