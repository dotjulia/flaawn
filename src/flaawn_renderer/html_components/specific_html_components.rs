#[macro_export]
macro_rules! div {
    (( $($on:ident = $ov:expr,)* ), $($child:expr,)*) => {
        flaawn::GenericHTMLTag!("div", ( $($on = $ov,)* ), $($child,)*)
    };
}

#[macro_export]
macro_rules! p {
    (( $($on:ident = $ov:expr,)* ), $($child:expr,)*) => {
        flaawn::GenericHTMLTag!("p", ( $($on = $ov,)* ), $($child,)*)
    };
}

#[macro_export]
macro_rules! ul {
    (( $($on:ident = $ov:expr,)* ), $($child:expr,)*) => {
        flaawn::GenericHTMLTag!("ul", ( $($on = $ov,)* ), $($child,)*)
    };
}

#[macro_export]
macro_rules! li {
    (( $($on:ident = $ov:expr,)* ), $($child:expr,)*) => {
        flaawn::GenericHTMLTag!("li", ( $($on = $ov,)* ), $($child,)*)
    };
}

#[macro_export]
macro_rules! ol {
    (( $($on:ident = $ov:expr,)* ), $($child:expr,)*) => {
        flaawn::GenericHTMLTag!("ol", ( $($on = $ov,)* ), $($child,)*)
    };
}

#[macro_export]
macro_rules! img {
    (( $($on:ident = $ov:expr,)* ), $($child:expr,)*) => {
        flaawn::GenericHTMLTag!("img", ( $($on = $ov,)* ), $($child,)*)
    };
}
