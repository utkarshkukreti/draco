use crate::{element::Ns, Element, NonKeyedElement};

macro_rules! elements {
    ($($ident:ident)+) => {
        $(
            pub fn $ident<Message: 'static>() -> NonKeyedElement<Message> {
                Element::new(Ns::Svg, stringify!($ident))
            }
        )+
        pub mod keyed {
            use crate::{Element, element::Ns, KeyedElement};
            $(
                pub fn $ident<Message: 'static>() -> KeyedElement<Message> {
                    Element::new(Ns::Svg, stringify!($ident))
                }
            )+
        }
    }
}

elements! {
    svg circle ellipse image line path polygon polyline rect title
}
