#!/usr/bin/env perl
use Data::Dumper;
use List::Util qw(min max);
my $template = <STDIN>;
chomp($template);
<STDIN>;
@rules = ();
while (<STDIN>) {
    /^(.)(.) -> (.)/;
    my @pair = ($1 . $2, $1 . "-" . $3 . "-" . $2); 
    push @rules, \@pair;
}
@pair = ("-", "");
push @rules, \@pair;
for ($i=0; $i<10; $i++) {
    foreach $rule (@rules) {
        $search = @{$rule}[0];
        $replace = @{$rule}[1];
        while ($template =~ m/$search/) {
            $template =~ s/${search}/${replace}/g;
        }
    }
}
my %count;
for ($i=length($template) - 1; $i>=0; $i--) {
    $count{substr($template, $i, 1)}++;
}
print( (max(values %count) - min(values %count)) . "\n");