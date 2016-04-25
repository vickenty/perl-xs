use strict;
use warnings;

use Test::More;
use Test::LeakTrace;

require_ok("XSTest");

my $av = [];

XSTest::Array::test_store($av, 42);
is scalar @$av, 1, "array length";
is $av->[0], 42, "stored value";

no_leaks_ok { XSTest::Array::test_store($av, 42) };

done_testing;
