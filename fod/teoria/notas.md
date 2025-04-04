## Archivos - Definición
Los datos que necesiten ser preservados mas alla del tiempo de ejecucución deben residir en archivos sobre dispositivos de almacenamiento permanente de información. Un archivo es una estructura de datos homgenea, caracterizada por el crecimiento y las modificaciones que se efectuan sobre estos.

## Aspectos físicos
- *Capacidad*: La memoria RAM tiene capacidad de acceso más limitada que, por ejemplo, un disco rígido.
- *Tiempo de Acceso*: La memoria RAM mide su tiempo de acceso en orden de nanosegundos, el disco rígido en milisegundos.

## Correspondencia archivo lógico - archivo físico
El SO opera sobre el archivo físico, el algoritmo utiliza un tipo de datos *file* que representa el nombre del mísmo. Se debe indicar que el archivo lógico corresponde al archivo físico administrado por el SO. Algunas definiciones de nombre físico contienen la extensión que se queire dar al archivo. Un archivo de tipo *string* contendrá elementos de *255 caracteres de longitud* en cada caso.

## Buffer de Datos
Debido a que las operaciones de lectura/escritura sobre disco tienen una velocidad de acceso menor a la RAM (milisec vs nanosec o 10⁻³ vs 10⁻⁹), por una cuestion de performance se utiliza una memoria intermedia denonimada *buffer* que se encuentra en la memoria RAM. Donde las operaciones de *read* y *write* van a intentar acceder para leer o escribir en primera instancia al buffer y si este esta lleno (para la escritura) o vacio (para la lectura) entonces hara el SO hara una operación de *input*. 


## Operaciones esenciales sobre archivos
- *Alta*: Ingresar nuevos datos al archivo.
- *Modificacion*: Alterar el contenido de algún dato del archivo.
- *Consulta*: Presentar el contenido total o parcial del archivo.
- *Baja*: Quitar información del archivo.
