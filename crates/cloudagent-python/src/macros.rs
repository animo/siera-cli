#[macro_export]
/// LALA
macro_rules! fill_query {
    ($options:expr, $query:expr, $($field:ident),*) => {
        {
            let mut query = Vec::new();
            $(
                $options.$field.as_ref().map(|c| query.push((stringify!($field), c.to_string())));
            )*
            query
        }
    };
}
