# SVG avatars

A Rust library for generating SVG avatars from identifiers.

## Example

```rust
use svg_avatars::{Rings, SvgAvatarBuilder};

fn main() {
    let svg = SvgAvatarBuilder::new()
        .identifier("foo")
        .rings(Rings::Three)
        .stroke_color("white")
        .build();

    svg.save("bar.svg").unwrap();
}
```

This produces the `bar.svg` file with the following content:

<!-- markdownlint-disable no-inline-html -->
<img src="./src/foo.svg" alt="three rings example"
    width="180px"
    height="auto" />
<!-- markdownlint-enable no-inline-html -->

## License

MIT; see the [LICENSE.txt](./LICENSE.txt) file.
