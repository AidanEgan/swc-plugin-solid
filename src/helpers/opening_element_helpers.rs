use swc_core::{
    atoms::Atom,
    ecma::ast::{JSXAttrName, JSXAttrOrSpread, JSXOpeningElement},
};

pub fn parse_attrs(
    opening_el: &mut JSXOpeningElement,
) -> (Vec<(Option<Atom>, JSXAttrOrSpread)>, bool, bool) {
    let mut has_spread = false;
    let mut has_is = false;
    let res = opening_el
        .attrs
        .drain(..)
        .map(|attr_or_spread| {
            let name = match &attr_or_spread {
                JSXAttrOrSpread::SpreadElement(_) => {
                    has_spread = true;
                    None
                }
                JSXAttrOrSpread::JSXAttr(a) => {
                    let name = match &a.name {
                        JSXAttrName::Ident(i) => {
                            if i.sym.as_str() == "is" {
                                has_is = true;
                            }
                            i.sym.clone()
                        }
                        JSXAttrName::JSXNamespacedName(nn) => {
                            format!("{0}:{1}", nn.ns.sym.as_str(), nn.name.sym.as_str())
                                .as_str()
                                .into()
                        }
                    };
                    Some(name)
                }
            };
            (name, attr_or_spread)
        })
        .collect();
    (res, has_spread, has_is)
}
