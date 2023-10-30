macro_rules! next_impl {
    ($self: ident, no_prefix: $($infallible_fn:ident),*; prefix: $($fallible_fn:ident),* or $fallback_fn:ident) => {{
        let mut curr = $self.curr;
        let mut token = None;

        $(
        if let Some((num, cursor)) = $self.$infallible_fn() {
            // number is never prefix of anything
            $self.curr = cursor;
            return Some(num);
        }
        )*

        $(
        if let Some((found_token, cursor)) = $self.$fallible_fn(curr - $self.curr + 1) {
            token = Some(found_token);
            curr = cursor;

            // token can't contain a whitespace, so this token has the maximum length already
            if $self.src.get(cursor).map(|s| s.is_whitespace()).unwrap_or(false) {
                $self.curr = curr;
                return token;
            }
        }
        )*

        match token {
            Some(token) => {
                $self.curr = curr;
                Some(token)
            }
            None => {
                let (token, curr) = $self.$fallback_fn(curr - $self.curr + 1)?;
                $self.curr = curr;
                return Some(token);
            }
        }
    }};
}

pub(super) use next_impl;
