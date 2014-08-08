#![doc(html_logo_url = "https://avatars0.githubusercontent.com/u/7853871?s=128", html_favicon_url = "https://avatars0.githubusercontent.com/u/7853871?s=256", html_root_url = "http://ironframework.io/core/urlencoded")]
#![license = "MIT"]

//! Url Encoded middleware for Iron.
//!
//! Parses "url encoded" data from client requests.
//! Capable of parsing both URL query strings and POST request bodies.

extern crate url;
extern crate iron;
extern crate serialize;

use iron::Request;
use url::form_urlencoded;
use std::collections::HashMap;

/// Mixin that attaches parsing methods for urlencoded data to a `Request`.
///
/// Each map is `Some` only if both of the following conditions are met:
///     1. Parsing of the field is enabled in the `UrlEncodedParser`.
///     2. The query string/body of the request is non-empty.
///
/// The values are stored in a vector so that keys which appear multiple times can map to
/// multple values.
/// e.g. "?a=b&a=c" is stored as `a => vec![b, c]`
pub trait UrlEncodedMixin {
    fn parse_urlencoded_query(&self) -> Option<HashMap<String, Vec<String>>>;
    fn parse_urlencoded_body(&self) -> Option<HashMap<String, Vec<String>>>;
}

impl UrlEncodedMixin for Request {
    fn parse_urlencoded_query(&self) -> Option<HashMap<String, Vec<String>>> {
        match self.url.query {
            Some(ref query) => create_param_hashmap(query.as_slice()),
            None => None
        }
    }

    fn parse_urlencoded_body(&self) -> Option<HashMap<String, Vec<String>>> {
        create_param_hashmap(self.body.as_slice())
    }
}

/// Parses a urlencoded string into an optional HashMap.
fn create_param_hashmap(data: &str) -> Option<HashMap<String, Vec<String>>> {
    match data {
        "" => None,
        _ => Some(combine_duplicates(form_urlencoded::parse_str(data)))
    }
}

fn combine_duplicates(q: Vec<(String, String)>) -> HashMap<String, Vec<String>> {

    let mut deduplicated: HashMap<String, Vec<String>> = HashMap::new();

    for (k, v) in q.move_iter() {
        deduplicated.find_with_or_insert_with(
            k, v,
            // Already a Vec here, push onto it
            |_, already, new| { already.push(new); },
            // No value, create a one-element Vec.
            |_, v| vec![v]
        );
    }

    deduplicated
}

#[test]
fn test_combine_duplicates() {
    let my_vec = vec!(("band".to_string(), "arctic monkeys".to_string()),
                      ("band".to_string(), "temper trap".to_string()),
                      ("color".to_string(),"green".to_string()));
    let answer = combine_duplicates(my_vec);
    let mut control = HashMap::new();
    control.insert("band".to_string(),
                   vec!("arctic monkeys".to_string(), "temper trap".to_string()));
    control.insert("color".to_string(), vec!("green".to_string()));
    assert_eq!(answer, control);
}
