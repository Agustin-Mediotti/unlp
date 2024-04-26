program ejercicio_6;

procedure update_max_product_type (precio, codigo: integer; tipo: string; var max_price_pant, cod_max_pant: integer);
begin
  if ((tipo = 'pantalon') and (precio > max_price_pant)) then begin
    cod_max_pant:= codigo;
    max_price_pant:= precio;
  end;
end;

procedure update_cheapest (precio, codigo: integer; tipo: string; var cheapest_product_cod, cheapest_product2_cod, cheapest_product_val, cheapest_product2_val: integer);
begin
  if precio < cheapest_product_val then begin
    cheapest_product2_cod:= cheapest_product_cod;
    cheapest_product2_val:= cheapest_product_val;
    cheapest_product_val:= precio;
    cheapest_product_cod:= codigo;
  end;
end;

function calc_promedio(var total: integer; cantidad, precio: integer): real;
begin
  total:= total+precio;
  calc_promedio:= total/cantidad;
end;

var
  i, cod, product_price, total, max_price_pant, cod_max_pant, cheapest_product_cod: integer;
  cantidad, cheapest_product2_cod, cheapest_product_val, cheapest_product2_val: integer;
  tipo: string;
  promedio: real;

begin
  max_price_pant:=-9999;
  cheapest_product_val:=9999;
  cheapest_product2_val:=9999;
  cantidad:=0;
  total:=0;

  for i:=1 to 100 do begin
    read(cod);
    read(tipo);
    read(product_price);
    cantidad:= cantidad+1;
    update_max_product_type(product_price, cod, tipo, max_price_pant, cod_max_pant);
    update_cheapest(product_price, cod, tipo, cheapest_product_cod, cheapest_product2_cod, cheapest_product_val, cheapest_product2_val);
    promedio:= calc_promedio(total, cantidad, product_price);
  end;

  writeln('codigo de los dos productos mas baratos: ', cheapest_product_cod, ' y ', cheapest_product2_cod);
  writeln('codigo del producto tipo pantalon mas caro: ', cod_max_pant);
  writeln('precio promedio: ', promedio);
end.
