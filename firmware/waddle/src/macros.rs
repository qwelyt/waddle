#[macro_export]
macro_rules! count_tts {
    () => { 0usize };
    ($_head:tt) => {1usize};
    ($_head:tt, $($tail:tt),*) => {1usize + count_tts!($($tail),*)};
}

#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* $(,)?) => {{
        const C: usize = count_tts!($($x),*);
        let mut temp_vec: heapless::Vec<_, C> = heapless::Vec::new();
        $(
            let _ = temp_vec.push($x);
        )*
        temp_vec
    }}
}