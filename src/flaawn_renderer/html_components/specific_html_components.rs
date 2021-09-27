#[macro_export]
macro_rules! div {
    (( $($on:ident = $ov:expr,)* ), $($child:expr,)*) => {
        GenericHTMLTag!("div", ( $($on = $ov,)* ), $($child,)*)
    };
}

#[macro_export]
macro_rules! p {
    (( $($on:ident = $ov:expr,)* ), $($child:expr,)*) => {
        GenericHTMLTag!("p", ( $($on = $ov,)* ), $($child,)*)
    };
}

#[macro_export]
macro_rules! ul {
    (( $($on:ident = $ov:expr,)* ), $($child:expr,)*) => {
        GenericHTMLTag!("ul", ( $($on = $ov,)* ), $($child,)*)
    };
}

#[macro_export]
macro_rules! ol {
    (( $($on:ident = $ov:expr,)* ), $($child:expr,)*) => {
        GenericHTMLTag!("ol", ( $($on = $ov,)* ), $($child,)*)
    };
}

#[macro_export]
macro_rules! img {
    (( $($on:ident = $ov:expr,)* ), $($child:expr,)*) => {
        GenericHTMLTag!("img", ( $($on = $ov,)* ), $($child,)*)
    };
}
