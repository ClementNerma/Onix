use anyhow::Error;
use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, String>;

pub fn format_err(err: Error) -> String {
    // TODO: to improve
    format!("{err:?}")
}

#[macro_export]
macro_rules! graphql_enum {
    ($(#[$outer:meta])* pub enum $name:ident { $($(#[$inner:meta])* $pat_name:ident $({ $($field_name:ident: $field_type: ty),* $(,)? })?),+ $(,)? }) => {
        #[derive(Debug, Clone)]
        $(#[$outer])*
        pub enum $name {
            $(
                $pat_name $({
                    $($field_name: $field_type),*
                })?
            ),+
        }

        impl $name {
            pub fn graphqlify(self) -> ::paste::paste! { [<$name GraphQL>] } {
                match self {
                    $($name::$pat_name $({ $($field_name),+ })? => ::paste::paste! {
                        [<$name GraphQL>]::$pat_name($crate::graphql_enum!(@internal[args] $name $pat_name => $($($field_name),+)?))
                    }),+
                }
            }
        }

        ::paste::paste! {
            #[derive(::async_graphql::Union, ::serde::Serialize, Debug, Clone)]
            $(#[$outer])*
            pub enum [<$name GraphQL>] {
                $($pat_name(::paste::paste! { [<$name $pat_name GraphQL>] })),+
            }
        }

        $(
            $crate::graphql_enum!(@internal[struct_decl] $name $pat_name => $($inner)+ => $($($field_name: $field_type),*)?);
        )+

        impl From<$name> for ::paste::paste! { [<$name GraphQL>] } {
            fn from(from: $name) -> Self {
                from.graphqlify()
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
    }
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
