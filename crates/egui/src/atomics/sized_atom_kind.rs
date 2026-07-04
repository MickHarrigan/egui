use crate::{AtomLayout, Image, SizedAtomLayout};
use emath::Vec2;
use epaint::Galley;
use std::rc::Rc;
use std::sync::Arc;

/// A sized [`crate::AtomKind`].
#[derive(Clone, Debug)]
pub enum SizedAtomKind<'a> {
    Empty {
        size: Option<Vec2>,
    },
    Text(Arc<Galley>),
    Image {
        image: Image<'a>,
        size: Vec2,
    },
    Layout {
        /// A shared handle to the original (unmeasured) layout, kept so a grown atom can be
        /// re-measured — and so re-wrap its contents — at the size its parent actually paints it
        /// at, without deep-cloning. See [`SizedAtomLayout::paint_at`].
        source: Rc<AtomLayout<'a>>,

        /// The layout measured at its natural size, used for the parent's own sizing.
        sized: Box<SizedAtomLayout<'a>>,
    },
}

impl Default for SizedAtomKind<'_> {
    fn default() -> Self {
        Self::Empty { size: None }
    }
}

impl SizedAtomKind<'_> {
    /// Get the calculated size.
    pub fn size(&self) -> Vec2 {
        match self {
            SizedAtomKind::Text(galley) => galley.size(),
            SizedAtomKind::Image { image: _, size } => *size,
            SizedAtomKind::Empty { size } => size.unwrap_or_default(),
            SizedAtomKind::Layout { sized, .. } => sized.outer_size,
        }
    }
}
