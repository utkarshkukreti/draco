use crate::{element::Ns, Element, NonKeyedElement};

macro_rules! names {
    ($($ident:ident)+) => {
        $(
            pub fn $ident<Message: 'static>() -> NonKeyedElement<Message> {
                Element::new(Ns::Html, stringify!($ident))
            }
        )+
        pub mod keyed {
            use crate::{Element, element::Ns, KeyedElement};
            $(
                pub fn $ident<Message: 'static>() -> KeyedElement<Message> {
                    Element::new(Ns::Html, stringify!($ident))
                }
            )+
        }
    }
}

names! {
    a abbr address article aside audio b bdi bdo blockquote br button canvas caption cite code col
    colgroup datalist dd del details dfn div dl dt em embed fieldset figcaption figure footer form
    h1 h2 h3 h4 h5 h6 header hr i iframe img input ins kbd label legend li main mark math menu
    menuitem meter nav object ol optgroup option output p param pre progress q rp rt ruby s samp
    section select small source span strong sub summary sup table tbody td textarea tfoot th thead
    time tr track u ul var video wbr
}
