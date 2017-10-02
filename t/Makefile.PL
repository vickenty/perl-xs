use inc::Module::Install;

name "XSTest";
version "0.01";
abstract "Tests for perl-xs";

configure_requires "Module::Install::Rust" => 0;
test_requires "Test::LeakTrace" => 0;
test_requires "Test::More" => 0;
test_requires "Test::Fatal" => 0;
requires "XSLoader" => 0;

rust_use_perl_xs { path => ".." };

#TODO: Change this to use the crate once published to crates.io
rust_requires "perl-sys" => { git=>"https://github.com/vickenty/perl-sys" };
rust_write;

WriteAll;
