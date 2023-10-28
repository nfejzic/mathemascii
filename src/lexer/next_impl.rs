macro_rules! next_impl {
    ($self: ident, no_prefix: $($infallible_fn:ident),*; prefix: $($fallible_fn:ident),*) => {{
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
        }
        )*

        $self.curr = curr;
        token
    }};
}

pub(super) use next_impl;
