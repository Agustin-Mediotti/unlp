# Orientacion a Objetos 1

## Tareas:
1. Analice el código existente. Utilice [el código y los tests](https://drive.google.com/file/d/1u5gmDqECvWlQmcq6rgNWGMLjayOJaiiA/view?usp=sharing) provistos por la cátedra y aplique lo aprendido (en particular en relación a herencia y polimorfismo) para eliminar los problemas mencionados. Siéntase libre de agregar nuevas clases como considere necesario. También puede cambiar la forma en la que los objetos se crean e inicializan. Asuma que una vez elegida una estrategia para un scheduler no puede cambiarse.
2. Verifique su solución con las pruebas automatizadas
Sus cambios probablemente hagan que los tests dejen de funcionar. Corríjalos y mejórelos como sea necesario.

Para mejorar el diseño, segun lo pedido habria que crear clases concretas de cada estrategia para eliminar el codigo duplicado y desacoplar.
Para eso implemente el patron [Strategy Design Pattern](https://www.geeksforgeeks.org/system-design/strategy-design-pattern-in-java/) ya que otorga desacoplamiento en las clases y permite que el contexto elija un estrategia concreta para realziar la tarea.