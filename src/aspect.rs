use crate::{Attribute, Listener, Mailbox, Property};
use std::rc::Rc;
use web_sys as web;

#[derive(Debug)]
pub enum Aspect<Message> {
    Attribute(Attribute),
    Property(Property),
    Listener(Listener<Message>),
}

impl<Message: 'static> Aspect<Message> {
    pub(crate) fn do_map<NewMessage: 'static>(
        self,
        f: Rc<impl Fn(Message) -> NewMessage + 'static>,
    ) -> Aspect<NewMessage> {
        match self {
            Aspect::Attribute(attribute) => Aspect::Attribute(attribute),
            Aspect::Property(property) => Aspect::Property(property),
            Aspect::Listener(listener) => Aspect::Listener(listener.do_map(f)),
        }
    }
}

pub fn patch<Message>(
    new_aspects: &mut [Aspect<Message>],
    old_aspects: &[Aspect<Message>],
    element: &web::Element,
    mailbox: &Mailbox<Message>,
) {
    macro_rules! find {
        ($aspects:expr, $name:expr, $ty:ident) => {
            $aspects
                .iter()
                .filter_map(|aspect| match aspect {
                    Aspect::$ty(aspect) if aspect.name == $name => Some(aspect),
                    _ => None,
                })
                .next()
        };
    }

    for new_aspect in new_aspects.iter_mut() {
        match new_aspect {
            Aspect::Attribute(attribute) => {
                attribute.patch(find!(old_aspects, attribute.name, Attribute), element)
            }
            Aspect::Property(property) => {
                property.patch(find!(old_aspects, property.name, Property), element)
            }
            Aspect::Listener(listener) => listener.attach(element, mailbox),
        }
    }
    for old_aspect in old_aspects {
        match old_aspect {
            Aspect::Attribute(attribute) => {
                if find!(new_aspects, attribute.name, Attribute).is_none() {
                    attribute.remove(element);
                }
            }
            Aspect::Property(property) => {
                if find!(new_aspects, property.name, Property).is_none() {
                    property.remove(element);
                }
            }
            Aspect::Listener(listener) => listener.detach(element),
        }
    }
}

impl<Message> From<Attribute> for Aspect<Message> {
    fn from(attribute: Attribute) -> Self {
        Aspect::Attribute(attribute)
    }
}

impl<Message> From<Property> for Aspect<Message> {
    fn from(property: Property) -> Self {
        Aspect::Property(property)
    }
}

impl<Message> From<Listener<Message>> for Aspect<Message> {
    fn from(listener: Listener<Message>) -> Self {
        Aspect::Listener(listener)
    }
}
