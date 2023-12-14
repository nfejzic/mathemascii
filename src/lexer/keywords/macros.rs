// const KEYWORDS = define_keywords!(...)

macro_rules! define_keywords {
    ($($($keyword:literal)|+),+) => {
        &[
            $(
                &[$($keyword,)+],
            )+
        ]
    };
}

macro_rules! generate_impl {
    ($kind:ident, $struct:ident, $($($lit:literal)|* => $var:ident),* $(,prefixes: $($($p_kind:ident)|* => $longer:literal),*)?) => {
        const LITERALS: &[&[&str]] = $crate::lexer::keywords::macros::define_keywords!(
            $($($lit)|*),*
        );

        #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
        pub enum $kind {
        $(
            $var,
        )*
        }

        impl AsRef<str> for $kind {
            fn as_ref(&self) -> &str {
                match self {
                    $(
                        $kind::$var => ($($lit),*,).0,
                    )*
                }
            }
        }

        impl TryFrom<&str> for $kind {
            type Error = ();

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                let kind = match value {
                    $(
                    $($lit)|* => $kind::$var,
                    )*
                    _ => return Err(()),
                };

                Ok(kind)
            }
        }

        fn map() -> &'static ::std::collections::BTreeMap<&'static str, $kind> {
            use ::std::{collections::BTreeMap, sync::OnceLock};
            static KEYWORDS: OnceLock<BTreeMap<&str, $kind>> = OnceLock::new();

            KEYWORDS.get_or_init(|| {
                LITERALS
                    .iter()
                    .flat_map(|s| s.iter())
                    .map(|&arr_str| (arr_str, $kind::try_from(arr_str).unwrap()))
                    .collect()
            })
        }

        const fn min_len() -> usize {
            let mut min_len = max_len();

            let mut i = 0;

            while i < LITERALS.len() {
                let mut j = 0;

                while j < LITERALS[i].len() {
                    let len = LITERALS[i][j].len();
                    if len < min_len {
                        min_len = len;
                    }
                    j += 1;
                }

                i += 1;
            }

            min_len
        }

        const fn max_len() -> usize {
            let mut max_len = 0;

            let mut i = 0;

            while i < LITERALS.len() {
                let mut j = 0;

                while j < LITERALS[i].len() {
                    let len = LITERALS[i][j].len();

                    if len > max_len {
                        max_len = len;
                    }

                    j += 1;
                }

                i += 1;
            }

            max_len
        }

        impl $crate::lexer::KeywordKind for $kind {
            fn prefix_of(&self) -> Option<usize> {
                match *self {
                    $(
                    $(
                        $($kind::$p_kind)|* => Some($longer.len()),
                    )*
                    )?

                    _ => None,
                }
            }
        }

        pub struct $struct;

        impl $crate::lexer::Keyword for $struct {
            const MAX_LEN: usize = max_len();
            const MIN_LEN: usize = min_len();

            type Kind = $kind;

            fn get(key: &str) -> Option<Self::Kind> {
                map().get(key).copied()
            }

            fn starts_with(symbol: $crate::scanner::Symbol<'_>) -> bool {
                use std::sync::OnceLock;
                static FIRST_SYMBOLS: OnceLock<Vec<&str>> = OnceLock::new();

                FIRST_SYMBOLS
                    .get_or_init(|| {
                        self::LITERALS
                            .iter()
                            .flat_map(|s| s.iter())
                            .filter_map(|lit| lit.split("").nth(1))
                            .collect()
                    })
                    .iter()
                    .any(|&first_sym| first_sym == symbol.content)
            }
        }
    };
}

pub(super) use {define_keywords, generate_impl};
