#[macro_export]
macro_rules! parse {
    ($fn:path, $( $input:expr ),+ $(,)?) => {

        vec![$($fn($input)),+]
    };
}
