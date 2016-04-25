use strict;
use warnings;
use Test::More;
use Test::LeakTrace;

require_ok("XSTest");

is_deeply [ map 0+$_, XSTest::test_sv_ok(+1) ], [ 1, 0, 0, 0, 0 ], "+1";
is_deeply [ map 0+$_, XSTest::test_sv_ok(-1) ], [ 1, 0, 0, 0, 0 ], "-1";
is_deeply [ map 0+$_, XSTest::test_sv_ok(~0) ], [ 1, 1, 0, 0, 0 ], "~0";
is_deeply [ map 0+$_, XSTest::test_sv_ok(.5) ], [ 0, 0, 1, 0, 0 ], ".5";
is_deeply [ map 0+$_, XSTest::test_sv_ok("") ], [ 0, 0, 0, 1, 0 ], "str";
is_deeply [ map 0+$_, XSTest::test_sv_ok([]) ], [ 0, 0, 0, 0, 1 ], "ref";

no_leaks_ok { XSTest::test_sv_ok("") };
no_leaks_ok { XSTest::test_sv_ok([]) };

done_testing;
