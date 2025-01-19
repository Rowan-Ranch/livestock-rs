# livestock-rs

![crates.io](https://img.shields.io/crates/v/livestock-rs.svg)

A comprehensive library for managing, identifying, and working with livestock breeds. Designed to support farming and ranching applications, breed registries, and livestock data processing.

Features
 - 🌱 Support for over 500+ livestock breeds, categorized by species.
 - 🐄 Includes common cattle breeds like Angus and Brahman.
 - 🐐 Includes common goat breeds like Alpine and Boer.
 - 🐑 Includes common sheep breeds like Dorper and St. Croix.
 - 🫏 Includes common donkey breeds like Miniature and Standard.
 - 🦌 Includes common reindeer breeds like Even and Nentsi.
 - 🛠️ Utilities for converting between enum variants and human-readable strings.
 - 🔒 Serde support for serialization and deserialization.

## Getting Started
Add the crate to your `Cargo.toml`:

```
[dependencies]
livestock_rs = "0.1.0"
```

or 

```
cargo add livestock-rs
```

## Usage Example
``` rust
use livestock_rs::GoatBreed;

let breed = GoatBreed::Caninde;
println!("{:?}", breed); // prints "Canindé"
```

## Cattle Breeds
 - Covers all major cattle breeds like Angus, Brahman, Jersey, and more.
 - Regional specialties, including Texas Longhorn, Deoni, and more.

## Goat Breeds
 - Covers all major goat breeds like Boer, Alpine, Nigerian Dwarf, and more.
 - Regional specialties, including Kalahari Red, Damascus, and Golden Guernsey.

## Sheep Breeds
- Features a wide variety of sheep breeds including popular types like Dorper, Romney, and Merino.
- Regional and specialty breeds such as the Icelandic, Navajo Churro, and Valais Blacknose.

## Donkey Breeds
- Features common donkey breeds including Standard, Large Standard, Miniature, Mary, Mammoth Jack Stock, and more.

## Reindeer Breeds
 - Features common reindeer breeds including Chukotka, Even, Evenk, and Nentsi.

## Roadmap
 - 🗂️ Expand support for other livestock species (e.g., chickens, ducks, pigs).
 - 🌍 Localization support for breed names in multiple languages.
 - 📊 Add more utilities for livestock data management.

## Contributing
We welcome contributions! If you’d like to add more breeds or improve the library, please follow these steps:
1.	Fork the repository.
2.	Create a new branch for your feature: git checkout -b feature-name.
3.	Commit your changes: git commit -m "Add feature-name".
4.	Push your branch and create a pull request.

## License
This project is licensed under the MIT License.

## About Us
This crate is maintained by Rowan Ranch Pasture & Pen, LLC, a small-scale regenerative family farm focusing on expanding agricultural technology solutions. We strive to build software that empowers farmers, researchers, and agricultural businesses.

## Acknowledgments
 - Thanks to the global farming community for inspiration.
 - Inspired by real-world agricultural needs and breed registries.
