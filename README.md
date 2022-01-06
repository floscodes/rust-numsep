# Adding thousands-seperators to numbers

### First of all add teh following
```
[dependencies]
numsep = "0.1.1"
```
### to your Cargo.toml-File.

# Examples:

```
use numsep::*;

let number = 10000;

assert_eq!("10,000", separate(n, Locale::English));

```

## The `Locale`- enum provides the following country-presets:

``` 
    Locale::English,
    Locale::German,
    Locale::Canadian,
    Locale::Swiss,
    Locale::Swiss2,
    Locale::Singapore,
```

## AND a CUSTOM-Scheme...

```
    Locale::Custom(Scheme)
```
## ... that can be used like that:

```
use numsep::*;

let custom = custom()
     .set_separator("'")
     .set_radix(",");

let n = 2000.5;
 
assert_eq!("2'000,5", separate(n, Locale::Custom(custom)));
```