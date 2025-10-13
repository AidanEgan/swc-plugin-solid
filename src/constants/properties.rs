// list of lowercase booleans
// Alphabetized
const BOOLEAN_ATTRIBUTES: [&str; 41] = [
    "adauctionheaders", // experimental
    "allowfullscreen",
    "async",
    "alpha",     // HTMLInputElement
    "autofocus", // HTMLElement prop
    "autoplay",
    "browsingtopics", // experimental
    "checked",
    "controls",
    "credentialless", // experimental
    "default",
    "defaultchecked",
    "defaultmuted",
    "defaultselected",
    "defer",
    "disabled",
    "disablepictureinpicture",
    "disableremoteplayback",
    "formnovalidate",
    "hidden", // HTMLElement prop - not a boolean
    "indeterminate",
    "inert", // HTMLElement prop
    "ismap",
    "loop",
    "multiple",
    "muted",
    "nomodule",
    "novalidate",
    "open",
    "playsinline",
    "preservespitch", // appears as camelCase property only (not as attribute)
    "readonly",
    "required",
    "reversed",
    "seamless", // HTMLIframeElement - non-standard
    "selected",
    "shadowrootclonable",
    "shadowrootcustomelementregistry", // experimental - doesnt seem to have a prop yet
    "shadowrootdelegatesfocus",
    "shadowrootserializable", // experimental
    "sharedstoragewritable",  // experimental
];

const PROPERTIES: [&str; 22] = [
    // locked to properties
    "className",
    "value",
    // booleans with camelCase
    "readOnly",
    "noValidate",
    "formNoValidate",
    "isMap",
    "noModule",
    "playsInline",
    "adAuctionHeaders", // experimental
    "allowFullscreen",
    "browsingTopics", // experimental
    "defaultChecked",
    "defaultMuted",
    "defaultSelected",
    "disablePictureInPicture",
    "disableRemotePlayback",
    "preservesPitch",
    "shadowRootClonable",
    "shadowRootCustomElementRegistry", // experimental
    "shadowRootDelegatesFocus",
    "shadowRootSerializable", // experimental
    "sharedStorageWritable",  // experimental
];

//const CHILD_PROPERTIES: [&str; 4] = ["innerHTML", "textContent", "innerText", "children"];

/*
// React Compat
const Aliases = /*#__PURE__*/ Object.assign(Object.create(null), {
  className: "class",
  htmlFor: "for"
});
*/

// Keep sorted will binary search
const PROP_ALIASES: [(&str, &str, &str); 19] = [
    // A
    ("adauctionheaders", "adAuctionHeaders", "IFRAME"),
    ("allowfullscreen", "allowFullscreen", "IFRAME"),
    // B
    ("browsingtopics", "browsingTopics", "IMG"),
    // C
    // D
    ("defaultchecked", "defaultChecked", "INPUT"),
    ("defaultmuted", "defaultMuted", "AUDIO;VIDEO"),
    ("defaultselected", "defaultSelected", "OPTION"),
    (
        "disablepictureinpicture",
        "disablePictureInPicture",
        "VIDEO",
    ),
    (
        "disableremoteplayback",
        "disableRemotePlayback",
        "AUDIO;VIDEO",
    ),
    // E
    // F
    ("formnovalidate", "formNoValidate", "BUTTON;INPUT"),
    // F
    // H
    // I
    ("ismap", "isMap", "IMG"),
    // J
    // K
    // L
    // M
    // N
    ("nomodule", "noModule", "SCRIPT"),
    ("novalidate", "noValidate", "FORM"),
    // O
    // P
    ("playsinline", "playsInline", "VIDEO"),
    ("preservespitch", "preservesPitch", "AUDIO;VIDEO"),
    // Q
    // R
    ("readonly", "readOnly", "INPUT;TEXTAREA"),
    // S
    ("shadowrootclonable", "shadowRootClonable", "TEMPLATE"),
    (
        "shadowrootdelegatesfocus",
        "shadowRootDelegatesFocus",
        "TEMPLATE",
    ),
    (
        "shadowrootserializable",
        "shadowRootSerializable",
        "TEMPLATE",
    ),
    (
        "sharedstoragewritable",
        "sharedStorageWritable",
        "IFRAME;IMG",
    ),
    // T,U,V,W,X,Y,Z
];

const SELF_CLOSING_TAGS: [&str; 14] = [
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr",
];

pub fn is_self_closing(tag: &str) -> bool {
    SELF_CLOSING_TAGS.binary_search(&tag).is_ok()
}

pub fn get_bool_attr<'a, 'b>(test: &'a str, element: Option<&'b str>) -> Option<&'a str> {
    if BOOLEAN_ATTRIBUTES.binary_search(&test).is_ok() {
        if let Some(element) = element {
            if let Ok(index) = PROP_ALIASES.binary_search_by_key(&test, |&(a, _, _)| a) {
                let element = element.to_uppercase();
                let (name, alias, elements) = PROP_ALIASES[index];
                let mut els = elements.split(";");
                if els.find(|&el| el == element.as_str()).is_some() {
                    Some(alias)
                } else {
                    Some(name)
                }
            } else {
                Some(test)
            }
        } else {
            Some(test)
        }
    } else if PROPERTIES.contains(&test) {
        Some(test)
    } else {
        None
    }
}
