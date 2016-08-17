use strict;
use warnings;

use Test::More;
use Test::LeakTrace;

require_ok("XSTest");

my $av = [];

XSTest::Array::test_store($av, 42);
is scalar @$av, 1, "array length";
is $av->[0], 42, "stored value";

$av = [ 42, 42 ];
is XSTest::Array::test_fetch($av), 1, "elem defined";
$av->[0] = undef;
is XSTest::Array::test_fetch($av), 2, "elem exists";
delete $av->[0];
is XSTest::Array::test_fetch($av), 3, "elem empty";
is scalar @$av, 2, "array len is correct";

no_leaks_ok { XSTest::Array::test_store($av, 42) };

done_testing;
