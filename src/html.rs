use crate::{
    element::{AttrValue, Children, Ns},
    Element, NonKeyedElement, S,
};

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

macro_rules! attributes {
    (
        $(
            $ident:ident
            :
            $ty:ty
            =>
            $name:literal
        ,)+
    ) => {
        impl<C: Children> Element<C> where C::Message: 'static {
            $(
                pub fn $ident<Value: Into<$ty> + Into<AttrValue>>(self, value: Value) -> Self {
                    self.attr($name, <Value as Into<AttrValue>>::into(value))
                }
            )+
        }
    }
}

attributes! {
    abbr: S => "abbr",
    accept: S => "accept",
    accept_charset: S => "accept-charset",
    accesskey: S => "accesskey",
    action: S => "action",
    allow: S => "allow",
    allowfullscreen: bool => "allowfullscreen",
    allowpaymentrequest: bool => "allowpaymentrequest",
    alt: S => "alt",
    as_: S => "as",
    async_: bool => "async",
    autocapitalize: S => "autocapitalize",
    autocomplete: bool => "autocomplete",
    autofocus: bool => "autofocus",
    autoplay: bool => "autoplay",
    charset: S => "charset",
    cite: S => "cite",
    color: S => "color",
    cols: i32 => "cols",
    colspan: i32 => "colspan",
    content: S => "content",
    contenteditable: bool => "contenteditable",
    controls: bool => "controls",
    coords: S => "coords",
    crossorigin: S => "crossorigin",
    data: S => "data",
    datetime: S => "datetime",
    decoding: S => "decoding",
    default: bool => "default",
    defer: bool => "defer",
    dir: S => "dir",
    dirname: S => "dirname",
    disabled: bool => "disabled",
    download: S => "download",
    draggable: bool => "draggable",
    enctype: S => "enctype",
    enterkeyhint: S => "enterkeyhint",
    for_: S => "for",
    form: S => "form",
    formaction: S => "formaction",
    formenctype: S => "formenctype",
    formmethod: S => "formmethod",
    formnovalidate: bool => "formnovalidate",
    formtarget: S => "formtarget",
    headers: S => "headers",
    height: S => "height",
    hidden: bool => "hidden",
    high: f64 => "high",
    href: S => "href",
    hreflang: S => "hreflang",
    http_equiv: S => "http-equiv",
    id: S => "id",
    inputmode: S => "inputmode",
    integrity: S => "integrity",
    is: S => "is",
    ismap: bool => "ismap",
    itemid: S => "itemid",
    itemprop: S => "itemprop",
    itemref: S => "itemref",
    itemscope: bool => "itemscope",
    itemtype: S => "itemtype",
    kind: S => "kind",
    label: S => "label",
    lang: S => "lang",
    list: S => "list",
    loop_: bool => "loop",
    low: f64 => "low",
    manifest: S => "manifest",
    max: f64 => "max",
    maxlength: S => "maxlength",
    media: S => "media",
    method: S => "method",
    min: f64 => "min",
    minlength: i32 => "minlength",
    multiple: bool => "multiple",
    muted: bool => "muted",
    name: S => "name",
    nomodule: bool => "nomodule",
    nonce: S => "nonce",
    novalidate: bool => "novalidate",
    open: bool => "open",
    optimum: f64 => "optimum",
    pattern: S => "pattern",
    ping: S => "ping",
    placeholder: S => "placeholder",
    playsinline: bool => "playsinline",
    poster: S => "poster",
    preload: S => "preload",
    readonly: bool => "readonly",
    referrerpolicy: S => "referrerpolicy",
    rel: S => "rel",
    required: bool => "required",
    reversed: bool => "reversed",
    rows: i32 => "rows",
    rowspan: i32 => "rowspan",
    sandbox: S => "sandbox",
    scope: S => "scope",
    selected: bool => "selected",
    shape: S => "shape",
    size: i32 => "size",
    sizes: S => "sizes",
    slot: S => "slot",
    span: i32 => "span",
    spellcheck: bool => "spellcheck",
    src: S => "src",
    srcdoc: S => "srcdoc",
    srclang: S => "srclang",
    srcset: S => "srcset",
    start: i32 => "start",
    step: f64 => "step",
    tabindex: i32 => "tabindex",
    target: S => "target",
    title: S => "title",
    translate: bool => "translate",
    type_: S => "type",
    typemustmatch: bool => "typemustmatch",
    usemap: S => "usemap",
    value: S => "value",
    width: i32 => "width",
    wrap: S => "wrap",
}
