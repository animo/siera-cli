#[macro_export]
macro_rules! agent {
    ($cli:expr, $($args:tt)+) => {
        $cli.exec(&format!($($args)+).clone())
    }
}

#[macro_export]
macro_rules! test {
    ($name:ident $($cb:tt)+) => {
        #[tokio::test]
        async fn $name() {
            run_test($($cb)+).await
        }
    };
}
