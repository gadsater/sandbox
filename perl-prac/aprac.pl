use strict;
use warnings;
use diagnostics;

use feature 'say';

use feature "switch";

# Comment

print "Hello World\n";

my $name = 'gadsater';

my ($num, $str) = (3725, "Hello world");

my $an_str = "$name, $num and $str\n";

$an_str = qq{$name, $num and $str\n};

print $an_str;
