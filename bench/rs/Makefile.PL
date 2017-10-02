use inc::Module::Install;

name "XSBench-RS";
version "0.01";
abstract "Benchmarks for perl-xs";

configure_requires "Module::Install::Rust" => 0;
requires "XSLoader" => 0;

rust_use_perl_xs { path => "../.." };

#TODO: Change this to use the crate once published to crates.io
rust_requires "perl-sys" => { git=>"https://github.com/vickenty/perl-sys" };
rust_profile release => { lto => \"1" };
rust_write;

WriteAll;
