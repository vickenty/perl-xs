use strict;
use warnings;

use Test::More;
require_ok("XSTest");

my @kv;

@kv = (alpha => 1, beta => "B", -charlie => "C", delta => 0, _echo => "E");
my $expecting = 'TestStruct { alpha: true, beta: Some("B"), charlie: Some("C"), delta: Some(false), echo: Some("E") }';
is XSTest::Derive::take_kv_struct(@kv), $expecting, "take_kv_struct basic";

done_testing;

# struct TestStruct {
#     alpha:          bool,
#     beta:           String,
#     #[perlxs(key = "-charlie")]
#     charlie:        Option<String>,
#     delta:          Option<bool>,
#     #[perlxs(key = "_echo")]
#     echo:           Option<String>,
# }