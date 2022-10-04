#[macro_export]
/// Checks whether a struct of Option<T> has any `Some` Value
///
/// { foo: Some(bar), bar: None } // true
/// { foo: None,      bar: None } // false
macro_rules! has_any_value_in_struct {
    ($options:expr, $($field:ident),*) => {
        {
            let mut query = Vec::new();
            $(
                query.push($options.$field);
            )*
            query.iter().any(|v| v.is_some())
        }
    };
}
