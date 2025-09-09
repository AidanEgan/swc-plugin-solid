use super::SolidJsVisitor;
use crate::config::PluginArgs;

use swc_core::common::{comments::Comments, SourceMapper};

pub fn create_solidjs_visitor<C: Clone + Comments, S: SourceMapper>(
    source_map: std::sync::Arc<S>,
    comments: C,
    plugin_options: PluginArgs,
) -> SolidJsVisitor<C, S> {
    /*
    * Would this be more performant? Documentation is sparse
       swc_core::common::pass::Optional::new(
           SolidJsVisitor::new(source_map, comments, plugin_options, filename),
           true,
       )
    */
    SolidJsVisitor::new(source_map, comments, plugin_options)
}
