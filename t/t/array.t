use strict;
use warnings;

use Test::More;
use Test::LeakTrace;

require_ok("XSTest");

my $av = [];

no_leaks_ok {
    XSTest::Array::test_store($av, 42);
};
is scalar @$av, 1, "array length";
is $av->[0], 42, "stored value";

$av = [ 1, 2 ];
XSTest::Array::test_clear($av);
is scalar @$av, 0, "array cleared";

$av = [ 42, 42 ];
is XSTest::Array::test_fetch($av), 1, "elem defined";
$av->[0] = undef;
is XSTest::Array::test_fetch($av), 2, "elem exists";
delete $av->[0];
is XSTest::Array::test_fetch($av), 3, "elem empty";
is scalar @$av, 2, "array len is correct";

$av = [ 1, 2, 3 ];
is XSTest::Array::test_delete($av), 1, "delete returns elem";
is_deeply $av, [ undef, 2, 3 ], "array elem deleted";
is XSTest::Array::test_discard($av), undef, "discard returns undef";
is_deeply $av, [ undef, undef, 3 ], "array elem deleted";
is scalar @$av, 3;

$av = [ 1 ];
ok XSTest::Array::test_exists($av), "defined key exists";
$av = [ undef ];
ok XSTest::Array::test_exists($av), "undef key exists";
$av = [];
ok !XSTest::Array::test_exists($av), "key does not exist";

$av = [ 1 ];
XSTest::Array::test_fill($av);
is_deeply $av, [ 1, undef, undef, undef, undef ], "array filled";

is XSTest::Array::test_top_index([]), -1, "top index of empty array";
is XSTest::Array::test_top_index([ 1 ]), 0, "top index of len 1 array";
is XSTest::Array::test_top_index([ 1, 2, 3 ]), 2, "top index of len 3 array";

$av = [ 1, 2 ];
is XSTest::Array::test_pop($av), 2, "pop array";
is XSTest::Array::test_pop($av), 1, "pop array";
is XSTest::Array::test_pop($av), undef, "pop empty array";

$av = [];
XSTest::Array::test_push($av, 1);
XSTest::Array::test_push($av, 2);
is_deeply $av, [ 1, 2 ], "push array";

$av = [ 1, 2 ];
is XSTest::Array::test_shift($av), 1, "shift array";
is XSTest::Array::test_shift($av), 2, "shift array";
is XSTest::Array::test_shift($av), undef, "shift empty array";

$av = [ 1 ];
XSTest::Array::test_unshift($av);
is_deeply $av, [ undef, undef, 1 ], "unshift array";

$av = [ 1, 2 ];
XSTest::Array::test_undef($av);
is_deeply $av, [], "undef array";

$av = [ 1, 20, 300, 4000 ];
is XSTest::Array::test_iter($av), 4321, "array iter";


done_testing;
