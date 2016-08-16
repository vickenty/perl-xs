use strict;
use warnings;
use Test::More;
use Test::LeakTrace;

require_ok("XSTest");

cmp_ok XSTest::test_new_sv_iv(42), "==", 42, "iv ok";
no_leaks_ok { XSTest::test_new_sv_iv(42) };

cmp_ok XSTest::test_new_sv_iv(0x7FFFFFFF), "==", 0x7FFFFFFF, "large iv ok";
cmp_ok XSTest::test_new_sv_iv(-0x80000000), "==", -0x80000000, "small iv ok";

cmp_ok XSTest::test_new_sv_nv(42**0.5), "==", 42**0.5, "nv ok";
no_leaks_ok { XSTest::test_new_sv_nv(42**0.5) };

cmp_ok XSTest::test_new_sv_nv(0+"inf"), "==", "inf", "inf nv ok";
cmp_ok XSTest::test_new_sv_nv(0+"-inf"), "==", "-inf", "-inf nv ok";
is XSTest::test_new_sv_nv(0+"nan"), 0+"nan", "nan nv ok";

ok !defined(XSTest::test_new_sv_undef()), "undef ok";
no_leaks_ok { XSTest::test_new_sv_undef() };

done_testing;
