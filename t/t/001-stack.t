use strict;
use warnings;
use Test::More tests => 3;
use Test::LeakTrace;

require_ok("XSTest");

is_deeply [ XSTest::test_push() ], [ -1, 42, 42 ** 0.5, "Don't panic!" ], "scalars ok";

no_leaks_ok { XSTest::test_push() }

