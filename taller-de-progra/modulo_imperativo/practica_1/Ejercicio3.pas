program Ejercicio3;

const 
  FIN = -1;
  GENEROS = 8;
type
  cod_gen = 1..GENEROS;
  pelicula = record
    gen: cod_gen;
    cod: integer;
    rate: real;
  end;
  lista = ^nodo;
  nodo = record
    dato: pelicula;
    sig: lista;
  end;
  more_rated_cod = record
    cod: integer;
    rate_avr: real;
  end;
  vector_pelis = array[cod_gen] of lista;           // TODO: Agregar un registro con PRI y ULT para AgregarAtras()
  vector_best = array[cod_gen] of more_rated_cod; 

function randi(min,max:integer):integer;
var num:integer;
begin
  num:=0;
  while num < min do
    num:=random(max);
  randi:=num; 
end;

procedure MejorPeliculaDelGenero(L:lista; var peli:more_rated_cod);
var
  avr:real;
  cod:integer;
begin
  avr:=-1;
  cod:=-1;
  while l <> nil do begin
    if l^.dato.rate > avr then begin
      avr:= l^.dato.rate;
      cod:=l^.dato.cod;
    end;
    l:=l^.sig;
  end;
  peli.cod:=cod;
  peli.rate_avr:=avr;
end;

procedure MejoresPeliculasPorGenero(var vec_p:vector_pelis; vec_b:vector_best);
var
  i:integer;
  e:more_rated_cod;
begin
  for i:=1 to GENEROS do begin
    MejorPeliculaDelGenero(vec_p[i],e);
    vec_b[i]:=e;
  end;
end;

procedure LeerPelicula(var peli:pelicula);
begin
  peli.gen:=randi(Low(cod_gen),High(cod_gen));
  peli.rate:=randi(1,10);
  readln(peli.cod);
end;

procedure AgregarAdelante(var L:lista; peli:pelicula);  // TODO: ArgregarAtras()
var nue: lista;
begin
  new(nue);
  nue^.dato:=peli;
  nue^.sig:=L;
  L:=nue;
end;

procedure CargarLista(var vec:vector_pelis);
var aux: pelicula;
begin
  LeerPelicula(aux);
  while aux.cod <> FIN do
    begin
      AgregarAdelante(vec[aux.cod],aux);
      LeerPelicula(aux);
    end;
end;

procedure OrdenarInsercion(var vec:vector_best);
var
  x,i:integer;
  aux:more_rated_cod;
begin
  for x:=2 to GENEROS do begin
    aux:=vec[x];
    i:=x-1;
    while ((i>0)and(vec[i].rate_avr < aux.rate_avr)) do begin
      vec[i+1]:=vec[i];
      i:=i-1;
    end;
    vec[i+1]:=aux;
  end;
end;

procedure InicializarListas(var vec:vector_pelis);
var i:integer;
begin
  for i:=1 to GENEROS do
    begin
      vec[i]:=nil;
    end;
end;

procedure PrintBestNWorst(vec:vector_best);
var
  pri,ult:integer;
begin
  pri:=1; ult:=GENEROS;
  writeln('CODIGO mejor pelicula: ', vec[pri].cod);
  writeln('CODIGO pelicula peor valorada: ', vec[ult].cod);
end;

var
  vec_movies: vector_pelis;
  vec_best_of:vector_best;

begin
  randomize;
  InicializarListas(vec_movies);
  CargarLista(vec_movies);                                {inciso a}
  MejoresPeliculasPorGenero(vec_movies, vec_best_of);     {inciso b}
  OrdenarInsercion(vec_best_of);                          {inciso c}
  PrintBestNWorst(vec_best_of);                           {inciso d}
end.
