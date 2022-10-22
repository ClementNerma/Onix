use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use serde::{Deserialize, Serialize};

pub struct CustomGraphQLError(pub String);

pub type Result<T, E = CustomGraphQLError> = std::result::Result<T, E>;

impl From<&str> for CustomGraphQLError {
    fn from(str: &str) -> Self {
        Self(str.to_string())
    }
}

impl From<String> for CustomGraphQLError {
    fn from(str: String) -> Self {
        Self(str)
    }
}

impl From<anyhow::Error> for CustomGraphQLError {
    fn from(err: anyhow::Error) -> Self {
        // TODO: to improve
        Self(format!("{err:?}"))
    }
}

impl Into<async_graphql::Error> for CustomGraphQLError {
    fn into(self) -> async_graphql::Error {
        async_graphql::Error::new(self.0)
    }
}

#[macro_export]
macro_rules! graphql_enum {
    ($(#[$base:meta])* $(graphql_attr($(#[$outer:meta])*))? pub enum $name:ident { $($(#[$inner:meta])* $pat_name:ident $({ $($field_name:ident: $field_type: ty),* $(,)? })?),+ $(,)? }) => {
        #[derive(Debug, Clone)]
        $(#[$base])*
        pub enum $name {
            $(
                $pat_name $({
                    $($field_name: $field_type),*
                })?
            ),+
        }

        impl $name {
            pub fn encode(self) -> ::paste::paste! { [<$name GraphQL>] } {
                match self {
                    $($name::$pat_name $({ $($field_name),+ })? => ::paste::paste! {
                        [<$name GraphQL>]::$pat_name($crate::graphql_enum!(@internal[args] $name $pat_name => $($($field_name),+)?))
                    }),+
                }
            }

            pub fn encode_cloned(&self) -> ::paste::paste! { [<$name GraphQL>] } {
                match self {
                    $($name::$pat_name $({ $($field_name),+ })? => ::paste::paste! {
                        [<$name GraphQL>]::$pat_name($crate::graphql_enum!(@internal[args_with_clone] $name $pat_name => $($($field_name),+)?))
                    }),+
                }
            }
        }

        ::paste::paste! {
            #[derive(::async_graphql::Union, ::serde::Serialize, Debug, Clone)]
            $($(#[$outer])*)?
            pub enum [<$name GraphQL>] {
                $($pat_name(::paste::paste! { [<$name $pat_name GraphQL>] })),+
            }

            impl [<$name GraphQL>] {
                #[allow(dead_code)]
                pub fn decode(self) -> $name {
                    self.into()
                }

                #[allow(dead_code)]
                pub fn decode_cloned(&self) -> $name {
                    self.clone().into()
                }
            }
        }

        $(
            $crate::graphql_enum!(@internal[struct_decl] $name $pat_name => $($inner)+ => $($($field_name: $field_type),*)?);
        )+

        impl From<$name> for ::paste::paste! { [<$name GraphQL>] } {
            fn from(from: $name) -> Self {
                from.encode()
            }
        }

        impl Into<$name> for ::paste::paste! { [<$name GraphQL>] } {
            fn into(self) -> $name {
                #[allow(unused_variables)]
                match self {
                    $(Self::$pat_name(extr) => $crate::graphql_enum!(@internal[args_for_into] $name $pat_name (extr) => $($($field_name),+)?)),+
                }
            }
        }
    };

    (@internal[struct_decl] $name:ident $pat_name: ident => $($inner:meta)* => $($field_name:ident : $field_type:ty),+) => {
        ::paste::paste! {
            #[derive(::async_graphql::SimpleObject, ::serde::Serialize, Debug, Clone)]
            $(#[$inner])*
            pub struct [<$name $pat_name GraphQL>] {
                $($field_name: $field_type),+
            }
        }
    };

    (@internal[struct_decl] $name:ident $pat_name: ident => $($inner:meta)* =>) => {
        ::paste::paste! {
            #[derive(::async_graphql::SimpleObject, ::serde::Serialize, Debug, Clone)]
            $(#[$inner])*
            pub struct [<$name $pat_name GraphQL>] {
                __empty: $crate::utils::graphql::Void
            }
        }
    };

    (@internal[args] $name:ident $pat_name: ident => $($field_name:ident),+) => {
        ::paste::paste! { [<$name $pat_name GraphQL>] { $($field_name),+ } }
    };

    (@internal[args] $name:ident $pat_name: ident =>) => {
        ::paste::paste! { [<$name $pat_name GraphQL>] { __empty: $crate::utils::graphql::Void } }
    };

    (@internal[args_with_clone] $name:ident $pat_name: ident => $($field_name:ident),+) => {
        ::paste::paste! { [<$name $pat_name GraphQL>] { $($field_name: $field_name.clone()),+ } }
    };

    (@internal[args_with_clone] $name:ident $pat_name: ident =>) => {
        ::paste::paste! { [<$name $pat_name GraphQL>] { __empty: $crate::utils::graphql::Void } }
    };

    (@internal[args_for_into] $name:ident $pat_name: ident ($extr: ident) => $($field_name:ident),+) => {
        $name::$pat_name {
            $($field_name: $extr.$field_name),+
        }
    };

    (@internal[args_for_into] $name:ident $pat_name: ident ($extr: ident) =>) => {
        $name::$pat_name
    };
}

#[macro_export]
macro_rules! declare_id_type {
    ($typename: ident) => {
        #[derive(Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
        pub struct $typename(pub u64);

        impl $typename {
            #[allow(dead_code)]
            pub fn encode(&self) -> String {
                ::base62::encode(self.0)
            }

            #[allow(dead_code)]
            pub fn decode(input: &str) -> ::anyhow::Result<Self> {
                let id = ::base62::decode(input)
                    .map_err(|err| ::anyhow::anyhow!("Failed to parse base 62 content: {err}"))?;

                let id = u64::try_from(id)
                    .map_err(|_| ::anyhow::anyhow!("Number is too big to find into 64 bits"))?;

                Ok(Self(id))
            }
        }

        impl ::std::fmt::Display for $typename {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", self.encode())
            }
        }

        impl ::std::fmt::Debug for $typename {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{} ({})", self.0, self.encode())
            }
        }

        #[::async_graphql::Scalar(name = "String")]
        impl ::async_graphql::ScalarType for $typename {
            fn parse(value: ::async_graphql::Value) -> ::async_graphql::InputValueResult<Self> {
                if let ::async_graphql::Value::String(value) = value {
                    Ok(Self::decode(&value)?)
                } else {
                    Err(::async_graphql::InputValueError::expected_type(value))
                }
            }

            fn to_value(&self) -> ::async_graphql::Value {
                ::async_graphql::Value::String(self.encode())
            }
        }
    };
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Hash, Serialize, Deserialize)]
pub struct Void;

#[Scalar]
impl ScalarType for Void {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::Null = &value {
            Ok(Self)
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::Null
    }
}

impl From<()> for Void {
    fn from(_: ()) -> Self {
        Self
    }
}
