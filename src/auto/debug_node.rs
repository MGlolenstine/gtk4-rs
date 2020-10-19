// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use gsk_sys;
use std::fmt;
use RenderNode;

glib_wrapper! {
    pub struct DebugNode(Object<gsk_sys::GskDebugNode, DebugNodeClass>) @extends RenderNode;

    match fn {
        get_type => || gsk_sys::gsk_debug_node_get_type(),
    }
}

impl DebugNode {
    pub fn new<P: IsA<RenderNode>>(child: &P, message: &str) -> DebugNode {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(gsk_sys::gsk_debug_node_new(
                child.as_ref().to_glib_none().0,
                message.to_glib_full(),
            ))
        }
    }

    pub fn get_child(&self) -> Option<RenderNode> {
        unsafe { from_glib_none(gsk_sys::gsk_debug_node_get_child(self.to_glib_none().0)) }
    }

    pub fn get_message(&self) -> Option<GString> {
        unsafe { from_glib_none(gsk_sys::gsk_debug_node_get_message(self.to_glib_none().0)) }
    }
}

impl fmt::Display for DebugNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DebugNode")
    }
}
