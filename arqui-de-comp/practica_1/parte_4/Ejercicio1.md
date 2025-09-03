|   U.   |       Codigo       | Registro | Pila  | valor | Referencia |
| :----: | :----------------: | :------: | :---: | :---: | :--------: |
| **a.** |    **mov ax,5**    |  **x**   |       | **x** |            |
| **a.** | **call subrutina** |  **x**   |       | **x** |            |
|   b.   |  mov dx, offset A  |    X     |       |       |     X      |
|   b.   |   call subrutina   |    X     |       |       |     X      |
| **c.** |   **mov bx, 5**    |          | **x** | **x** |            |
| **c.** |    **push bx**     |          | **x** | **x** |            |
| **c.** | **call subrutina** |          | **x** | **x** |            |
| **c.** |     **pop bx**     |          | **x** | **x** |            |
|   d.   |  mov cx, offset A  |          |   X   |       |     X      |
|   d.   |      push cx       |          |   X   |       |     X      |
|   d.   |   call subrutina   |          |   X   |       |     X      |
|   d.   |       pop cx       |          |   X   |       |     X      |
| **e.** |   **mov dl, 5**    |  **x**   |       | **x** |            |
| **e.** | **call subrutina** |  **x**   |       | **x** |            |
|   f.   |   call subrutina   |    X     |       |   X   |            |
|   f.   |     mov A, dx      |    X     |       |   X   |            |
