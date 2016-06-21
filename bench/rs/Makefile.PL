use inc::Module::Install;

name "XSBench-RS";
version "0.01";
abstract "Benchmarks for perl-xs";

configure_requires "Module::Install::Rust" => 0;
requires "XSLoader" => 0;

rust_use_perl_xs { path => "../.." };
rust_requires "perl-sys" => { path => "../../../perl-sys" };
rust_write;

WriteAll;
