# motivación

Logica de la aplicacion => descripción del dominio
foco de la materia => lógica de la app + testing (unit)

# sistema orientado a objetos

**Objetos**
-  datos + comportamiento
-  no tiene un main
-  muy cohesivo + poco acoplado
-  Jerarquía: **procedimientos** y **subprocedimientos**
   -  Red de objetos
   -  compisición o subclasificación
-  Son **Abstracciones del dominio**
-  Tiene:
   -  **Estado interno**: 
      -  ocultan datos + son poco acoplados
      -  Asegura el **encapsulamiento**
      -  Oculta el estado interno del objeto para lograr el bajo acoplamiento.
      -  reciben mensajes y metodos
      -  el estado interno se mantiene en sus variables de **instancia**
   -  **Comportamiento**
      -  **binding dinámico**: la clase que llama el método de una instancia no conoce la implementación del metodo que se ejecuta.
      -  Siempre hay un receptor de un mensaje
   -  **Identidad**:
      -  las variables son punteros a objetos -> más de una variable pueden apuntar a un mismo objeto.

> **Binding dinámico**: El lenguaje permite desacoplar las invocaciones de las implementaciones.