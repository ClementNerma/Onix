#[macro_export]
macro_rules! graphqlify {
    (pub enum $name:ident { $($pat_name:ident { $($field_name:ident: $field_type: ty,)* },)+ }) => {
        #[derive(Debug, Clone)]
        pub enum $name {
            $($pat_name { $($field_name: $field_type),* }),+
        }

        impl $name {
            pub fn graphqlify(self) -> paste! { [<$name GraphQL>] } {
                match self {
                    $($name::$pat_name { $($field_name),+ } => paste! { [<$name GraphQL>]::$pat_name(paste! { [<$name $pat_name GraphQL>] { $($field_name),+ } }) }),+
                }
            }
        }

        paste! {
            #[derive(::async_graphql::Union, Debug, Clone)]
            pub enum [<$name GraphQL>] {
                $($pat_name(paste! { [<$name $pat_name GraphQL>] })),+
            }
        }

        $(paste! {
            #[derive(::async_graphql::SimpleObject, Debug, Clone)]
            pub struct [<$name $pat_name GraphQL>] {
                $($field_name: $field_type),+
            }
        })+

        impl From<$name> for paste! { [<$name GraphQL>] } {
            fn from(from: $name) -> Self {
                from.graphqlify()
            }
        }
    };
}
