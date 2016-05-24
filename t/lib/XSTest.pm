package XSTest;

use strict;
use warnings;

sub dies {
    die "Croak!\n";
}

require XSLoader;
XSLoader::load();
