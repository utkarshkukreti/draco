use std::fmt;

pub mod parse;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Url {
    pub path: Vec<String>,
    pub query: Vec<(String, String)>,
    pub hash: Option<String>,
}

impl<T: Into<String>> From<T> for Url {
    fn from(t: T) -> Self {
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
            .filter(|(k, v)| !k.is_empty() || !v.is_empty())
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

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for p in &self.path {
            write!(f, "/{}", p)?;
        }
        if !self.query.is_empty() {
            write!(f, "?")?;
        }
        for (name, value) in &self.query {
            write!(f, "{}", name)?;
            if !value.is_empty() {
                write!(f, "={}", value)?;
            }
        }
        if let Some(hash) = &self.hash {
            write!(f, "#{}", hash)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Url;
    #[test]
    fn t() {
        let urls = [
            "",
            "/foo",
            "/foo/bar",
            "/foo?bar",
            "/foo#bar",
            "/foo?bar#baz",
            "/foo?bar=baz#quux",
        ];
        for &url in &urls {
            dbg!(Url::from(url));
            assert_eq!(url, Url::from(url).to_string());
        }
        assert_eq!("/foo", Url::from("/foo#").to_string());
    }
}
