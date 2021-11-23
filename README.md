# Reproducing [DER crate](https://github.com/RustCrypto/formats/) compilation failure
See this repo https://github.com/mishazharov/der-repro for a small test case to show the error

## Versions

```bash
$ rustc --version
rustc 1.56.0 (09c42c458 2021-10-18)
```

```bash
$ cargo --version
cargo 1.56.0 (4ed5d137b 2021-10-04)
```

## Platform
Kernel: 5.11.0-38-generic
Distro: #42~20.04.1-Ubuntu
Arch: x86_64

## Steps to reproduce

Run `cargo build --lib --release --target wasm32-unknown-unknown`.

## Error shown
```
error[E0277]: can't compare `usize` with `()`
   --> /home/misha/.cargo/registry/src/github.com-1ecc6299db9ec823/der-0.4.4/src/encoder.rs:151:43
    |
151 |         if nested_encoder.finish()?.len() == length.try_into()? {
    |                                           ^^ no implementation for `usize == ()`
    |
    = help: the trait `PartialEq<()>` is not implemented for `usize`

error[E0277]: the trait bound `(): From<Length>` is not satisfied
   --> /home/misha/.cargo/registry/src/github.com-1ecc6299db9ec823/der-0.4.4/src/encoder.rs:151:53
    |
151 |         if nested_encoder.finish()?.len() == length.try_into()? {
    |                                                     ^^^^^^^^ the trait `From<Length>` is not implemented for `()`
    |
    = note: required because of the requirements on the impl of `Into<()>` for `Length`
note: required because of the requirements on the impl of `TryFrom<Length>` for `()`
   --> /home/misha/.cargo/registry/src/github.com-1ecc6299db9ec823/der-0.4.4/src/asn1/null.rs:46:6
    |
46  | impl TryFrom<Any<'_>> for () {
    |      ^^^^^^^^^^^^^^^^     ^^
    = note: required because of the requirements on the impl of `TryInto<()>` for `Length`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `der` due to 2 previous errors
```

## Expected behaviour

When running `cargo build --lib --release` this error does not manifest itself, so it would be nice if it
also didn't show up on the wasm target.
