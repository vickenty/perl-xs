# common
requires "XSLoader";
requires "Module::Install::Rust" => "0.03";

# used by bench/
requires "Dumbbench";

# used by t/
requires "Test::More";
requires "Test::Fatal";
requires "Test::LeakTrace";

# used by perl-sys crate
requires "Ouroboros" => "0.13";
