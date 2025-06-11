//! Module that provides types describing the compile-time parsing context.

/// Kind of comment
pub enum CommentKind {
    SingleLine,
}

/// Context for parsing doc comments.
#[derive(Clone, Copy, Default)]
pub struct DocCtx {
    depth: usize,
}

impl DocCtx {
    /// Creates `DocCtx`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a context for parsing a block of code inside a doc comment.
    /// 
    /// # Arguments
    /// - `comment_kind` Kind of comment inside which the code block is located.
    pub fn deeper(&self, _comment_kind: CommentKind) -> Self {
        Self {
            depth: self.depth + 1,
        }
    }

    /// Gets the number of doc comments within which parsing is performed.
    pub fn depth(&self) -> usize {
        self.depth
    }
}

/// Context for parsing.
#[derive(Clone, Copy, Default)]
pub struct Ctx<C> {
    pub doc_ctx: DocCtx,
    pub additional: C,
}

impl<C> Ctx<C> {
    /// Creates `Ctx`.
    /// 
    /// # Arguments
    /// - `doc_ctx` Context for parsing doc comments.
    /// - `additional` Additional context that may be needed in local parsers.
    pub fn new(doc_ctx: DocCtx, additional: C) -> Self {
        Self {
            doc_ctx,
            additional,
        }
    }

    /// Creates a context for parsing a block of code inside a doc comment.
    /// 
    /// # Arguments
    /// - `comment_kind` Kind of comment inside which the code block is located.
    pub fn deeper(self, comment_kind: CommentKind) -> Self {
        Self {
            doc_ctx: self.doc_ctx.deeper(comment_kind),
            additional: self.additional,
        }
    }
}
