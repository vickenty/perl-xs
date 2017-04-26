use strict;
use warnings;
use utf8;
use Test::More tests => 10;
use Test::LeakTrace;

require_ok("XSTest");

is_deeply [ XSTest::test_push() ], [ -1, 42, 42 ** 0.5, "Don't panic!", 1, "" ], "scalars ok";

no_leaks_ok { XSTest::test_push() };

my $strings = [ XSTest::test_push_unicode() ];
is_deeply $strings, [
    "Don't panic!",
    "Не паникуйте!",
    "あわてるな",
    "Nu intra i\x{0302}n panica\x{0306}",
    "😱❌",
], "unicode strings ok";

ok utf8::is_utf8($strings->[$_]), "string $_ is utf8"
    for (0..$#$strings);

no_leaks_ok { XSTest::test_push_unicode() };
