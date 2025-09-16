use swc_core::{
    atoms::Atom,
    ecma::ast::{JSXAttrName, JSXAttrOrSpread, JSXOpeningElement},
};

pub fn parse_attrs(opening_el: &mut JSXOpeningElement) -> Vec<(Option<Atom>, JSXAttrOrSpread)> {
    opening_el
        .attrs
        .drain(..)
        .map(|attr_or_spread| {
            let name = match &attr_or_spread {
                JSXAttrOrSpread::SpreadElement(_) => None,
                JSXAttrOrSpread::JSXAttr(a) => {
                    let name = match &a.name {
                        JSXAttrName::Ident(i) => i.sym.clone(),
                        JSXAttrName::JSXNamespacedName(nn) => nn.name.sym.clone(),
                    };
                    Some(name)
                }
            };
            (name, attr_or_spread)
        })
        .collect()
}
