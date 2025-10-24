# Entity vs Value Object

## Entity

- Las entidades tienen identificador, son modificables y comparables **por identidad**
- Comparables por contenido, **no tienen identificador**
- Necesitan una entidad base, son intercambiables y **persisten adjunto a su base**
- Inmutables (sin setters)

## Value Object

*Cuando necesitamos i.e: Moneda, Fecha, Direccion, que puedan tener cierto coportamiento (getters)*

- Inmutables, caso contrario **no es Value Object**
- Label: <<value object>>

## Heuristicas de Diseño

> Principios SOLID