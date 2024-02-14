# SVG avatars

[![master](https://github.com/nathiss/svg_avatars/actions/workflows/master.yaml/badge.svg)](https://github.com/nathiss/svg_avatars/actions/workflows/master.yaml)
[![Crates.io](https://img.shields.io/crates/v/svg_avatars)](https://crates.io/crates/svg_avatars)
[![docs.rs](https://docs.rs/svg_avatars/badge.svg)](https://docs.rs/svg_avatars/)
![Crates.io](https://img.shields.io/crates/l/svg_avatars)

A Rust library for generating SVG avatars from identifiers.

## Example

```rust
use svg_avatars::{Rings, SvgAvatarBuilder};

fn main() {
    let svg = SvgAvatarBuilder::new()
        .identifier("foo")
        .rings(Rings::Three)
        .stroke_color("black")
        .build();

    svg.save("bar.svg").unwrap();
}
```

This produces the `bar.svg` file with the following content:

<!-- markdownlint-disable no-inline-html -->
<img src="https://raw.githubusercontent.com/nathiss/svg_avatars/master/src/foo.svg"
    alt="three rings example"
    width="180px"
    height="auto" />
<!-- markdownlint-enable no-inline-html -->

## License

MIT; see the [LICENSE.txt](./LICENSE.txt) file.
