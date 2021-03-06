/*!
An interface to all manner of locale-related information.

This crate provides a locale-related codes/identifiers and any standards-based
information concerning them. For example, ISO-396 language identifiers, or
ISO-3166 country identifiers. These are under the module
[`locale_codes::codes`](codes/index.html). These modules are effectively
registries of standard code/identifiers and any metadate published as a part
of the associated standard(s). For example, _Codes for the representation of
currencies_, or ISO 4217 is based on a spreadsheet published directly by ISO
itself with some additional fields added from other publicly accessible sources.

While there is no formal type system or traits for modules exporting codes, there
are definitely some patterns all of the current implementations follow.

1. modules typically implement a `lookup()` function that returns an `Option`,
1. although where some standards have both alphabetic and numeric identifiers
   there are `lookup_by_alpha()` and `lookup_by_numeric()` instead, .
1. Most will also include a function `all_codes()` to retrieve a vector of all
   the known identifiers,
1. or, `all_alpha_codes()` and `all_numeric_codes()` as appropriate.

Some standards, specifically language and country, support 2-character and
3-character alphabetic identifiers, a single `lookup()` function is used to
lookup either.

## Example - Codes

The following example demonstrates some of the components of the crate, at
least some reasonable use cases.

1. Construct a _strict_ locale string where identifiers are checked against
   known standard codes where possible.
1. Lookup the ISO-3166 data for the country (in the
   [`CountryInfo`](codes/country/struct.CountryInfo.html) struct) identified
   by the ISO-3166, part 2, 3-character identifier.
1. The data fromn the last call contains one or more regions (in the
   [`RegionInfo`](/codes/region/struct.RegionInfo.html) struct), determine
   the countries name from the `country_code`.
1. Now we have the country name we can lookup the details of the currencies
   (in, the [`CurrencyInfo`](CurrencyInfo) struct).

```
use locale_codes::{country, currency, region};

let mexico = country::lookup("MEX").unwrap();
println!("{:?}", mexico);

let mexico_region = region::lookup(mexico.country_code).unwrap();
println!("{:?}", mexico_region);

let currencies = currency::currencies_for_country_name(mexico_region.name.as_str());
println!("{:?}", currencies);
```

## JSON Data Files

The script [`create-data-modules`](https://github.com/johnstonskj/locale-codes/blob/master/create-data-modules.sh)
on the other hand is used to process files downloaded, or scraped, from
standards web sites to create data used by the library. This data is generated
as JSON files in the `src/codes/data` folder and read as a part of the
build for `codes` modules using the Rust `include!` macro.

Currently data is generated for the following standards:

* ISO 639 _Codes for the representation of names of languages_; Parts 1-4,
  2-character and 3-character codes supported.
* ISO 3166 _Codes for the representation of names of countries and their
  subdivisions_; Part 1, 2-character codes, only.
* ISO 4217 _Codes for the representation of currencies_; alphabetic and
  numeric codes supported.
* ISO 15924 _Codes for the representation of names of scripts_; alphabetic
  and numeric codes supported.

Each folder under `src-data` represents a single standard, which may
generate one or more data sets. Each directory will contain a Python
script, `generate.py` which is called by the top-level script to create
the JSON in the correct location. Each should also contain a README
that includes attribution for any data retrieved to make this possible.
*/

#![warn(
    missing_debug_implementations,
    missing_docs,
    unused_extern_crates,
    rust_2018_idioms
)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

// ------------------------------------------------------------------------------------------------
// Public Modules
// ------------------------------------------------------------------------------------------------

pub mod codeset;

pub mod country;

pub mod currency;

pub mod language;

pub mod region;

pub mod script;
