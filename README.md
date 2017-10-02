[![Build Status](https://travis-ci.org/vickenty/perl-xs.svg?branch=master)](https://travis-ci.org/vickenty/perl-xs)

# Perl XS for Rust

High-level Rust bindings to Perl XS API.

## Example

```rust
xs! {
  package Array::Sum;
  sub sum_array(ctx, array: AV) {
    array.iter().map(|it| it.unwrap_or(0.0)).sum::<NV>()
  }
}
```

For a more complete example see the XSTest package in the `t/` directory.

## Goals

- safety
- correctness
- speed

Perl XS API is deliberately low-level and requires user to maintain a
good deal of internal invariants, thus allowing for very fast
code. This package takes a different approach of encapsulating
implementation details to provide a simpler and safer API at the cost
of speed.

For now, this library focuses on Perl's public documented API, which
is a small subset of what is available to authors of modules written
in C.

## Work in progress

This project is very much in progress. It is not yet clear if project
goals are attainable at all or if the API will make any sense in
practice.

## How to use

`Module::Install::Rust` integrates traditional Perl build system with
Cargo, allowing Rust code to be compiled and installed using familiar
`perl Makefile.PL && make` process. For example, see test package
under `t` directory.

## Prerequisites

- Perl 5.20+ (for 64-bit array methods)
- Rust 1.11+ (`catch_unwind`, `cdylib`)

## Testing

To install packages required for testing and benchmarking:

```shell
cpanm --installdeps .
```

To run tests:

```shell
(cd t && perl Makefile.PL && make test)
```
