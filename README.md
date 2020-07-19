# abbreviator

A Rust library for abbreviating long words.


## Example usage
```rust
let threshold_length = 10;
abbreviator.abbreviate("localization", threshold_length) == "l10n";
abbreviator.abbreviate("internationalization", threshold_length) == "i18n";

let threshold_length = 50;
abbreviator.abbreviate("localization", threshold_length) == "localization";
abbreviator.abbreviate("internationalization", threshold_length) == "internationalization";
```

