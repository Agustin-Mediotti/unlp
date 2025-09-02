# Obtención de requerimientos

## Parte I Definiciones:
 1. Un requerimiento es una característica del sistema o una descripción que el sistema es capaz de hacer con el objeto de satisfacer el propósito del sistema.
 2. Los **requisitos funcionales** describen lo que debe hacer un sistema, centrandose en acciones, comportamientos e interacciones específicas. Por otro lado los **requisitos no funcionales** describen el rendimiento del sistema, abordando atributos de calidad como el rendimiento, la seguridad y la escalabilidad.
 3. El término **stakeholder** se utiliza para referirse a cualquier persona o grupo que se verá afectado por el sistema, directa o indirectamente. Por ej: Usuarios finales, ingenieros, gerentes, expertos del dominio.
 4. Las fuentes más importantes para la obtención de información son la **documentación**, los **stakeholders** y las **especificaciones de sistemas similares**.
 5. Existen tres tipos de puntos de vista: 
    1. Punto de vista de los **interactuadores**: Representan a las personas u otros sistemas que interactúan directamente con el sistema.
    2. Punto de vista **indirecto**: Representan a los stakeholders que no utilizan el sistema ellos mismos pero que influyen en los requerimientos de algún modo.
    3. Punto de vista del **dominio**: Representan las características y restricciones del dominio que influyen en los requerimientos del sistema.
 6. Tres **problemas de comunicación** que pueden existir en la licitación de requerimientos son:
    1. Dificultad para expresar claramente las necesidades.
    2. No ser conscientes de sus propias necesidades.
    3. Intereses distintos en el sistema a desarrollar.

## Parte I Problemas:

### a. 
1. 
**Stakeholders**:
   - Alumnos de la materia, 
   - Profesor a cargo,
   - JTP del turno y
   - La oficina de alumnos

**Puntos de Vista**:
   - **Interactuador**:
     - Los alumnos y
     - El Profesor a cargo
   - **Indirectos**:
     - JTP del turno
   - **Dominio**:
     - La reglamentación sobre la privacidad de los datos,
     - La autorización del JTP y
     - El ámbito de la facultad de informática

**Fuentes de Información**:
   - Stakeholders
   - La reglamentación sobre la privacidad de los datos

2. 
**Stakeholders**:
   - Paciente
   - Enfermero
   - Medico
   - Empleado
   - Director de la clínica

**Puntos de Vista**:
   - **Interactuador**:
     - Empleado
     - Enfermero
     - Medico
     - Director de la clínica
   - **Indirectos**:
     - Paciente
   - **Dominio**:
     - Normativa impuesta por el ministerio de salud de la provincia de Bs As
     - Anotaciones habituales y controles

**Fuentes de Información**:
   - Stakeholders
   - Normativa impuesta por el ministerio de salud de la provincia de Bs As
   - Anotaciones habituales y controles

### b. 
Los requerimientos de los distintos stakeholders podrian entrar en conflicto debido a intereses distintos sobre el sistema, por ejemplo: los alumnos podrían no querer usar sus datos bíometricos para el presente, o que los pacientes quieran que su historial clínico sea privado

# Entrevistas

## Parte I Definiciones

1. Por medio de las entrevistas se obtiene información de las personas a través de la interacción cara a cara
2. 

## Parte II Situaciones

**Situación 1**

a. Que opinión tiene del actual funcionamiento de la empresa?

b. Le queda algo más que añadir?

c. Cree usted que sería útil computarizar las ventas mensuales y luego realizar un análisis de la tendencia?

d. Cree usted que se puede mejorar la manera de hacer proyecciones de sus ventas?

**Situación 2**

a. Le preguntaría al entrevistado si es un mal momento para realizar la entrevista, en ese caso propondría reprogramar la entrevista.

b. Si la entrevista no puede reprogramarse de forma presencial, propondría llevarla a cabo de manera virtual.

**Situación 3**

a. La sensación que deja el informe es que la entrevista no estaba bien estructurada, sin tiempos claros, el entrevistador no tuvo el control de la entrevista.

b. La mención de "las cosas no han cambiado desde que él ha estado en la empresa, hace aproximandamente 16 años" es irrelevante.

c. Tres sugerencias que le haria son: Que estructure un guión de entrevista con preguntas concretas y objtivos, Que defina cuanto va a durar la entrevista así como los tiempos para cada pregunta, Y que mantenga el control de la entrevista.

## Parte II Problemas

**Problema 1**

> Se desea desarrollar un sistema que permita **compartir un vehículo para un viaje**. La idea es que cuando una persona 
tiene que realizar un viaje lo publique en la aplicación. Luego el resto de los usuarios **se postulan para acompañarla** y 
el chofer podrá **seleccionar quienes viajan**. El objetivo es **abaratar costos y evitar congestiones en el tránsito**.



|       Entrevistado       |   Fecha    |  Hora   |            Lugar             |                                     Tema                                      |
| :----------------------: | :--------: | :-----: | :--------------------------: | :---------------------------------------------------------------------------: |
| [Nombre del stakeholder] | [DD/MM/AA] | [HH:MM] | [Presencial/Virtual/Oficina] | Requerimientos funcionales y no funcionales del sistema de viajes compartidos |



| Tiempo asignado |                                                   Pregunta del administrador                                                    |                    Objetivo                     | Respuesta del entrevistado |
| :-------------: | :-----------------------------------------------------------------------------------------------------------------------------: | :---------------------------------------------: | :------------------------: |
|    **5 min**    |                                                   **INTRODUCCIÓN Y CONTEXTO**                                                   |                                                 |                            |
|      1 min      |                                    Me presento y doy a conocer el propósito de la entrevista                                    |                  Presentacion                   |                            |
|      2 min      |    ¿Podrías contarme sobre tu experiencia actual con viajes compartidos? <br> **Seguimiento** ¿Has usado alguna app similar?    |   Conocer el background y experiencia previa    |                            |
|      3 min      | ¿Qué te motiva a participar en un sistema de viajes compartidos? <br> **Seguimiento** ¿Cuáles son tus expectativas principales? |  Identificar motivaciones y expectativas clave  |                            |
|   **15 min**    |                                     **PROCESO DE PUBLICACIÓN DE VIAJES (Para conductores)**                                     |                                                 |                            |
|      3 min      |               ¿Qué información considerás esencial incluir al publicar un viaje? (destino, horario, precio, etc.)               |    Definir campos obligatorios y opcionales     |                            |
|      3 min      |        ¿Con cuánta anticipación solés planificar tus viajes? <br> **Seguimiento** ¿Cuándo te gustaría poder publicarlos?        | Establecer ventanas temporales para publicación |                            |
|      2 min      |   ¿Preferirías rutas fijas o la posibilidad de hacer paradas intermedias? <br> **Seguimiento** ¿Cómo manejarías los desvíos?    |        Determinar flexibilidad en rutas         |                            |
|      3 min      |              ¿Cómo te gustaría determinar el costo por pasajero? <br> **Seguimiento** ¿Qué factores considerarías?              |            Definir modelo de pricing            |                            |
|      2 min      |                       ¿Qué información de los pasajeros potenciales te gustaría ver antes de aceptarlos?                        |     Identificar datos de perfil necesarios      |                            |
|      2 min      |                      ¿Preferirías aceptar automáticamente por orden de llegada o poder elegir manualmente?                      |         Definir mecanismo de selección          |                            |
|   **12 min**    |                                     **PROCESO DE BÚSQUEDA Y POSTULACIÓN (Para pasajeros)**                                      |                                                 |                            |
|      3 min      |       ¿Cómo buscarías viajes disponibles? ¿Por destino exacto, zona, horario?  <br> **Seguimiento** ¿Qué filtros usarías?       |       Diseñar funcionalidades de búsqueda       |                            |
|      2 min      |                      ¿Qué información del viaje y del conductor necesitarías ver para decidir postularte?                       |     Definir información visible en listados     |                            |
|      2 min      |   ¿Te gustaría poder postularte a múltiples viajes al mismo tiempo? <br> **Seguimiento** ¿Cómo manejarías las confirmaciones?   |    Establecer reglas de postulación múltiple    |                            |
|      3 min      |                Si no te aceptan en un viaje, ¿te gustaría saber por qué? ¿Preferirías quedar en lista de espera?                |  Definir manejo de rechazos y listas de espera  |                            |
|      2 min      |                         ¿Cómo te gustaría recibir notificaciones sobre el estado de tus postulaciones?                          |      Establecer sistema de notificaciones       |                            |
|    **8 min**    |                                                    **SEGURIDAD Y CONFIANZA**                                                    |                                                 |                            |
|      3 min      |           ¿Qué medidas de seguridad considerás importantes? ¿Verificación de identidad, calificaciones, referencias?            |       Identificar requisitos de seguridad       |                            |
|      2 min      |                           ¿Te gustaría un sistema de calificaciones mutuas? ¿Cómo debería funcionar?                            |          Diseñar sistema de reputación          |                            |
|      3 min      |                    ¿Qué información personal estarías dispuesto/a a compartir? ¿Qué te daría más confianza?                     |          Definir niveles de privacidad          |                            |
|    **8 min**    |                                                    **PAGOS Y TRANSACCIONES**                                                    |                                                 |                            |
|      3 min      |                       ¿Cómo preferirías manejar los pagos? ¿Efectivo, transferencia, a través de la app?                        |             Definir métodos de pago             |                            |
|      2 min      |                          ¿Cuándo debería realizarse el pago? ¿Al confirmar, antes del viaje, después?                           |       Establecer timing de transacciones        |                            |
|      3 min      |                    ¿Qué pasaría si alguien cancela? ¿Debería haber penalizaciones o políticas de reembolso?                     |        Definir políticas de cancelación         |                            |
|    **7 min**    |                                                 **COMUNICACIÓN Y COORDINACIÓN**                                                 |                                                 |                            |
|      3 min      |                 ¿Cómo te gustaría comunicarte con otros usuarios? ¿Chat interno, WhatsApp, solo notificaciones?                 |         Diseñar sistema de comunicación         |                            |
|      2 min      |                 ¿Qué información necesitarías el día del viaje? ¿Datos del auto, teléfono, punto de encuentro?                  |      Definir información del día del viaje      |                            |
|      2 min      |                                ¿Te gustaría compartir ubicación en tiempo real durante el viaje?                                |          Evaluar funciones de tracking          |                            |
|    **8 min**    |                                          **EXPERIENCIA DE USUARIO Y FUNCIONALIDADES**                                           |                                                 |                            |
|      3 min      |                                  ¿Qué dispositivos usarías principalmente? ¿Móvil, web, ambos?                                  |        Definir plataformas prioritarias         |                            |
|      2 min      |                     ¿Te gustaría guardar rutas frecuentes o tener viajes recurrentes (ej: trabajo diario)?                      |      Identificar funciones de conveniencia      |                            |
|      3 min      |                   ¿Qué harías si el conductor o pasajero no aparece? ¿Debería haber un sistema de emergencia?                   |        Definir protocolos de emergencia         |                            |
|    **5 min**    |                                              **ASPECTOS TÉCNICOS Y REGULATORIOS**                                               |                                                 |                            |
|      2 min      |                          ¿Considerás importante que el sistema funcione sin internet en algún momento?                          |         Evaluar requerimientos offline          |                            |
|      3 min      |                      ¿Tenés alguna preocupación sobre aspectos legales o de seguros en viajes compartidos?                      |       Identificar consideraciones legales       |                            |
|    **7 min**    |                                             **CIERRE Y REQUERIMIENTOS ADICIONALES**                                             |                                                 |                            |
|      3 min      |                       ¿Hay alguna funcionalidad que no hayamos mencionado pero que considerás importante?                       |     Capturar requerimientos no contemplados     |                            |
|      2 min      |                  ¿Cuáles serían las 3 características más importantes para que uses este sistema regularmente?                  |         Priorizar funcionalidades clave         |                            |
|      2 min      |                       ¿Hay algo más que te gustaría agregar o alguna preocupación que no hayamos tocado?                        |      Cierre abierto para temas pendientes       |                            |

*Tiempo total estimado: 75 minutos*

**Problema 2**

> **CookBooks** es un negocio pequeño manejado por una pareja jubilada. Hasta este momento, Cookbooks ha vendido 
sus libros sólo a través de **pedidos por correo**. Los dueños ahora quieren desarrollar un **sistema en línea** para vender 
**libros de cocina difíciles de conseguir** y agotados a través de internet. 

> Los visitantes podrán **hojear diferentes libros de cocina**, pero tendrán que crear una **cuenta del cliente** antes de 
poder hacer una compra. **Los pagos se aceptarán sólo en línea** con una tarjeta de crédito reconocida.

| Entrevistado             | Fecha      | Hora    | Lugar                        | Tema                                                                               |
| ------------------------ | ---------- | ------- | ---------------------------- | ---------------------------------------------------------------------------------- |
| [Nombre del stakeholder] | [DD/MM/AA] | [HH:MM] | [Presencial/Virtual/Oficina] | Requerimientos funcionales y no funcionales para tienda online de libros de cocina |



| Tiempo asignado | Pregunta del administrador                                                                                 | Objetivo                                      | Respuesta del entrevistado |
| --------------- | ---------------------------------------------------------------------------------------------------------- | --------------------------------------------- | -------------------------- |
| **8 min**       | **INTRODUCCIÓN Y CONTEXTO DEL NEGOCIO**                                                                    |                                               |                            |
| 1 min           | Me presento y doy a conocer el propósito de la entrevista                                                  | Presentacion                                  |                            |
| 3 min           | ¿Podrían contarme cómo funciona actualmente su negocio de venta por correo? ¿Cuál es el proceso típico?    | Entender el modelo de negocio actual          |                            |
| 3 min           | ¿Qué los motivó a dar el salto al mundo digital? ¿Qué expectativas tienen del sistema online?              | Identificar objetivos y expectativas          |                            |
| 2 min           | ¿Cuántos pedidos manejan aproximadamente por mes/semana actualmente?                                       | Dimensionar volumen de transacciones esperado |                            |
| **12 min**      | **CATÁLOGO Y GESTIÓN DE LIBROS**                                                                           |                                               |                            |
| 3 min           | ¿Cómo organizan actualmente su inventario? ¿Por categorías, autores, tipo de cocina?                       | Diseñar estructura del catálogo               |                            |
| 3 min           | ¿Qué información consideran esencial mostrar de cada libro? (título, autor, año, descripción, etc.)        | Definir campos del producto                   |                            |
| 2 min           | ¿Cómo manejan los libros "difíciles de conseguir" vs los "agotados"? ¿Hay diferencia en la gestión?        | Establecer estados de inventario              |                            |
| 2 min           | ¿Tienen un sistema para notificar cuando un libro agotado vuelve a estar disponible?                       | Evaluar funcionalidad de notificaciones       |                            |
| 2 min           | ¿Quién se encargará de actualizar el catálogo online? ¿Con qué frecuencia?                                 | Definir roles de administración               |                            |
| **10 min**      | **NAVEGACIÓN Y BÚSQUEDA PARA VISITANTES**                                                                  |                                               |                            |
| 3 min           | ¿Cómo les gustaría que los visitantes exploren los libros? ¿Por categorías, búsqueda, recomendaciones?     | Diseñar experiencia de navegación             |                            |
| 2 min           | ¿Qué significa "hojear" para ustedes? ¿Ver portada, índice, algunas páginas de muestra?                    | Definir funcionalidad de preview              |                            |
| 3 min           | ¿Consideran importante mostrar reseñas de clientes, valoraciones, o libros relacionados?                   | Evaluar funciones de recomendación            |                            |
| 2 min           | ¿Hay información que NO quieren mostrar hasta que el usuario se registre?                                  | Establecer niveles de acceso                  |                            |
| **8 min**       | **REGISTRO Y CUENTAS DE CLIENTE**                                                                          |                                               |                            |
| 3 min           | ¿Qué información necesitan del cliente para procesar un pedido? ¿Solo datos de envío y facturación?        | Definir campos obligatorios de registro       |                            |
| 2 min           | ¿Prefieren un registro simple y rápido o más detallado con preferencias de cocina?                         | Determinar complejidad del registro           |                            |
| 3 min           | ¿Les gustaría que los clientes puedan ver su historial de compras, pedidos favoritos, o listas de deseos?  | Evaluar funcionalidades del panel de usuario  |                            |
| **10 min**      | **PROCESO DE COMPRA Y CARRITO**                                                                            |                                               |                            |
| 3 min           | ¿Cómo manejan actualmente el stock? ¿Tienen ejemplares únicos o múltiples copias?                          | Entender gestión de inventario                |                            |
| 2 min           | ¿Qué pasa si un cliente quiere comprar un libro que se agota mientras está en su carrito?                  | Definir manejo de stock en tiempo real        |                            |
| 3 min           | ¿Calculan gastos de envío por peso, destino, o tienen tarifa fija? ¿Hay envío gratuito en algunos casos?   | Establecer lógica de costos de envío          |                            |
| 2 min           | ¿Permiten que los clientes guarden productos en el carrito para comprar después?                           | Evaluar persistencia del carrito              |                            |
| **12 min**      | **PAGOS Y TRANSACCIONES**                                                                                  |                                               |                            |
| 3 min           | ¿Con qué tarjetas de crédito trabajan actualmente? ¿Visa, MasterCard, American Express?                    | Definir métodos de pago aceptados             |                            |
| 2 min           | ¿Han pensado en usar servicios como PayPal, Stripe, o similar para procesar pagos?                         | Evaluar plataformas de pago                   |                            |
| 3 min           | ¿Cuándo se cobra al cliente? ¿Al hacer el pedido, al enviar, o tienen alguna política especial?            | Establecer timing de cobro                    |                            |
| 2 min           | ¿Cómo manejarían las devoluciones o cancelaciones de pedido?                                               | Definir políticas de devolución               |                            |
| 2 min           | ¿Necesitan generar facturas automáticas o recibos especiales?                                              | Identificar requerimientos de facturación     |                            |
| **8 min**       | **GESTIÓN DE PEDIDOS Y LOGÍSTICA**                                                                         |                                               |                            |
| 3 min           | ¿Cómo es su proceso actual de empaque y envío? ¿Quién lo maneja?                                           | Entender flujo logístico                      |                            |
| 2 min           | ¿Les gustaría que los clientes puedan rastrear sus pedidos? ¿Recibir notificaciones del estado?            | Evaluar seguimiento de envíos                 |                            |
| 3 min           | ¿Cómo organizarían los pedidos internamente? ¿Necesitan reportes, etiquetas de envío automáticas?          | Definir herramientas administrativas          |                            |
| **10 min**      | **ADMINISTRACIÓN Y GESTIÓN DEL SISTEMA**                                                                   |                                               |                            |
| 3 min           | ¿Quién se encargará de administrar la tienda online día a día? ¿Ambos o uno de ustedes?                    | Identificar usuarios administradores          |                            |
| 2 min           | ¿Qué nivel de conocimiento técnico tienen? ¿Necesitarían capacitación para usar el sistema?                | Evaluar requerimientos de usabilidad          |                            |
| 3 min           | ¿Qué reportes les serían útiles? ¿Ventas, productos más buscados, clientes frecuentes?                     | Definir necesidades de reporting              |                            |
| 2 min           | ¿Les gustaría poder hacer ofertas, descuentos, o promociones especiales?                                   | Evaluar funcionalidades de marketing          |                            |
| **6 min**       | **ASPECTOS TÉCNICOS Y SEGURIDAD**                                                                          |                                               |                            |
| 2 min           | ¿Tienen preferencias sobre el aspecto visual de la tienda? ¿Algo que refleje la tradición de cocina?       | Entender expectativas de diseño               |                            |
| 2 min           | ¿Es importante que funcione bien en celulares y tablets, o principalmente en computadora?                  | Definir prioridades de dispositivos           |                            |
| 2 min           | ¿Qué nivel de seguridad esperan, especialmente para los datos de tarjetas de crédito?                      | Identificar requerimientos de seguridad       |                            |
| **5 min**       | **COMUNICACIÓN CON CLIENTES**                                                                              |                                               |                            |
| 2 min           | ¿Cómo les gustaría comunicarse con los clientes? ¿Email, teléfono, chat en línea?                          | Establecer canales de atención                |                            |
| 3 min           | ¿Necesitan enviar newsletters, notificaciones de nuevos libros, o ofertas por email?                       | Evaluar herramientas de marketing directo     |                            |
| **6 min**       | **CIERRE Y CONSIDERACIONES ESPECIALES**                                                                    |                                               |                            |
| 2 min           | Como son libros especializados y únicos, ¿hay alguna característica especial que debería tener el sistema? | Capturar requerimientos específicos del nicho |                            |
| 2 min           | ¿Cuáles serían las 3 funcionalidades más importantes para que el sistema sea exitoso?                      | Priorizar funcionalidades críticas            |                            |
| 2 min           | ¿Hay alguna preocupación específica sobre el cambio del correo tradicional al mundo digital?               | Identificar resistencias o temores            |                            |

*Tiempo total estimado: 85 minutos*

# Cuestionarios

## Definiciones

1. Mediante cuestionarios se busca recabar requerimientos, recolectar hechos de un gran número de personas, detectar un sentimiento generalizado, detectar problemas entre usuarios o cuantificar respuestas.
2. Considero apropiado utilizar cuestionarios cuando las personas están dispersas geográficamente, cuando hay muchas personas involucradas, caundo queremos obtener opinines generales p queremos identificar problemas generales.
3. Los cuestionarios pueden tener dos tipos de preguntas: **abiertas**: son las que dejan abietas todas las posibles opciones de respuesta; **cerradas**: son las que limitan o cierran las opciones de respuestas disponibles.

## Situaciones

**Situación 1**

Es necesario reescribir las preguntas, porque hacen suposiciones y contiene preguntas irrelevantes.