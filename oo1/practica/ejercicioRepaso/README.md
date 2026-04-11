# Ejercicio - Recetas de cocina

## Composición de recetas de cocina
Se busca desarrollar un programa que permita representar recetas de cocina. En este ejercico, nos enfocaremos solo en una parte de dicho programa: la obtención de la descripción de una receta y el cálculo de su costo estimado.

Las **recetas** pueden incluir bases, condimentos o proteínas.

De las **bases** se conoce: tipo de base, cantidad de porciones y si la base es integral o tradicional.

De los **condimentos** se conoce: mezcla de condimentos, cantidad de cucharaditas y si el condimento es picante o no picante.

De las **proteínas** se conoce: tipo de proteína, forma de presentación, cantidad de porciones y precio por porción.

Las recetas tienen un nombre. Una receta generalmente incluye componentes de distintos tipos.

Se requiere implementar la siguiente funcionalidad:

## 1. Obtener la descripción de una receta
La descripción de una receta es un String de varias líneas (separadas por saltos de línea) que contiene la descripción de cada componente de la receta, uno por línea. Por ejemplo:

Receta “Bowl tibio de pollo”

1. Base de arroz (integral, 2 porciones)

2.Condimento mix provenzal (no picante, 3 cucharaditas)

3.Proteína de pollo en cubos (2 porciones a $2200 por porción)

Las descripciones de los componentes siguen un cierto patrón que depende de cada tipo de componente:

Para bases:  "Base de <tipo de base> (<integral/tradicional>, <cantidad de porciones> porciones)"

Para condimentos:  "Condimento <mezcla> (<picante/no picante>, <cantidad de cucharaditas> cucharaditas)"

Para proteínas:  "Proteína de <tipo de proteína> en <forma de presentación> (<cantidad de porciones> porciones a $<precio por porción> por porción)"

 

Nota: para concatenar textos, pueden utilizar el operador +. Para generar un salto de línea en un String, utilicen "\n" donde sea necesario.

## 2. Cálculo del costo estimado de una receta
El costo estimado de una receta se calcula como la suma de los costos estimados de cada componente incluido en ella, teniendo en cuenta lo siguiente: 

Las bases tradicionales generan un costo fijo de $1500 cada una. 

Las bases integrales generan un costo fijo de $2200 cada una. 

Los condimentos no generan costo, ya que se consideran parte del stock básico de cocina. 

Las proteínas generan un costo igual al precio por porción multiplicado por la cantidad de porciones.

 
## Tareas
Realice el diagrama de clases UML que documente su diseño.

Implemente en Java todo lo necesario para resolver la funcionalidad de descripción y cálculo del costo estimado de una receta.

Provea un fragmento de código Java que instancie completamente el ejemplo presentado anteriormente (“Bowl tibio de pollo”).

Desarrollo los casos de prueba:

Indique claramente cuáles métodos de cuáles clases deberían ser testeados por tests de unidad de JUnit. 

Por cada uno de los métodos que se debería testear, indique todos los casos que considera que habría que probar. Para cada caso especifique el fixture (la configuración de objetos que utilizaría). Para cada configuración, indique el resultado que debería esperarse al ejecutar el test.