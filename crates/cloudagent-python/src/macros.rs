#[macro_export]
/// Macro to fill a vector<(str,str)> with a structure where fields can be optional
///
/// ```rust
/// struct Foo<'a> {
///     a: Option<'a str>
///     b: Option<'a str>
/// }
///
/// let foo = Foo {a: 'baz', b: 'bar'}
///
/// ```
///
/// will convert to
///
/// ```rust
/// let query = fill_query!(foo, a, b)
/// println!("{:?}", query); // [("a", "baz"), ("b", "bar")]
/// ```
macro_rules! fill_query {
    ($options:expr, $($field:ident),*) => {
        {
            let mut query = Vec::new();
            $(
                $options.$field.as_ref().map(|c| query.push((stringify!($field), c.to_owned())));
            )*
            query
        }
    };
}
