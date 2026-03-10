use crate::syntax_kind::SyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CobolLanguage {}

impl rowan::Language for CobolLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 < SyntaxKind::__LAST as u16);
        // SAFETY: SyntaxKind is repr(u16) and we checked the range
        unsafe { std::mem::transmute::<u16, SyntaxKind>(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        kind.into()
    }
}

pub type SyntaxNode = rowan::SyntaxNode<CobolLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<CobolLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<CobolLanguage>;
