program ejercicio_6;

var
  code: 1..200;
  price, i, cheap, cheap_cod, cheap_2, cheap_cod_2, even_cod_plus16_worth: integer;
begin
  cheap:=9999;
  cheap_2:=9999;
  cheap_cod:=0;
  cheap_cod_2:=0;
  even_cod_plus16_worth:=0;
  for i:=1 to 5 do begin
    readln(code);
    readln(price);
    if price < cheap then begin
      cheap_2:=cheap;
      cheap:=price;
      cheap_cod_2:=cheap_cod;
      cheap_cod:=code;
    end;
    if (price > 16) and (code mod 2 = 0) then
      even_cod_plus16_worth:=even_cod_plus16_worth+1;
  end;
  writeln('Los codigos de los productos mas baratos son: ', cheap_cod, ' y ', cheap_cod_2);
  writeln('Cantidad de productos de mas de 16 pesos con codigo par: ', even_cod_plus16_worth);
end.
