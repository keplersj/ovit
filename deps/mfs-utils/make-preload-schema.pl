#! /usr/bin/perl  

use strict;

#1 Test1 1 Version int optional {} base
#1 Test1 2 Expiration int optional {} base


my @attrs;
my @type_names;
my %attr_idx;
my %str2type = (
		"int"    => "TYPE_INT",
		"string" => "TYPE_STRING",
		"object" => "TYPE_OBJECT",
		"file"   => "TYPE_FILE",
	       );
my $attr_count = 0;

#
# Read file
#
while (<>) {
  my ($itype, $typestr, $iattr, $attrstr, $attrtype)  = split(' ', $_);
  $attr_idx{$attrstr} = $attr_count++
    if (!defined($attr_idx{$attrstr}));
  $type_names[$itype] = $typestr;
  my $type = $str2type{$attrtype};
  $attrs[$itype][$iattr] = [ $attr_idx{$attrstr}, defined($type)?$type:0  ];
}

#
# Print the array of type names
#
printf <<EOF,scalar(@type_names);
#include "mfs.h"

typedef struct {
	const char *name;
	int objtype;
} attr_t;

static const char *stype_names[] = {
EOF
for(my $i=0; $i<scalar(@type_names); $i++) {
  my $str=$type_names[$i];
  $str = defined($str)?"\"$str\"":0;
  printf "\t$str,\n";
}

#
# Print the array of attr names
#
printf "};\n";


my @attr_names;
while( my ($n,$i) = each %attr_idx) {
  @attr_names[$i] = $n;
}

for(my $i=0; $i<scalar(@attr_names); $i++) {
  printf("static const char attrname_%d[] = \"$attr_names[$i]\";\n",$i)
    if (defined($attr_names[$i]));
}

#
# Construct a double indirection table of attr_t structs indexed by type and attr
#
for(my $i=0; $i<scalar(@attrs); $i++) {
  next if (!defined($attrs[$i]));
  printf("static attr_t sattrs_%d[] = {\n",$i);
  my @a = @{$attrs[$i]};
  for(my $j=0; $j<scalar(@a); $j++) {
    if (!defined($a[$j])) {
      print "\t{},\n";
      next;
    }
    my ($attr,$type) = @{$a[$j]};
    $attr = defined($attr)?"attrname_$attr":0;
    printf "\t{ $attr, $type, },\n";
  }
  printf "};\n";
}

printf <<EOF;

typedef struct { int n; attr_t *a; } attrs_t;

static attrs_t sattrs[] = {
EOF

for(my $i=0; $i<scalar(@attrs); $i++) {
  if (!defined($attrs[$i])) {
    print "\t{ 0, 0 },\n";
  } else {
    my @a = @{$attrs[$i]};
    my $n = scalar(@a);
    print "\t{ $n, sattrs_$i },\n";
  }
}
printf "};\n";
    


