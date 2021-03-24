# The emojic code generation utility 🚧🧰🔨

This folder contains an utility program to download and parse the latest Unicode
Emoji standard, and then generate lots the Rust code from it that is used in the
emojic crate.

Notice, this is not a dependency of emojic, but instead a pure utility meant for
contributors of the emojic crate.


## ⚙️ How it works

If executed via

```sh
cargo +nightly run
```

it will first download the current set of all [standardized emojis](https://unicode.org/Public/emoji/13.1/emoji-test.txt)
and parse it. Then it will combine all emojis which only differ in
attributes (such as skin tone, gender, and hair style).
And then feed this list of emojis and emoji variants to the
[Tera](https://crates.io/crates/tera) templates under
the [`templates`](./templates) folder and store the generated sources as
`alias.rs`, `flat.rs`, and `grouped.rs`. These files then can be copied into the
[`src`](../src) folder of the `emojic` crate where they are included from `lib.rs`.
