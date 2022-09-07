#[macro_export]
macro_rules! redacted_debug {
    ($name:ident) => {
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, concat!(stringify!($name), "([redacted])"))
            }
        }
    };
}

/// borrowed newtype plumbing
#[macro_export]
macro_rules! borrowed_newtype {
    ($name:ident, $borrowed:ty) => {
        impl std::ops::Deref for $name {
            type Target = $borrowed;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<'a> Into<std::borrow::Cow<'a, $borrowed>> for &'a $name {
            fn into(self) -> std::borrow::Cow<'a, $borrowed> {
                std::borrow::Cow::Borrowed(&self.0)
            }
        }

        impl AsRef<$borrowed> for $name {
            fn as_ref(&self) -> &$borrowed {
                self
            }
        }
    };
}

#[macro_export]
macro_rules! newtype {
    ($name:ident, $owned:ty, $borrowed:ty) => {
        super::borrowed_newtype!($name, $borrowed);

        impl<'a> From<&'a $borrowed> for $name {
            fn from(value: &'a $borrowed) -> Self {
                Self(value.to_owned())
            }
        }

        impl From<$owned> for $name {
            fn from(value: $owned) -> Self {
                Self(value)
            }
        }

        impl<'a> From<&'a $owned> for $name {
            fn from(value: &'a $owned) -> Self {
                Self(value.to_owned())
            }
        }

        impl<'a> Into<$owned> for $name {
            fn into(self) -> $owned {
                self.0
            }
        }
    };
}

/// Macro for handling errors returned from the `reqwest` crate
///
/// The argument of this macro_invocation should be a `Result<T, reqwest::Error>`
#[macro_export]
macro_rules! unwrap_or_err {
    ($expression:expr) => {
        match $expression {
            Ok(t) => t,
            Err(e) => return Err(e),
        }
    };
    ($expression:expr, $error: expr) => {
        match $expression {
            Ok(t) => t,
            Err(_) => return Err($error),
        }
    };
}

#[macro_export]
macro_rules! return_some {
    ($expr:expr) => {
        match $expr {
            Some(x) => {
                return x
            }
            None => {
            }
        }
    };
}

///  some_or!(state.seg, return true);
#[macro_export]
macro_rules! some_or {($opt:expr, $els:stmt) => {
    if let Some(xxx) = $opt { xxx } else { $els }
}}

#[macro_export]
macro_rules! ok_or {($res:expr, $els:stmt) => {
    match $res {
        Ok(xxx) => xxx,
        Err(_) => { $els },
    }
}}

/// Macro for handling errors returned from the `reqwest` crate
///
/// The argument of this macro_invocation should be a `Result<T, reqwest::Error>`
#[macro_export]
macro_rules! process_opts_or {
    ($expression:expr, $func:expr, $fallback:expr) => {
        match $expression {
            Some(t) => $func(t),
            None => $fallback,
        }
    }
}

