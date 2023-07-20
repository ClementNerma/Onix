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

impl From<CustomGraphQLError> for async_graphql::Error {
    fn from(val: CustomGraphQLError) -> Self {
        async_graphql::Error::new(val.0)
    }
}

#[macro_export]
macro_rules! graphql_enum {
    ($(#[$base:meta])* $(graphql_attr($(#[$outer:meta])*))? pub enum $name:ident { $($(#[$inner:meta])* $pat_name:ident $({ $($field_name:ident: $field_type: ty),* $(,)? })?),+ $(,)? }) => {
        ::paste::paste! { #[allow(non_snake_case)] mod [<_graphql_module_ $name>] {

        use ::std::borrow::Cow;

        use ::async_graphql::{
            SimpleObject, InputObject,
            Union, OneofObject,
            InputType, OutputType,

            Value,
            ContextSelectionSet,
            ServerResult,
            Positioned,
            InputValueResult,
            InputValueError,

            registry::Registry,
            parser::types::Field,
        };

        use ::serde::{Serialize, Deserialize};

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
            pub fn encode(self) -> [<$name GraphQL>] {
                match self {
                    $($name::$pat_name $({ $($field_name),+ })? =>
                        [<$name GraphQL>]::$pat_name($crate::graphql_enum!(@internal[args] $name $pat_name => $($($field_name),+)?))
                    ),+
                }
            }

            pub fn encode_cloned(&self) -> [<$name GraphQL>] {
                match self {
                    $($name::$pat_name $({ $($field_name),+ })? => ::paste::paste! {
                        [<$name GraphQL>]::$pat_name($crate::graphql_enum!(@internal[args_with_clone] $name $pat_name => $($($field_name),+)?))
                    }),+
                }
            }
        }

        #[derive(Union, OneofObject, Serialize, Debug, Clone)]
        #[graphql(input_name_suffix = "Input")]
        $($(#[$outer])*)?
        pub enum [<$name GraphQL>] {
            $($pat_name([<$name $pat_name GraphQL>])),+
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

        $(
            $crate::graphql_enum!(@internal[struct_decl] $name $pat_name => $($inner)+ => $($($field_name: $field_type),*)?);
        )+

        impl From<$name> for [<$name GraphQL>] {
            fn from(from: $name) -> Self {
                from.encode()
            }
        }

        impl From<[<$name GraphQL>]> for $name {
            fn from(from: [<$name GraphQL>]) -> Self {
                #[allow(unused_variables)]
                match from {
                    $([<$name GraphQL>]::$pat_name(extr) => $crate::graphql_enum!(@internal[args_for_into] $name $pat_name (extr) => $($($field_name),+)?)),+
                }
            }
        }

        #[::async_trait::async_trait]
        impl OutputType for $name {
            fn type_name() -> ::std::borrow::Cow<'static, str> {
                <[<$name GraphQL> ] as OutputType>::type_name()
            }

            fn create_type_info(registry: &mut Registry) -> String {
                <[<$name GraphQL> ] as OutputType>::create_type_info(registry)
            }

            async fn resolve(
                &self,
                ctx: &ContextSelectionSet<'_>,
                field: &Positioned<Field>,
            ) -> ServerResult<Value> {
                <[<$name GraphQL>] as OutputType>::resolve(&self.encode_cloned(), ctx, field).await
            }
        }

        impl InputType for $name {
            type RawValueType = Self;

            fn type_name() -> Cow<'static, str> {
                let base_name = <[<$name GraphQL>] as InputType>::type_name();
                format!("{base_name}Input").into()
            }

            fn create_type_info(registry: &mut Registry) -> String {
                <[<$name GraphQL>] as InputType>::create_type_info(registry)
            }

            fn parse(value: Option<async_graphql::Value>) -> InputValueResult<Self> {
                <[<$name GraphQL>] as InputType>::parse(value)
                    .map([<$name GraphQL>]::decode)
                    .map_err(InputValueError::propagate)
            }

            fn to_value(&self) -> async_graphql::Value {
                <[<$name GraphQL>] as InputType>::to_value(&self.encode_cloned())
            }

            fn as_raw_value(&self) -> Option<&Self::RawValueType> {
                Some(self)
            }
        }

        }

        pub use [<_graphql_module_ $name>]::*;

        }
    };

    (@internal[struct_decl] $name:ident $pat_name: ident => $($inner:meta)* => $($field_name:ident : $field_type:ty),+) => {
        ::paste::paste! {
            #[derive(SimpleObject, InputObject, Serialize, Debug, Clone)]
            #[graphql(input_name_suffix = "Input")]
            $(#[$inner])*
            pub struct [<$name $pat_name GraphQL>] {
                $($field_name: $field_type),+
            }
        }
    };

    (@internal[struct_decl] $name:ident $pat_name: ident => $($inner:meta)* =>) => {
        ::paste::paste! {
            #[derive(SimpleObject, InputObject, Serialize, Debug, Clone)]
            #[graphql(input_name_suffix = "Input")]
            $(#[$inner])*
            pub struct [<$name $pat_name GraphQL>] {
                #[graphql(name = "_empty")]
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
        if let Value::Object(map) = &value {
            if map.is_empty() {
                Ok(Self)
            } else {
                Err(InputValueError::custom("Void object must be empty"))
            }
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
