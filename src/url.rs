use std::fmt;

pub mod parse;

#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct Url {
    path: Vec<String>,
    query: Vec<(String, String)>,
    hash: Option<String>,
}

impl Url {
    pub fn path(&self) -> &[String] {
        &self.path
    }

    pub fn query(&self) -> &[(String, String)] {
        &self.query
    }

    pub fn hash(&self) -> &Option<String> {
        &self.hash
    }
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
        for (index, (name, value)) in self.query.iter().enumerate() {
            if index > 0 {
                write!(f, "&")?;
            }
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

#[derive(Default)]
pub struct Builder {
    url: Url,
}

impl Builder {
    pub fn path(mut self, path: impl ToString) -> Self {
        self.url.path.push(path.to_string());
        self
    }

    pub fn query(mut self, name: impl ToString, value: impl ToString) -> Self {
        self.url.query.push((name.to_string(), value.to_string()));
        self
    }

    pub fn query_optional(mut self, name: impl ToString, value: Option<impl ToString>) -> Self {
        if let Some(value) = value {
            self.url.query.push((name.to_string(), value.to_string()));
        }
        self
    }

    pub fn hash(mut self, hash: Option<impl ToString>) -> Self {
        self.url.hash = hash.map(|v| v.to_string());
        self
    }

    pub fn finish(self) -> Url {
        self.url
    }
}

pub fn build() -> Builder {
    Builder::default()
}

#[cfg(test)]
mod tests {
    use super::{Builder, Url};
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
            std::dbg!(Url::from(url));
            assert_eq!(url, Url::from(url).to_string());
        }
        assert_eq!("/foo", Url::from("/foo#").to_string());

        let url = Builder::default()
            .path("foo")
            .query("bar", "baz")
            .path("quux")
            .query(1, 2)
            .query_optional("???", None::<String>)
            .hash(Some("foo"))
            .finish();

        assert_eq!("/foo/quux?bar=baz&1=2#foo", url.to_string());
    }
}
