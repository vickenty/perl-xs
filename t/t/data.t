use strict;
use warnings;
use utf8;

use Test::More;
use Test::LeakTrace;

require_ok("XSTest");

{
    my $c = XSTest::Data->new(0);
    is $c->get(), 0, "got initial value";
    is $c->inc(), undef, "inc returns undef";
    is $c->get(), 1, "inc incremented";
}

no_leaks_ok(sub {
    my $c = XSTest::Data->new(1);
    $c->get();
    $c->inc();
    $c->get();
});

done_testing;
