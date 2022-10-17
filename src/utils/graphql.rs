use anyhow::Error;

pub type Result<T> = std::result::Result<T, String>;

pub fn format_err(err: Error) -> String {
    // TODO: to improve
    format!("{err:?}")
}

#[macro_export]
macro_rules! graphql_enum {
    ($(#[$outer:meta])* pub enum $name:ident { $($(#[$inner:meta])* $pat_name:ident { $($field_name:ident: $field_type: ty),* $(,)? }),+ $(,)? }) => {
        #[derive(Debug, Clone)]
        $(#[$outer])*
        pub enum $name {
            $(
                $pat_name {
                    $($field_name: $field_type),*
                }
            ),+
        }

        impl $name {
            pub fn graphqlify(self) -> ::paste::paste! { [<$name GraphQL>] } {
                match self {
                    $($name::$pat_name { $($field_name),+ } => ::paste::paste! { [<$name GraphQL>]::$pat_name(::paste::paste! { [<$name $pat_name GraphQL>] { $($field_name),+ } }) }),+
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

        $(::paste::paste! {
            #[derive(::async_graphql::SimpleObject, ::serde::Serialize, Debug, Clone)]
            $(#[$inner])*
            pub struct [<$name $pat_name GraphQL>] {
                $($field_name: $field_type),+
            }
        })+

        impl From<$name> for ::paste::paste! { [<$name GraphQL>] } {
            fn from(from: $name) -> Self {
                from.graphqlify()
            }
        }
    };

}
