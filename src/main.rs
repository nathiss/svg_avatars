use svg_avatars::{Rings, SvgAvatarBuilder};

fn main() {
    SvgAvatarBuilder::new()
        .identifier("foo")
        .rings(Rings::Three)
        .build()
        .save("foo.svg")
        .unwrap();
}
