use strict;
use warnings;

use Test::More;
use Test::Fatal;
use Test::LeakTrace;

require_ok("XSTest");

is exception { XSTest::Panic::test_panic() }, "Panic!\n", "panic ok";
is XSTest::Panic::unwind_counter(), 1, "panic unwind ok";

is exception { XSTest::Panic::test_croak() }, "Croak!\n", "croak ok";
is XSTest::Panic::unwind_counter(), 1, "croak unwind ok";

done_testing;
