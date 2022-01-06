//! # Adding thousands-seperators to numbers
//! 
//! ### First of all add teh following
//! ```
//! [dependencies]
//! numsep = "0.1.1"
//! ```
//! ### to your Cargo.toml-File.
//! 
//! # Examples:
//! 
//! ```
//! use numsep::*;
//! 
//! let number = 10000;
//! 
//! assert_eq!("10,000", separate(n, Locale::English));
//! 
//! ```
//! 
//! ## The `Locale`- enum provides the following country-presets:
//! 
//! ``` 
//!     Locale::English,
//!     Locale::German,
//!     Locale::Canadian,
//!     Locale::Swiss,
//!     Locale::Swiss2,
//!     Locale::Singapore,
//! ```
//! 
//! ## AND a CUSTOM-Scheme...
//! 
//! ```
//!     Locale::Custom(Scheme)
//! ```
//! ## ... that can be used like that:
//! 
//! ```
//! use numsep::*;
//! 
//! let custom = custom()
//!      .set_separator("'")
//!      .set_radix(",");
//! 
//! let n = 2000.5;
//!  
//! assert_eq!("2'000,5", separate(n, Locale::Custom(custom)));
//! ```

use slicestring::Slice;

/// public enum to set the separation-style using the cconcerning language.
/// There could also be set a custom-style using the functions [`custom`], [`set_separator`] and [`set_radix`].
/// [`set_separator`]: struct.Scheme.html#method.set_separator
/// [`set_radix`]: struct.Scheme.html#method.set_radix
/// [`custom`]: fn.custom.html#
pub enum Locale<Scheme> {
    English,
    German,
    Canadian,
    Swiss,
    Swiss2,
    Singapore,
    Custom(Scheme),
}

pub struct Scheme<'a> {
    pub (in crate) separator: &'a str,
    pub (in crate) radix: &'a str,
}

/// creates a custom-[`Scheme`] that can be customized with [`set_separator`] and [`set_radix`]
/// [`Scheme`]: struct.Scheme.html#
/// [`set_separator`]: struct.Scheme.html#method.set_separator
/// [`set_radix`]: struct.Scheme.html#method.set_radix
pub fn custom() -> Scheme<'static> {
    let s = Scheme{separator: "", radix: ""};
    s
}

impl <'a>Scheme<'a> {
    /// sets a custom thousands-separator
    pub fn set_separator(self, sep: &'a str) -> Scheme {
        let new = Scheme{separator: sep, radix: self.radix};
        new
    }

    /// sets a custom radix character to separate the integer part of the value from its fractional part
    pub fn set_radix(self, com: &'a str) -> Scheme {
        let new = Scheme{separator: self.separator, radix: com};
        new
    }
}

/// This function adds thousands-separators to the concerning number. It takes the number as the first argument, and the country-style or customized-style as the second argument that's type is [`Locale`].
/// # Example:
/// ```
/// use numsep::*;
/// 
/// let n = separate(2000, Locale::English);
/// 
/// assert_eq!("2.000", n);
/// ```
/// # Example for Customized-Style:
/// ```
/// use numsep::*;
/// 
/// let custom = custom()
///     .set_separator("'")
///     .set_radix(",");
/// 
/// let n = 2000.5;
///  
/// assert_eq!("2'000,5", separate(n, Locale::Custom(custom)));
/// ```
/// [`Locale`]: enum.Locale.html#
pub fn separate<T: ToString>(n: T, lang: Locale<Scheme>) -> String {

    let s = n.to_string();
    let (sep, radix) = get_separator(&lang);


    //First seperate dec-digits
    let mut s2 = String::new();
    let mut s1 = s.slice(0, match s.find(".") {
        Some(index) => {
            s2 = s.slice(index+1, s.len());
            index
        },
        None => match s.find(",") {
            Some(index) => {
                s2 = s.slice(index+1, s.len());
                index
            },
            None => s.len()
        }
    });

    //adding thousands-separators

    let mut i = 0;
    while i <= s1.len() {


        let a = s1.slice(0, s1.len()-i);
        let b = s1.slice(s1.len()-i, s1.len());
        s1 = format!("{}{}{}", a, sep, b);

        i=i+4;
    }

    //clear from unnecessesary separators
    if &s1[0..1] == sep {
        s1 = s1[1..s1.len()].to_string();
    }

    if &s1[s1.len()-1..s1.len()] == sep {
        s1 = s1[0..s1.len()-1].to_string();
    }

    //return number with thousands-separators, cheking if there were some digits behind the radix
    format!("{}{}{}", s1, 
    if s2 != "" {
        radix
    } else {
        ""
    }, s2)
}

fn get_separator<'a>(lang: &'a Locale<Scheme>) -> (&'a str, &'a str) {

    match lang {
        Locale::English => (",", "."),
        Locale::German => (".", ","),
        Locale::Canadian => (" ", "."),
        Locale::Swiss => ("'", "."),
        Locale::Swiss2 => ("'", ","),
        Locale::Singapore => (",", "·"),
        Locale::Custom(s) => (s.separator, s.radix),

    }
}
