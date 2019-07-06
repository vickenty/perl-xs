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

$hv = { a => 1, b => 20, c => 300, d => 4000 };
is XSTest::Hash::test_iter($hv), 4321, "hash iter";

no_leaks_ok { XSTest::Hash::test_iter({ a => 1, b => 20, c => 200, d => 4000 }); };

$hv = { a => 1, b => 20, c => 300, d => 4000 };
is XSTest::Hash::test_values($hv), 4321, "hash values";

no_leaks_ok { XSTest::Hash::test_values({ a => 1, b => 20, c => 200, d => 4000 }); };

$hv = { a => 1, b => 20, c => 300, d => 4000 };
is XSTest::Hash::test_keys($hv), 4321, "hash keys";

no_leaks_ok { XSTest::Hash::test_keys({ a => 1, b => 20, c => 200, d => 4000 }); };

$hv = { a => 1, b => 20, c => 300, d => 4000 };
is XSTest::Hash::test_for($hv), 4321, "hash in for loop";

no_leaks_ok { XSTest::Hash::test_for({ a => 1, b => 20, c => 200, d => 4000 }); };

done_testing;
