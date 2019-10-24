use crate::url::Url;
use std::marker::PhantomData;
use std::str::FromStr;

pub trait Parse {
    type Output;

    fn parse(&self, url: &Url, index: usize) -> Option<(Self::Output, usize)>;

    fn optional(self) -> Optional<Self>
    where
        Self: Sized,
    {
        Optional(self)
    }
}

impl Parse for str {
    type Output = ();

    fn parse(&self, url: &Url, index: usize) -> Option<(Self::Output, usize)> {
        if url.path.get(index).map_or(false, |string| string == self) {
            Some(((), index + 1))
        } else {
            None
        }
    }
}

impl Parse for &'static str {
    type Output = ();

    fn parse(&self, url: &Url, index: usize) -> Option<(Self::Output, usize)> {
        Parse::parse(*self, url, index)
    }
}

#[derive(Debug)]
pub struct Param<T: FromStr>(PhantomData<T>);

pub fn param<T: FromStr>() -> Param<T> {
    Param(PhantomData)
}

impl<T: FromStr> Parse for Param<T> {
    type Output = T;

    fn parse(&self, url: &Url, index: usize) -> Option<(Self::Output, usize)> {
        if let Some(param) = url.path.get(index) {
            if let Ok(ok) = param.parse() {
                return Some((ok, index + 1));
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct Query<'a, T: FromStr>(&'a str, PhantomData<T>);

pub fn query<T: FromStr>(name: &str) -> Query<T> {
    Query(name, PhantomData)
}

impl<'a, T: FromStr> Parse for Query<'a, T> {
    type Output = T;

    fn parse(&self, url: &Url, index: usize) -> Option<(Self::Output, usize)> {
        if let Some(value) = url.query.iter().find(|(k, _)| k == self.0).map(|(_, v)| v) {
            if let Ok(ok) = value.parse() {
                return Some((ok, index));
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct Hash<T: FromStr>(PhantomData<T>);

pub fn hash<T: FromStr>() -> Hash<T> {
    Hash(PhantomData)
}

impl<T: FromStr> Parse for Hash<T> {
    type Output = T;

    fn parse(&self, url: &Url, index: usize) -> Option<(Self::Output, usize)> {
        if let Some(ref hash) = url.hash {
            if let Ok(ok) = hash.parse() {
                return Some((ok, index));
            }
        }
        None
    }
}

impl Parse for () {
    type Output = ();

    fn parse(&self, _url: &Url, index: usize) -> Option<(Self::Output, usize)> {
        Some(((), index))
    }
}

macro_rules! go {
    ($($($ident:ident)+,)+) => {
        $(
            impl<$($ident: Parse,)+> Parse for ($($ident,)+) {
                type Output = ($($ident::Output,)+);
                fn parse(&self, url: &Url, mut index: usize) -> Option<(Self::Output, usize)> {
                    #[allow(non_snake_case)]
                    let ($($ident,)+) = self;
                    $(
                        #[allow(non_snake_case)]
                        let ($ident, new_index) = $ident.parse(url, index)?;
                        index = new_index;
                    )*
                    Some((($($ident,)+), index))
                }
            }
        )+
    }
}

go! {
    A,
    A B,
    A B C,
    A B C D,
    A B C D E,
    A B C D E F,
    A B C D E F G,
    A B C D E F G H,
    A B C D E F G H I,
    A B C D E F G H I J,
}

#[derive(Debug)]
pub struct Optional<T: Parse>(T);

impl<T: Parse> Parse for Optional<T> {
    type Output = Option<T::Output>;
    fn parse(&self, url: &Url, index: usize) -> Option<(Self::Output, usize)> {
        if let Some((t, index)) = self.0.parse(url, index) {
            Some((Some(t), index))
        } else {
            Some((None, index))
        }
    }
}

#[derive(Debug)]
pub struct Parser<'a, T> {
    url: &'a Url,
    value: Option<T>,
}

impl<'a, T> Parser<'a, T> {
    pub fn new(url: &'a Url) -> Self {
        Parser { url, value: None }
    }

    pub fn alt<P: Parse>(mut self, p: P, f: impl Fn(P::Output) -> T) -> Self {
        if self.value.is_none() {
            if let Some((route, index)) = p.parse(self.url, 0) {
                if index == self.url.path.len() {
                    self.value = Some(f(route));
                    return self;
                }
            }
        }
        self
    }

    pub fn finish(self) -> Option<T> {
        self.value
    }
}

pub fn parse<T>(url: &Url) -> Parser<T> {
    Parser::new(url)
}
