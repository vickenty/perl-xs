use strict;
use warnings;
use utf8;

use Test::More;

use constant {
    UTF_OK => -1,
    ASCII => "Don't panic!",
    KANJI => "慌てる必要がありません",
    DIACR => "a\x{035C}\x{0361}",

    NONUTF => "Don't\xffpanic!",
    NONUTF_OFFSET => 5,
};

require_ok("XSTest");

is_deeply [ XSTest::test_unicode(ASCII) ], [ ASCII, UTF_OK ], "ascii ok";
is_deeply [ XSTest::test_unicode(KANJI) ], [ KANJI, UTF_OK ], "kanji ok";
is_deeply [ XSTest::test_unicode(DIACR) ], [ DIACR, UTF_OK ], "diacritics ok";

is_deeply [ XSTest::test_unicode(NONUTF) ], [ "", NONUTF_OFFSET ], "binary not ok";

done_testing;
