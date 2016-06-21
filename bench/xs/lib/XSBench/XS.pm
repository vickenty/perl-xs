package XSBench::XS;

use strict;
use warnings;
use XSLoader;
use Exporter "import";

our @EXPORT = qw/xs_sum xs_gcd/;

XSLoader::load();

1;
