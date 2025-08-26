use swc_core::common::{comments::Comments, BytePos};

pub fn should_skip<C: Clone + Comments>(
    require_import_source: &Option<String>,
    comments: &C,
    pos: BytePos,
) -> bool {
    if let Some(import_source) = require_import_source {
        let key = format!("@jsxImportSource {0}", import_source);
        if let Some(comments) = comments.get_leading(pos) {
            comments.iter().any(|comment| comment.text.contains(&key))
        } else {
            true
        }
    } else {
        false
    }
}
