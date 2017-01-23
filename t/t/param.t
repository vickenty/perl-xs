use strict;
use warnings;

use Test::More;
use Test::Fatal;

require_ok("XSTest");

like exception { XSTest::Param::add() }, qr/not enough arguments/, "dies with no args";
like exception { XSTest::Param::add(1) }, qr/not enough arguments/, "dies with one arg";
like exception { XSTest::Param::len(1) }, qr/invalid argument/, "dies with invalid arg";

sub test {
    my ($a, $b, $exp, $exp_warn, $name) = @_;
    my @warn;

    local $Test::Builder::Level += 1;
    local $SIG{__WARN__} = sub { push @warn, @_ };

    is XSTest::Param::add($a, $b), $exp, "$name: result";
    ok @warn == 0+!!$exp_warn, "$name: number of warnings";
    like $_, $exp_warn, "$name: warning text" foreach @warn;
}

test 1, 2, 3, undef, "works with two args";
test "3.14", 2.71, 5, undef, "converts args";
test 1, undef, 1, qr/uninitialized/, "warns on undef param";
test "2", "b", 2, qr/isn't numeric/, "warns on non numeric param";

like exception { XSTest::Param::strlen("\xFF") }, qr/invalid utf-8/, "dies on invalid utf8";
is XSTest::Param::strlen("Don't panic"), 11, "ascii works ok";
{
    no utf8;
    is XSTest::Param::strlen("慌てる必要がありません"), 11, "undecoded utf8 works ok";
}
{
    use utf8;
    is XSTest::Param::strlen("慌てる必要がありません"), 11, "decoded utf8 works ok";
}

done_testing;
