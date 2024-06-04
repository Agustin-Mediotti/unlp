program ejercicio_6_inciso_b;

type
  texto = string[24];
  ptr_str = ^texto;

var
  list_str: ptr_str;
  str: texto;

begin
  new(list_str);
  writeln(sizeof(str) + sizeof(list_str^))
end.
