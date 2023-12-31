use std::ops::Deref;

/// Symbol found in the source input.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Symbol<'src> {
    /// Source input of ascii math.
    src: &'src str,
    /// Content of this symbols.
    pub(crate) content: &'src str,
    /// Offset at which the input is found in the input.
    offs: usize,
}

fn scan_str<S>(src: &S) -> Vec<Symbol<'_>>
where
    S: AsRef<str> + ?Sized,
{
    let src = src.as_ref();
    let mut symbols: Vec<_> = src
        .split("")
        .skip(1)
        .enumerate()
        .map(|(offs, content)| Symbol { src, content, offs })
        .collect();

    symbols.pop();
    symbols
}

impl<'src> Symbol<'src> {
    /// Extracts the string slice from ascii math source corresponding to the given [`Symbol`]s.
    ///
    /// # Panics
    ///
    /// Panics (in debug version) if the given slice contains [`Symbol`]s with references to
    /// different sources. Such case indicates a clear bug, and should not be possible to construct
    /// with the given crate-level API.
    pub fn as_str(symbols: &[Self]) -> Option<&'src str> {
        debug_assert!(symbols
            .windows(2)
            .all(|window| window[0].src == window[1].src));

        let first = symbols.first()?;
        let src = first.src;
        let start = first.offs;
        let end = symbols.last()?.offs;

        Some(&src[start..=end])
    }

    pub(crate) fn is_digit(&self) -> bool {
        matches!(
            self.content,
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
        )
    }

    pub(crate) fn is_dot(&self) -> bool {
        matches!(self.content, ".")
    }

    pub(crate) fn is_whitespace(&self) -> bool {
        self.content.chars().all(|c| c.is_whitespace())
    }

    pub(crate) fn is_letter(&self) -> bool {
        self.content.chars().all(|c| c.is_ascii_alphabetic())
    }
}

pub struct Symbols<'src>(pub(crate) Vec<Symbol<'src>>);

impl<'s> From<Vec<Symbol<'s>>> for Symbols<'s> {
    fn from(value: Vec<Symbol<'s>>) -> Self {
        Symbols(value)
    }
}

impl<'src> Deref for Symbols<'src> {
    type Target = <Vec<Symbol<'src>> as Deref>::Target;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<'src, T> From<&'src T> for Symbols<'src>
where
    T: AsRef<str>,
{
    fn from(value: &'src T) -> Self {
        Symbols(scan_str(value.as_ref()))
    }
}

impl<'src> From<&'src str> for Symbols<'src> {
    fn from(value: &'src str) -> Self {
        Symbols(scan_str(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_str() {
        let src = "Hi !";
        let symbols = scan_str(src);

        assert_eq!(
            symbols,
            vec![
                Symbol {
                    content: "H",
                    offs: 0,
                    src,
                },
                Symbol {
                    content: "i",
                    offs: 1,
                    src,
                },
                Symbol {
                    content: " ",
                    offs: 2,
                    src
                },
                Symbol {
                    content: "!",
                    offs: 3,
                    src
                },
            ]
        );
    }

    #[test]
    fn test_as_str() {
        let src = "Hi there!";
        let symbols = scan_str(src);

        assert_eq!(Symbol::as_str(&symbols), Some("Hi there!"));
        assert_eq!(Symbol::as_str(&symbols[0..=2]), Some("Hi "));
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn mismatched_symbols() {
        let symbols = vec![
            Symbol {
                content: "H",
                offs: 0,
                src: "Hi",
            },
            Symbol {
                content: "h",
                offs: 1,
                src: "There",
            },
        ];

        let _ = Symbol::as_str(&symbols);
    }
}
