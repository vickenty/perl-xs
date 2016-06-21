use strict;
use warnings;

use Test::More;
use Test::LeakTrace;
use utf8;

require_ok("XSTest");

my $hv;

$hv = {};
XSTest::Hash::test_store($hv, "Don't panic!", 42);
is_deeply $hv, { "Don't panic!" => 42 }, "store latin1";

no_leaks_ok { XSTest::Hash::test_store({}, "Don't panic!", 42); };

$hv = {};
XSTest::Hash::test_store($hv, "Nu intrat\x{0326}i i\x{0302}n panica\x{0306}!", 42);
is_deeply $hv, { "Nu intrat\x{0326}i i\x{0302}n panica\x{0306}!" => 42 }, "store unicode";

no_leaks_ok { XSTest::Hash::test_store({}, "Nu intrat\x{0326}i i\x{0302}n panica\x{0306}!", 42); };

$hv = { "Don't panic!" => 42 };
is XSTest::Hash::test_exists($hv, "Don't panic!"), 1, "exists";
XSTest::Hash::test_clear($hv);
is_deeply $hv, {}, "clear";

no_leaks_ok { XSTest::Hash::test_clear({ "Don't panic" => 42 }) };

$hv = { "Don't panic!" => 42 };
is XSTest::Hash::test_delete($hv, "Don't panic!"), 42, "delete returns value";
is_deeply $hv, {}, "delete removes value";

no_leaks_ok { XSTest::Hash::test_delete({ "Don't panic!" => 42 }, "Don't panic!" ); };

done_testing;
