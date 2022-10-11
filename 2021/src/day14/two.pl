#!/usr/bin/env perl
use Data::Dumper;
use List::Util qw(min max);

my %memoize_transform = ();
sub transform {
    my $pair = $_[0];
    my $depth = $_[1];
    my %result = ();
    if ( $memoize_transform{$pair . $depth} ) { return $memoize_transform{$pair . $depth}; }
    $result{$rules{$pair}{"adds"}} += 1;
    if ($depth == 1 ) {
        $memoize_transform{$pair . $depth} = \%result;
        return \%result;
    }
    if ($rules{$pair}{"next"}[0]) { add_counts(\%result, transform( $rules{$pair}{"next"}[0], $depth -1 )); }
    if ($rules{$pair}{"next"}[1]) { add_counts(\%result, transform( $rules{$pair}{"next"}[1], $depth -1 )); }
    $memoize_transform{$pair . $depth} = \%result;
    return \%result;
}
sub add_counts {
    my $left = $_[0]; 
    my $right = $_[1];
    foreach my $key (keys %{$right}) { $left->{$key} += $right->{$key}; }
    return \%left;
}

my $template = <STDIN>;
chomp($template);
<STDIN>;
%rules = ();
while (<STDIN>) {
    /^(.)(.) -> (.)/;
    my @next = ( $1.$3, $3.$2 );
    my %rule = ( "adds" => $3, 'next' => \@next );
    $rules{$1.$2} = \%rule;
}
my %res = ();
for ($i=length($template) - 1; $i>=0; $i--) { $res{substr($template, $i, 1)}++; }
for ($i=length($template) - 2; $i>=0; $i--) {
    if ($rules{substr($template, $i, 2)}) { add_counts(\%res, transform(substr($template, $i, 2), 40)); }
}
print((max(values %res) - min(values %res)) . "\n");