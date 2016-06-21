package XSBench::RS;
use strict;
use warnings;
use XSLoader;
use Exporter "import";

our @EXPORT = qw/rs_sum rs_gcd/;

XSLoader::load();

1;
