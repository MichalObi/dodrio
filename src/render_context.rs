use crate::{
    cached_set::{CacheId, CachedSet},
    Node,
};
use bumpalo::Bump;
use std::fmt;

/// Common context available to all `Render` implementations.
///
/// Notably, the `RenderContext` gives access to the bump arena that the virtual
/// DOM should be allocated within. This is available via the `bump` field.
pub struct RenderContext<'a> {
    /// The underlying bump arena that virtual DOMs are rendered into.
    ///
    /// ## Example
    ///
    /// ```
    /// use dodrio::RenderContext;
    ///
    /// // Given a rendering context, allocate an i32 inside its bump arena.
    /// fn foo<'a>(cx: &mut RenderContext<'a>) -> &'a mut i32 {
    ///     cx.bump.alloc(42)
    /// }
    /// ```
    pub bump: &'a Bump,

    pub(crate) cached_set: &'a crate::RefCell<CachedSet>,

    // Prevent exhaustive matching on the rendering context, so we can always
    // add more members in a semver-compatible way.
    _non_exhaustive: (),
}

impl fmt::Debug for RenderContext<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RenderContext")
            .field("bump", &self.bump)
            .finish()
    }
}

impl<'a> RenderContext<'a> {
    pub_unstable_internal! {
        pub(crate) fn new(bump: &'a Bump, cached_set: &'a crate::RefCell<CachedSet>) -> Self {
            RenderContext {
                bump,
                cached_set,
                _non_exhaustive: (),
            }
        }
    }

    pub(crate) fn cache<F>(&self, f: F) -> CacheId
    where
        F: for<'b> FnOnce(&mut RenderContext<'b>) -> Node<'b>,
    {
        CachedSet::insert(self.cached_set, |bump, cached_set| {
            let mut nested_cx = RenderContext::new(bump, cached_set);
            f(&mut nested_cx)
        })
    }
}

impl<'a, 'b> From<&'b RenderContext<'a>> for &'a Bump {
    #[inline]
    fn from(cx: &'b RenderContext<'a>) -> &'a Bump {
        cx.bump
    }
}

impl<'a, 'b, 'c> From<&'c &'b mut RenderContext<'a>> for &'a Bump {
    #[inline]
    fn from(cx: &'c &'b mut RenderContext<'a>) -> &'a Bump {
        cx.bump
    }
}
