/// # `hiccup!`: 
/// * The main objective of this lib is to prevent unclosed html tags.
/// This macro is inspired by Clojures [hiccup](https://github.com/weavejester/hiccup)
/// 
/// ## Basic usage: 
/// 
/// The macro `hiccup! receives a mutable string as the first argument and mutates the string to emit the HTML.
/// The order of the elemnts is: 
/// 1. `tag` as the first element.
/// 2. Optional attribute inside the tag should follow the tag name as `{attribute1=>"value1 vlaue2 ... valuen", attr=>"value"}`. Also, the attributes should be inside `{...}` and separate each key value pair by `,`.
/// The element should be written as `key=>"value"`, where key is a symbol, followed by an arrow (`=>`), and then the value as a string `"value"`.
/// 3. After (Optional) the tag name or the attributes `{...}` tou should include `[...]` that can have other tags, such as `p["text"]` or regular string values.
/// 
/// ### Differences between Clojure and Rust Hiccup: 
/// * [Clojure](https://github.com/weavejester/hiccup/wiki/Syntax): `[:a {:href "http://github.com"} "GitHub"]`
/// * Rust: `a{href=>"http://github.com"}["GitHub"]`
/// 
/// ## Example
/// ```rust
/// extern crate hiccup;
///
/// use hiccup::hiccup;
///
/// fn main() {
///     let mut html = String::new();
///
///     let _ = hiccup!(&mut html,
///         html[
///             head[meta{name=>"author", content=>"Julia Naomi"}
///                 title["Hiccup guide"]]
///             body{class=>"amazing hiccup guide"}[
///                 h1{font=>"bold", color=>"red"}["Hiccup is the best!"]
///                 p["please lookup clojure's hiccup for better ideas on this macro"]]
///         ]);
///
///     assert_eq!(html,"<html><head><meta name=\"author\" content=\"Julia Naomi\"/>\
///     <title>Hiccup guide</title></head><body class=\"amazing hiccup guide\">\
///     <h1 font=\"bold\" color=\"red\">Hiccup is the best!</h1>\
///     <p>please lookup clojure\'s hiccup for better ideas on this macro</p></body></html>");
/// }
/// ```
/// 
#[macro_export]
macro_rules! hiccup {
    ($w:expr, ) => (());

    ($w:expr, $e:tt) => {{
        use std::fmt::Write;
        let _ = write!($w, "{}", $e);
    }};

    ($w:expr, $tag:ident {$($key:expr => $value:expr),*}[$($inner:tt)*] $($rest:tt)*) => {{
        use std::fmt::Write;
        
        let _ = write!($w, "<{}", stringify!($tag));
        $(
            let _ = write!($w, " {}=", stringify!($key));
            let _ = write!($w, "{}", stringify!($value));
        )*
        let _ = write!($w, ">");

        hiccup!($w, $($inner)*);
        let _ = write!($w, "</{}>", stringify!($tag));
        hiccup!($w, $($rest)*);
    }};

    ($w:expr, $tag:ident {$($key:expr => $value:expr),*} $($rest:tt)*) => {{
        use std::fmt::Write;
        
        let _ = write!($w, "<{}", stringify!($tag));
        $(
            let _ = write!($w, " {}=", stringify!($key));
            let _ = write!($w, "{}", stringify!($value));
        )*
        let _ = write!($w, "/>");
        hiccup!($w, $($rest)*);
    }};

    ($w:expr, $tag:ident [$($inner:tt)*] $($rest:tt)*) => {{
        use std::fmt::Write;
        
        let _ = write!($w, "<{}>", stringify!($tag));
        hiccup!($w, $($inner)*);
        let _ = write!($w, "</{}>", stringify!($tag));
        hiccup!($w, $($rest)*);
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_html() {
        let mut out = String::new();

        let _ = hiccup!(&mut out,
            html[
                head[title["Hiccup guide"]]
                body[h1["Hiccup is the best!"]]
            ]);

            assert_eq!(out, "<html><head><title>Hiccup guide</title></head>\
            <body><h1>Hiccup is the best!</h1></body></html>");
    }

    #[test]
    fn attr_block() {
        let mut out = String::new();

        let _ = hiccup!(&mut out,
            html[
                head[title["Hiccup guide"]]
                body[h1{class=>"value", c=>"v"}["Hiccup is the best!"]]
            ]);

        assert_eq!(out, "<html><head><title>Hiccup guide</title></head><body>\
        <h1 class=\"value\" c=\"v\">Hiccup is the best!</h1></body></html>");
    }
}
