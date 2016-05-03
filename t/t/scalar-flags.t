use strict;
use warnings;
use Test::More;
use Test::LeakTrace;

require_ok("XSTest");

is_deeply [ map 0+$_, XSTest::test_sv_ok(undef) ], [ 0, 0, 0, 0, 0, 0 ], "undef";
is_deeply [ map 0+$_, XSTest::test_sv_ok(+1) ], [ 1, 1, 0, 0, 0, 0 ], "+1";
is_deeply [ map 0+$_, XSTest::test_sv_ok(-1) ], [ 1, 1, 0, 0, 0, 0 ], "-1";
is_deeply [ map 0+$_, XSTest::test_sv_ok(~0) ], [ 1, 1, 1, 0, 0, 0 ], "~0";
is_deeply [ map 0+$_, XSTest::test_sv_ok(.5) ], [ 1, 0, 0, 1, 0, 0 ], ".5";
is_deeply [ map 0+$_, XSTest::test_sv_ok("") ], [ 1, 0, 0, 0, 1, 0 ], "str";
is_deeply [ map 0+$_, XSTest::test_sv_ok([]) ], [ 1, 0, 0, 0, 0, 1 ], "ref";

no_leaks_ok { XSTest::test_sv_ok(undef) };
no_leaks_ok { XSTest::test_sv_ok("") };
no_leaks_ok { XSTest::test_sv_ok([]) };

sub a {}

is_deeply [ map 0+$_, XSTest::test_sv_type(\undef) ], [ 1, 0, 0, 0, 0 ], "undef";
is_deeply [ map 0+$_, XSTest::test_sv_type(\42) ], [ 1, 0, 0, 0, 0 ], "num";
is_deeply [ map 0+$_, XSTest::test_sv_type(\"") ], [ 1, 0, 0, 0, 0 ], "str";
is_deeply [ map 0+$_, XSTest::test_sv_type([0]) ], [ 0, 1, 0, 0, 0 ], "arr";
is_deeply [ map 0+$_, XSTest::test_sv_type({ }) ], [ 0, 0, 1, 0, 0 ], "hash";
is_deeply [ map 0+$_, XSTest::test_sv_type(\&a) ], [ 0, 0, 0, 1, 0 ], "code";
is_deeply [ map 0+$_, XSTest::test_sv_type(\*a) ], [ 1, 0, 0, 0, 1 ], "glob";

no_leaks_ok { XSTest::test_sv_type(\undef) };
no_leaks_ok { XSTest::test_sv_type(\"") };
no_leaks_ok { XSTest::test_sv_type([0]) };
no_leaks_ok { XSTest::test_sv_type(\*a) };

done_testing;
