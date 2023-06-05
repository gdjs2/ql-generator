pub mod chatgpt;

use crate::{
    extractor::Func,
};

/**
This is the back end of the whole engine. It receives
[`Func`] and decides whether this function is a specified
type function or not.
*/
pub trait Engine {
    /**
    Decide whether a function is a allocator or not.

    * return: [`bool`], whether it is a allocator or not
    */
    fn is_allocator(&self, f: &Func) -> bool;
}
