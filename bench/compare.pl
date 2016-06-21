use strict;
use warnings;

use blib "xs/blib";
use blib "rs/blib";

use XSBench::XS qw/xs_sum xs_gcd/;
use XSBench::RS qw/rs_sum rs_gcd/;
use Dumbbench;

sub bench {
    my @tests;
    while (@_) {
        push @tests, Dumbbench::Instance::PerlSub->new(name => shift, code => shift);
    }

    my $bench = Dumbbench->new(
        target_rel_precision => 0.005,
        initial_run => 20,
    );
    $bench->add_instances(@tests);
    $bench->run;
    $bench->report;
}

sub pp_sum {
    my $array = shift;
    my $sum = 0;
    foreach my $val (@$array) {
        $sum += $val;
    }
    return $sum;
}

sub pp_gcd {
    my $a = shift;
    my $b = shift;
    my $t;
    while ($b > 0) {
        $t = $a;
        $a = $b;
        $b = $t % $b;
    }

    return $a < 0 ? -$a : $a;
}

my ($pp, $xs, $rs);

my $specimen = [ 1 .. 50_000_000 ];

bench(
    pp => sub { $pp = pp_sum($specimen) },
    xs => sub { $xs = xs_sum($specimen) },
    rs => sub { $rs = rs_sum($specimen) },
);

print "pp=$pp, xs=$xs, rs=$rs\n";

my $a = 4_000_000_000_000;
my $b = 3_984_589_159_111;

bench(
    pp => sub { $pp = pp_gcd($a, $b) for 0..200000 },
    xs => sub { $xs = xs_gcd($a, $b) for 0..200000 },
    rs => sub { $rs = rs_gcd($a, $b) for 0..200000 },
);
print "pp=$pp, xs=$xs, rs=$rs\n";
