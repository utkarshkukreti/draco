#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Url {
    pub path: Vec<String>,
    pub query: Vec<(String, String)>,
    pub hash: Option<String>,
}

impl Url {
    pub fn new<T: Into<String>>(t: T) -> Url {
        let string = t.into();
        let (path_and_query, hash) = split(&string, '#');
        let (path, query) = split(path_and_query, '?');
        let path = path
            .split('/')
            .filter(|str| *str != "")
            .map(Into::into)
            .collect();
        let query = query
            .split('&')
            .map(|part| split(part, '='))
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        let hash = if hash == "" { None } else { Some(hash.into()) };

        return Url { path, query, hash };

        fn split(haystack: &str, needle: char) -> (&str, &str) {
            let mut splitted = haystack.splitn(2, needle);
            (splitted.next().unwrap(), splitted.next().unwrap_or(""))
        }
    }
}
