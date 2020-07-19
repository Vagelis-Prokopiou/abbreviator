# abbreviator
A Rust library for abbreviating long words.

This abbreviation is the following: The first letter + the length of the word minus the first and the last letter + the last letter.

Example:
let threshold_length = 10;
abbreviator.abbreviate("localization", threshold) == "l10n";
abbreviator.abbreviate("internationalization", threshold) == "i18n";
