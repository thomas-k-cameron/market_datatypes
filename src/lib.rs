macro_rules! mod_and_use {
    ($($i: ident;)+) => {
        $(
            mod $i;
           pub use $i::*;
        ) +
    };
}

mod_and_use! {
    signal;
    side;
    price_qty;
    time_in_force;
}