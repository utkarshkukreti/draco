use crate::url::Url;
use std::marker::PhantomData;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct State<'a> {
    url: &'a Url,
    index: usize,
}

pub trait Parse {
    type Output;

    fn parse_state(&self, state: &mut State) -> Option<Self::Output>;

    fn parse(&self, url: &Url) -> Option<Self::Output> {
        let mut state = State { url, index: 0 };
        let route = self.parse_state(&mut state)?;
        if state.index == state.url.path.len() {
            Some(route)
        } else {
            None
        }
    }

    fn optional(self) -> Optional<Self>
    where
        Self: Sized,
    {
        Optional(self)
    }
}

impl Parse for str {
    type Output = ();

    fn parse_state(&self, state: &mut State) -> Option<Self::Output> {
        if state
            .url
            .path
            .get(state.index)
            .map_or(false, |string| string == self)
        {
            state.index += 1;
            return Some(());
        }
        None
    }
}

impl Parse for &'static str {
    type Output = ();

    fn parse_state(&self, state: &mut State) -> Option<Self::Output> {
        (*self).parse_state(state)
    }
}

#[derive(Debug)]
pub struct Param<T: FromStr>(PhantomData<T>);

pub fn param<T: FromStr>() -> Param<T> {
    Param(PhantomData)
}

impl<T: FromStr> Parse for Param<T> {
    type Output = T;

    fn parse_state(&self, state: &mut State) -> Option<Self::Output> {
        if let Some(param) = state.url.path.get(state.index) {
            if let Ok(ok) = param.parse() {
                state.index += 1;
                return Some(ok);
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

    fn parse_state(&self, state: &mut State) -> Option<Self::Output> {
        if let Some(value) = state
            .url
            .query
            .iter()
            .find(|(k, _)| k == self.0)
            .map(|(_, v)| v)
        {
            if let Ok(ok) = value.parse() {
                return Some(ok);
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

    fn parse_state(&self, state: &mut State) -> Option<Self::Output> {
        if let Some(ref hash) = state.url.hash {
            if let Ok(ok) = hash.parse() {
                return Some(ok);
            }
        }
        None
    }
}

impl Parse for () {
    type Output = ();

    fn parse_state(&self, _state: &mut State) -> Option<Self::Output> {
        Some(())
    }
}

macro_rules! go {
    ($($($ident:ident)+,)+) => {
        $(
            impl<$($ident: Parse,)+> Parse for ($($ident,)+) {
                type Output = ($($ident::Output,)+);
                fn parse_state(&self, state: &mut State) -> Option<Self::Output> {
                    #[allow(non_snake_case)]
                    let ($($ident,)+) = self;
                    $(
                        #[allow(non_snake_case)]
                        let $ident = $ident.parse_state(state)?;
                    )*
                    Some(($($ident,)+))
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
    fn parse_state(&self, state: &mut State) -> Option<Self::Output> {
        let cloned = state.clone();
        if let Some(t) = self.0.parse_state(state) {
            Some(Some(t))
        } else {
            *state = cloned;
            Some(None)
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
            if let Some(t) = p.parse(self.url) {
                self.value = Some(f(t));
            }
        }
        self
    }

    pub fn value(self) -> Option<T> {
        self.value
    }
}

pub fn parse<T>(url: &Url) -> Parser<T> {
    Parser::new(url)
}
