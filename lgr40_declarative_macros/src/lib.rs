#[macro_export]
macro_rules! vec {
    // vec![1,2,3];
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push( $x );
            )*
            temp_vec
        }
    }
}
