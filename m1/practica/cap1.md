# Conjuntos y Funciones

## Demostraciones

- $\text{Método Directo} \quad (\forall x) (p(x) \to q(x))$    
- $\text{Método Contrarrecíproco} \quad (\forall x) (\neg q(x) \to \neg p(x))$     
- $\text{Método Absurdo} \quad (\forall x) \neg (p(x) \to q(x))$

### Ejercicios

1. Demostrar que la suma de 3 números enteros consecutivos es un múltiplo de 3. (Un múltiplo
    de 3 es un número que puede escribirse como 3 por un número entero: si 𝑎 es múltiplo de 3
    entonces 𝑎 = 3ℎ, ℎ entero)
2. Si el cuadrado de un número entero 𝑤 es par , el cuadrado del anterior a 𝑤 es impar.

3. La suma de 3 números enteros consecutivos, si el primero es impar, es múltiplo de 6.

4. Recordemos que un número racional o fraccionario es aquel que puede expresarse como
    cociente de enteros, es decir si 𝑥 = 𝑎
    𝑏 y 𝑏 ≠ 0 , decimos que 𝑥 es un número racional. Un número
    es irracional si no puede escribirse como cociente de enteros, por ejemplo: √2, 𝜋, √5.
    Demostrar que la suma de un número racional y un irracional es un número irracional.

---

1. La suma de 3 numeros enteros consecutivos puede definirse como: $ x + (x+1) + (x+2) $ que es lo mímso que decir $ 3x + 3 $. Luego sacamos factor común (3) y nos queda $ 3(x+1) $ como $x$ es un número entero, sumarle 1 también da como resultado otro número entero. Si a este nuevo número lo llamamos $h$, la expresión se convierte exactamente en $3h$.

2. Si definimos por el método directo y tomamos como verdadera nuestra hipótesis: $w²$ es par, entoneces podemos definir $w=2h$ siendo $h$ un número entero. Por lo tanto podemos definir *"el cuadrado anterior a $w$"* como $(w-1)²$. Si reemplazamos $w$ por $2h$ nos queda $(2h-1)²$ y luego de desarrollar el cuadrado quedaría $(2h)² + 2*2h*1 + (-1)² \implies 4h² - 4h + 1$. Se puede expresar un número impar como $2*(\text{un numero entero}) + 1$, para llegar a esa forma sacamos factor común $2(2h² - 2h) + 1$ y llamamos al bloque $(2g² - 2h)$ como $k$, entonces la expresión se vuelve $2k+1$
 
> Fórmula del binomio del cuadrado: $(a-b)^2 = a^2 - 2ab + b²$

**Hipótesis:** $w² \text{ es par}$  
**Tesis:** $(w-1)² \text{ es impar}$

$$w² \text{ es par} \implies w \text{ es par}$$
$$\implies w=2h \text{ (con $h \in \mathbb{Z}$)}$$
$$\implies (w-1)² = (2h-1)²$$
$$\implies (w-1)² = 4h² - 4h + 1$$
$$\implies (w-1)² = 2(2h²-2h) + 1$$
$$\text{Como $h$ es un número entero entonces $(2h²-2)$ }$$
$$\text{también es un número entero, entonces defino el bloque $w$}$$
$$\implies (w-1)² = 2k+1 \text{ (con $k \in \Zeta$)}$$
$$\implies (w-1)² = impar$$
$$\blacksquare$$

3. 
$$w = 2k + 1 \text{ es impar}$$
$$\implies w + (w+1) + (w+2)$$
$$\implies 3w+3$$
$$\implies 3*(2k+1)+3$$
$$\implies 6k+6$$
$$\implies 6(k+1)$$
$$\text{(como k es un numero entero, k+1 tambien es un numero entero, lo nombramos $m$)}$$
$$\implies 6m \text{ (con } m \in \mathbb{Z})$$
$$\blacksquare$$

## Conjuntos

$ A=(x,y,t,z)$ (nombre del conjunto / elementos del conjunto)

$ t \in A $ (el elemento *$t$* pertenece al conjunto *$A$*)

$ t \not \in A $ (el elemento *$t$* no pertenece al conjunto *$A$*)

$∅$ (conjunto vacío) 

### Ejercicios

11. Sean $𝐴 = {1,2}$, $𝐵 = {1,2,3,6}$ , $𝐶 = {𝑥: 𝑥 = 2𝑘 ∧ 𝑘 ∈ ℕ}$ , $𝐷 = {𝑥: 𝑥 = 3𝑚 ∧ 𝑚 ∈ ℤ }$ 𝑦 $𝑈 = ℤ$
    1. Expresar por comprensión $𝐴 ∪ ℤ$
    2. Expresar por comprensión $𝐴^𝑐$
    3. Expresar por extensión $𝐴 ∩ 𝐶$
    4. Expresar por extensión $𝐵 − (𝐷 ∩ 𝐴)$
    5. Expresar por comprensión $𝐶^𝑐$
    6. Expresar por comprensión $𝐷^𝑐 ∪ 𝐵^𝑐$, recuerde que por las propiedades mencionadas
    puede calcularlo como $(𝐷 ∩ 𝐵)^𝑐$
    7. Expresar por extensión $(𝐴 ∪ 𝐵) ∩ (𝐴 ∪ 𝐶)$ , según las propiedades enunciadas, ¿de qué
    otra forma podría calcularlo?
    8. Expresar por comprensión $𝐴^𝑐 ∪ 𝐷^𝑐$
    9. Expresar por comprensión $𝐴^𝑐 ∩ 𝐶^𝑐$
12. Sean $𝐴 = {𝑥: 𝑥 = 5𝑤 ∧ 𝑤 ∈ ℤ}$ y $𝐵 = {𝑥: 𝑥 ∈ ℤ ∧ 1 ≤ 𝑥 + 2 ≤ 7}$
    1. Hallar por extensión los conjuntos: $𝐴 ∩ 𝐵$ 𝑦 $𝐵 − 𝐴$
    2. Definir un conjunto $𝐻$, que cumpla que $𝐻 ⊆ 𝐵$
13. Sean los conjuntos $𝐴 = ${$1,2,3,4,5,6$} , $𝐵 = ${$𝑥: 𝑥 = 4𝑘 ∧ 𝑘 ∈ ℤ$} y $𝐶 = ${$3,4,5,6,7,8$}
    1. Hallar y expresar por extensión **i)** $𝐴 ∩ (𝐵 ∪ 𝐶)$ **ii)** $𝐶 − (𝐴 − 𝐵)$.
    2. Hallar un conjunto $𝐷$ que esté incluido en $𝐵$ .
14. Sean $𝑃$ el conjunto de los enteros pares e $𝐼$ el conjunto de los enteros impares y $U = ℤ$
    1. Expresar por comprensión: $𝑃 ∪ 𝐼$, $𝑃 − 𝐼$, $𝐼 − 𝑃$ , $𝑃^𝑐$ , $𝐼^𝑐$
    2. Probar que $𝑃 ∩ 𝐼=Ø$       
**Indicaciones:** probarlo por el absurdo suponiendo que fuera distinto del $Ø$, es decir que
existe un número m que es a la vez elemento de 𝑃 y elemento de 𝐼.
15. Si T es el conjunto de enteros múltiplos de 3 y $C= {𝑥: 𝑥 ∈ ℤ ∧ 𝑥 = 3𝑘 + 1 ∧ 𝑘 ∈ ℤ} 𝑦 𝑈 = ℤ$
    1. Hallar $𝑇 ∩ 𝐶$
    2. Hallar $(𝑇 ∪ 𝐶)^c$

---

13. $𝐴 = ${$1,2,3,4,5,6$} , $𝐵 = ${$𝑥: 𝑥 = 4𝑘 ∧ 𝑘 ∈ ℤ$} y $𝐶 = ${$3,4,5,6,7,8$}
    1. **i)** $A \cap (B \cup C) = $ { $1,2$ } **ii)** $C - (A - B) = ${$4,7,8$}
    2. $D = ${$8k$}

14. Sean $𝑃$ el conjunto de los enteros pares e $𝐼$ el conjunto de los enteros impares y $U = ℤ$
    1. $P \cup I = \mathbb{Z}$, $P - I = P$, $I - P = I$, $P^c = I$, $I^c = P$
    2. Demostracion mediante el absurdo:    
        $m:m \in \mathbb{Z} \land m = 2k \land k  \in \mathbb{Z}$    
        $m:m \in \mathbb{Z} \land m = 2j+1 \land j  \in \mathbb{Z}$     
        $2k = 2j + 1$   
        $2k - 2j = 1$   
        $2(k-j) = 1$      
        $2(m) = 1$     
        $P \cap I = \emptyset$ $\blacksquare$