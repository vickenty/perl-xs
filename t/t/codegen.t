use strict;
use warnings;

use Test::More;
require_ok("XSTest");

my @kv;

@kv = (alpha => 1, beta => "B", -charlie => "C", delta => 0, _echo => "E");
is XSTest::Codegen::take_kv_struct(@kv), "", "take_kv_struct basic";

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