use std::collections::HashSet;
use std::sync::OnceLock;

fn delegated_events() -> &'static HashSet<&'static str> {
    static HASHSET: OnceLock<HashSet<&str>> = OnceLock::new();
    HASHSET.get_or_init(|| {
        let events = vec![
            "beforeinput",
            "click",
            "dblclick",
            "contextmenu",
            "focusin",
            "focusout",
            "input",
            "keydown",
            "keyup",
            "mousedown",
            "mousemove",
            "mouseout",
            "mouseover",
            "mouseup",
            "pointerdown",
            "pointermove",
            "pointerout",
            "pointerover",
            "pointerup",
            "touchend",
            "touchmove",
            "touchstart",
        ];
        HashSet::from_iter(events)
    })
}

// Could use OnceLock + HashSet for larger types
// given the size here it's better to just use slice
// Could also sort + binary serach, given size will also not
// do here but would consider w/ more elements
const DELEGATED_EVENTS: [&str; 22] = [
    "beforeinput",
    "click",
    "dblclick",
    "contextmenu",
    "focusin",
    "focusout",
    "input",
    "keydown",
    "keyup",
    "mousedown",
    "mousemove",
    "mouseout",
    "mouseover",
    "mouseup",
    "pointerdown",
    "pointermove",
    "pointerout",
    "pointerover",
    "pointerup",
    "touchend",
    "touchmove",
    "touchstart",
];

pub fn is_delegated_event(possible_event: &str) -> bool {
    DELEGATED_EVENTS.contains(&possible_event)
}
