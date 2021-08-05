#[macro_export]
macro_rules! hashmap {
    (,) => {
        $compile_error!("Single comma not allowed")
    };
    ($($key: expr => $value: expr),*$(,)?) => {
        {
            use ::std::collections::HashMap;
            let mut hm = HashMap::new();
            $(hm.insert($key, $value);)*
            hm
        }
    }
}
