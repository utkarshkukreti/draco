use crate::{
    element::{Children, Ns},
    Element, NonKeyedElement, S,
};

macro_rules! elements {
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

elements! {
    a abbr address article aside audio b bdi bdo blockquote br button canvas caption cite code col
    colgroup datalist dd del details dfn div dl dt em embed fieldset figcaption figure footer form
    h1 h2 h3 h4 h5 h6 header hr i iframe img input ins kbd label legend li main mark math menu
    menuitem meter nav object ol optgroup option output p param pre progress q rp rt ruby s samp
    section select small source span strong sub summary sup table tbody td textarea tfoot th thead
    time tr track u ul var video wbr
}

impl<C: Children> Element<C>
where
    C::Message: 'static,
{
    pub fn value(self, value: impl Into<S>) -> Self {
        self.property("value", value.into())
    }
}

macro_rules! string_attributes {
    (
        $($ident:ident => $name:expr,)+
    ) => {
        impl<C: Children> Element<C> where C::Message: 'static {
            $(
                pub fn $ident(self, value: impl Into<S>) -> Self {
                    self.attribute($name, value.into())
                }
            )+
        }
    }
}

macro_rules! bool_properties {
    (
        $($ident:ident => $name:expr,)+
    ) => {
        impl<C: Children> Element<C> where C::Message: 'static {
            $(
                pub fn $ident(self, value: bool) -> Self {
                    self.property($name, value)
                }
            )+
        }
    }
}

macro_rules! to_string_attributes {
    (
        $($ident:ident: $ty:ty => $name:expr,)+
    ) => {
        impl<C: Children> Element<C> where C::Message: 'static {
            $(
                pub fn $ident(self, value: $ty) -> Self {
                    self.attribute($name, value.to_string())
                }
            )+
        }
    }
}

string_attributes! {
    abbr => "abbr",
    accept => "accept",
    accept_charset => "accept-charset",
    accesskey => "accesskey",
    action => "action",
    allow => "allow",
    alt => "alt",
    as_ => "as",
    autocapitalize => "autocapitalize",
    charset => "charset",
    cite => "cite",
    class => "class",
    color => "color",
    content => "content",
    coords => "coords",
    crossorigin => "crossorigin",
    data => "data",
    datetime => "datetime",
    decoding => "decoding",
    dir => "dir",
    dirname => "dirname",
    download => "download",
    enctype => "enctype",
    enterkeyhint => "enterkeyhint",
    for_ => "for",
    form => "form",
    formaction => "formaction",
    formenctype => "formenctype",
    formmethod => "formmethod",
    formtarget => "formtarget",
    headers => "headers",
    height => "height",
    href => "href",
    hreflang => "hreflang",
    http_equiv => "http-equiv",
    id => "id",
    inputmode => "inputmode",
    integrity => "integrity",
    is => "is",
    itemid => "itemid",
    itemprop => "itemprop",
    itemref => "itemref",
    itemtype => "itemtype",
    kind => "kind",
    label => "label",
    lang => "lang",
    list => "list",
    manifest => "manifest",
    maxlength => "maxlength",
    media => "media",
    method => "method",
    name => "name",
    nonce => "nonce",
    pattern => "pattern",
    ping => "ping",
    placeholder => "placeholder",
    poster => "poster",
    preload => "preload",
    referrerpolicy => "referrerpolicy",
    rel => "rel",
    sandbox => "sandbox",
    scope => "scope",
    shape => "shape",
    sizes => "sizes",
    slot => "slot",
    src => "src",
    srcdoc => "srcdoc",
    srclang => "srclang",
    srcset => "srcset",
    target => "target",
    title => "title",
    type_ => "type",
    usemap => "usemap",
//    value => "value",
    width => "width",
    wrap => "wrap",
}

bool_properties! {
//    allowfullscreen => "allowfullscreen",
//    allowpaymentrequest => "allowpaymentrequest",
//    async_ => "async",
//    autocomplete => "autocomplete",
    autofocus => "autofocus",
    autoplay => "autoplay",
    checked => "checked",
    contenteditable => "contentEditable",
    controls => "controls",
    default => "default",
//    defer => "defer",
    disabled => "disabled",
//    draggable => "draggable",
//    formnovalidate => "formnovalidate",
//    hidden => "hidden",
    ismap => "isMap",
//    itemscope => "itemscope",
    loop_ => "loop",
    multiple => "multiple",
//    muted => "muted",
//    nomodule => "nomodule",
    novalidate => "noValidate",
//    open => "open",
//    playsinline => "playsinline",
    readonly => "readOnly",
    required => "required",
    reversed => "reversed",
    selected => "selected",
    spellcheck => "spellcheck",
//    translate => "translate",
//    typemustmatch => "typemustmatch",
}

to_string_attributes! {
    cols: i32 => "cols",
    colspan: i32 => "colspan",
    high: f64 => "high",
    low: f64 => "low",
    max: f64 => "max",
    min: f64 => "min",
    minlength: i32 => "minlength",
    optimum: f64 => "optimum",
    rows: i32 => "rows",
    rowspan: i32 => "rowspan",
    size: i32 => "size",
    span: i32 => "span",
    start: i32 => "start",
    step: f64 => "step",
    tabindex: i32 => "tabindex",
}
