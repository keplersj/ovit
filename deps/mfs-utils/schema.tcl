#!/tvbin/tivosh
set db [dbopen]
transaction {
  set types [db $db schema types]
  set i 1
  foreach type $types {
    set attrs [db $db schema attrs $type]
    set j 1
    foreach attr $attrs {
      set ai [db $db schema attrinfo $type $attr]
      puts "$i $type $j $attr $ai"
      set j [expr $j+1]
      #13-15 never seem to be used -- embeem
      if { $j == 13 } { set j 16 }
    }
    set i [expr $i+1]
  }
}
