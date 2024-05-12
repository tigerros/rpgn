macro_rules! concat_strings {
    () => {
        ::std::string::String::new()
    };

    ($($s:literal),+) => {
        ::core::concat!($($s),+)
    };

    ($($s:expr),+) => {{#[allow(clippy::arithmetic_side_effects)]{
        let mut buf = ::std::string::String::with_capacity(0$(+$s.len())+);
        $(buf.push_str($s);)+
        buf
    }}};
}

pub(crate) use concat_strings;
