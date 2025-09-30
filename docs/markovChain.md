# Cadena de Markov

## Procesos estocásticos

Una sucesión de observaciones $X_1, X_2, \ldots$ se denomina **proceso estocástico** cuando:

- Si los valores de estas observaciones no se pueden predecir exactamente
- Pero se pueden especificar las probabilidades para los distintos valores posibles en cualquier instante de tiempo

  $$\boxed{X_1:\ \text{v.a. que define el \textbf{estado inicial del proceso}}}$$

  $$\boxed{X_n:\ \text{v.a. que define el \textbf{estado del proceso en el instante de tiempo} n}}$$

Para cada posible valor del estado inicial $s_1$ y para cada uno de los sucesivos
valores $s_n$ de los estados $X_n$, $n =2,3,...,$ especificamos:

$$\boxed{P(X_{n+1} = s_{n+1} | X_1 = s_1, X_2 = x_2,\ldots, X_n=s_n)}$$

## Cadenas de Markov

Una cadena de Markov es un proceso estocástico en el que

$$\boxed{\text{Si el estado actual } X_n \text{ y los estados previos } X_1,\ldots, X_{n-1} \text{ son conocidos}}$$

$$\Huge \Downarrow$$

$$
\boxed{\text{La probabilidad del estado futuro }X_{n+1} }
$$

- No depende de los estados anteriores $X_1, \ldots, X_{n-1},$
  y
- Solamente depende del estado actual $X_n$

Es decir,

- Para $n=1,2, \ldots$ y
- Para cualquier sucesión de estados $s_1, \ldots, s_{n+1}$

$$\boxed{P(X_{n+1} = s_{n+1} | X_1 = s_1, X_2 = s_2, \ldots, X_n = s_n)=P(X_{n+1} = s_{n+1} | X_n = s_n)}$$

### Ejemplo

Consideremos que en un locutorio telefónico con 5 líneas de teléfono en
un instante de tiempo dado puede haber un número cualquiera de líneas
ocupadas. Durante un periodo de tiempo se observan las líneas telefónicas
a intervalos de 2 minutos y se anota el número de líneas ocupadas en cada
instante.

- Sea $X_1$ la v.a. que representa el número de líneas ocupadas al principio del
  periodo.
- Sea $X_2$ la v.a. que representa el número de líneas ocupadas cuando se
  observa en el segundo instante de tiempo, 2 minutos más tarde.
- En general, $n =1,2,\ldots X_n$ es una v.a. que representa el número de
  líneas ocupadas cuando se observan en el instante de tiempo $n$−ésimo.
- El estado del proceso en cualquier instante de tiempo es el número de líneas
  que están siendo utilizadas en ese instante.
- Un proceso estocástico como el que acabamos de describir se llama **proceso
  de parámetro discreto**, ya que las líneas se observan en puntos discretos a lo largo del tiempo.

$$
\boxed{
\begin{gathered}
\text{Para que el proceso estocástico del número} \\
\text{de líneas ocupadas sea una cadena de Markov} \\
\text{es necesario que la probabilidad de cada posible número de} \\
\text{líneas ocupadas en cualquier instante de tiempo} \\
\text{dependa solamente del número de} \\
\text{líneas ocupadas 2 minutos antes.}
\end{gathered}
}
$$

## Cadenas de Markov finitas con probabilidades de transición estacionarias

### Cadena de Markov finita

Es una cadena de Markov para la que existe sólo un número finito k de
estados posibles $s_1,\ldots,s_k$ y en cualquier instante de tiempo la cadena está
en uno de estos k estados

### Probabilidad de transición

Es la probabilidad condicionada

$$\boxed{P(X_{n+1} = s_j | X_n = s_i)}$$

### Probabilidad de transición estacionaria

Una cadena de Markov tiene **probabilidades de transición estacionarias** si para cualquier par de estados $s_i$ y $s_j$ existe una probabilidad de transición $p_{ij}$ tal que

$$\boxed{P(X_{n+1} = s_j | X_n = s_i) = p_{ij} \text{ para } n = 1,2,\ldots}$$

## Matriz de transición

### Matriz estocástica

Es una matriz cuadrada cuyos elementos son no negativos y tal que la suma de los elementos de cada fila es 1.

### Matriz de transición en un solo paso

Dada una cadena de Markov con $k$ estados posibles $s_1, s_2, \ldots, s_k$ y con probabilidades de transición estacionarias.

$$
\boxed{\text{Si } p_{ij} = P(X_{n+1} = s_j | X_n = s_i)}
\Rightarrow
\boxed{P = \begin{pmatrix}
p_{11} & \ldots &p_{12} \\
p_{21} & \ldots &p_{22} \\
\vdots &  & \vdots \\
p_{k1} & \ldots &p_{kk}
\end{pmatrix}
}
$$

$$
\boxed{
\begin{gathered}
\text{ La matriz de transición P de cualquier cadena de Markov finita con} \\
\text{probabilidades de transición estacionarias es una matriz estocástica}
\end{gathered}
}
$$

### Ejemplo

Supongamos que el clima de una determinada región sólo puede ser soleado
$(s_1)$ o nublado $(s_2)$ y que las condiciones del clima en mañanas sucesivas forman
una cadena de Markov con probabilidades de transición estacionarias. La matriz
de transición está dada por:

$$
\boxed{
P = \begin{pmatrix}
0.7 & 0.3 \\
0.6 & 0.4
\end{pmatrix}
}
$$

Si un día concreto está nublado, ¿cuál es la probabilidad de que esté nublado
el día siguiente?

$$
\boxed{p_{22} = 0.4}
$$

### Matriz de transición en varios pasos

$$
\boxed{
  \begin{gathered}
  \text{Dada una cadena de Markov con } k \text{ estados posibles } s_1, s_2, \ldots, s_k \\
  \text{y matriz de transición } P \\
  \text{Si notamos } p_{ij}^{(2)} = P(X_{n+2} = s_j | X_n = s_i) \\
  \end{gathered}
}
$$

$$\Huge \Downarrow$$

$$
\boxed{
\begin{aligned}
\bullet \quad & p_{ij}^{(2)} : \text{ Elemento de la } i\text{-ésima fila y } j\text{-ésima columna de la matriz } P^2 \\
\bullet \quad & P^m : \text{ Potencia } m\text{-ésima de } P, \text{ con } (m = 2, 3, \ldots) \\
\bullet \quad & p_{ij}^{(m)} : \text{ Elemento de la fila } i \text{ y de la columna } j \text{ de la matriz } P^m
\end{aligned}
}
$$

### Generalizando

$$
\boxed{
  \begin{gathered}
  P_m \text{ es la matriz de probabilidades } p_{ij}^{(m)} \text{ de la que la cadena pasa del estado } s_i \\
   \text{al estado } s_j \text{ en } m \text{ pasos; para cualquier valor de } m, (m=2,3,...). \\
   P^m \text{ es la matriz de transición de } m \text{ pasos de la cadena de Markov.}
  \end{gathered}
}
$$

### Ejemplo

En el ejemplo del clima con matriz de transición

$$
\boxed{
P = \begin{pmatrix}
0.7 & 0.3 \\
0.6 & 0.4
\end{pmatrix}
}
$$

Si un miercoles está nublado, ¿cuál es la probabilidad de que el viernes siguiente haga sol?

- Calculamos la matriz de transición en dos pasos:

$$
\boxed{
P^2 = \begin{pmatrix}
0.67 & 0.33 \\
0.66 & 0.34
\end{pmatrix}
\Longrightarrow
\text{Probabilidad pedida es } 0.66
}
$$

## Vector de probabilidades iniciales

### Vector de probabilidades

$$
\boxed{
\begin{gathered}
w = (w_1, w_2, \ldots, w_k) \text{ se llama } \textbf{vector de probabilidades} \text{ si} \\
\bullet \quad w_i \geq 0 \text{ para } i = 1, 2, \ldots, k \text{ y } \\
\bullet \quad \sum_{i=1}^{k} w_i = 1
\end{gathered}
}
$$

Consideramos una cadena de Markov con:

1. $s_1, \ldots, s_k$ posibles estados en los que la cadena puede estar en el tiempo de observación inicial $n = 1$

2. Para $i = 1, \ldots, k; P(X_1 = s_i) = v_i,$ con $v_i \geq 0$ y $v_1 + \ldots + v_k = 1$

### Vector de probabilidades iniciales

El vector de probabilidades $v = (v_1, v_2, \ldots, v_k)$ se llama **vector de probabilidades iniciales** de la cadena de Markov.

$$
\boxed{
  \begin{gathered}
  \text{El vector de probabilidades iniciales y } \\
  \text{la matriz de transición determinan la probabilidad} \\
  \text{ para el estado de la cadena en el segundo instante de tiempo,} \\
  \text{ dicha probabilidad viene dada por el vector } vP
  \end{gathered}
}
$$

- Además, si las posibilidades de los diversos estados en el instante de $n$ se especifican por el vector de probablidades $w$, entonces

$$
\boxed{
  \begin{gathered}
\text{ Las probabilidades en el instante n +1} \\
\text{ se especifican por el vector de probabilidades }wP
  \end{gathered}
}
$$

### Ejemplo

En el ejemplo del clima con matriz de transición

$$
\boxed{
P = \begin{pmatrix}
0.7 & 0.3 \\
0.6 & 0.4
\end{pmatrix}
}
$$

- Suponemos que la probabilidad de que el miercoles haga sol es 0.2 y la probabilidad de que esté nublado es 0.8.

Calcular:

1. Probabilidad de que esté nublado el jueves.
2. Probablidad de que esté nublado el viernes.
3. Probablidad de que esté nublado el sábado.

### Solución

1. Miercoles: $v = (0.2, 0.8) \Longrightarrow w = vP = (0.62, 0.38)$

$$\boxed{P [\text{esté nublado el jueves}] = 0.38}$$

2. $w = vP = (0.62, 0.38) \Longrightarrow vP^2= vPP = wP = (0.662, 0.338)$

$$\boxed{P [\text{esté nublado el viernes}] = 0.338}$$

4. $vP^{2} = (0.662, 0.338) \Longrightarrow vP^{3} = vP^{2}P = (0.6662, 0.3338)$

$$
\boxed{P[\text{ esté nublado el sábado }] = 0.3338}
$$

### Ejemplo

Suponemos que en el ejemplo del locutorio telefónico los números de líneas
que están siendo utilizadas en los instantes de tiempo $1, 2,\ldots$ constituyen
una cadena de Markov con probabilidades de transición estacionarias.

Sea $b_1$ el estado en el que están utilizando exactamente $i$ líneas, en un instante de tiempo determinado $(i = 0, 1, \ldots, 5)$.

#### Matriz de transición P

$$
P = \begin{pmatrix}
 0.1 & 0.4 & 0.2 & 0.1 & 0.1 & 0.1 \\
 0.2 & 0.3 & 0.2 & 0.1 & 0.1 & 0.1 \\
 0.1 & 0.2 & 0.3 & 0.2 & 0.1 & 0.1 \\
 0.1 & 0.1 & 0.2 & 0.3 & 0.2 & 0.1 \\
 0.1 & 0.1 & 0.1 & 0.2 & 0.3 & 0.2 \\
 0.1 & 0.1 & 0.1 & 0.1 & 0.4 & 0.2
\end{pmatrix}
$$

- Si las cinco líneas están ocupadas en un instante de tiempo concreto.

$$\boxed{P\text{[Exactamente 4 líneas ocupadas en el siguiente instante de tiempo]}=p_{65}=0.4}$$

- Si en un instante de tiempo no hay ninguna línea ocupada.

$$\boxed{P\text{[Al menos una línea ocupada en el siguiente instante de tiempo]}=1−p_{11}=0.9}$$

#### Matriz de transición en dos pasos

$$
P^2 = \begin{pmatrix}
  0.14 & 0.23 & 0.2 & 0.15 & 0.16 & 0.12 \\
 0.13 & 0.24 & 0.2 & 0.15 & 0.16 & 0.12 \\
 0.12 & 0.2 & 0.21 & 0.18 & 0.17 & 0.12 \\
 0.11 & 0.17 & 0.19 & 0.2 & 0.2 & 0.13 \\
 0.11 & 0.16 & 0.16 & 0.18 & 0.24 & 0.15 \\
 0.11 & 0.16 & 0.15 & 0.17 & 0.25 & 0.16 \\
\end{pmatrix}
$$

- Si dos líneas están ocupadas en un instante de tiempo concreto.

$$
\boxed{
   P\text{[Cuatro líneas ocupadas dos instantes después]} =0.17
}
$$

- Si en un instante de tiempo concreto hay tres líneas ocupadas.

$$
\boxed{
  P\text{[Dos instantes después hay de nuevo tres líneas ocupadas]}=0.2
}
$$

$$
P^3 = \begin{pmatrix}
   0.123 & 0.208 & 0.192 & 0.166 & 0.183 & 0.128 \\
   0.124 & 0.207 & 0.192 & 0.166 & 0.183 & 0.128 \\
   0.120 & 0.197 & 0.192 & 0.174 & 0.188 & 0.129 \\
   0.117 & 0.186 & 0.186 & 0.179 & 0.199 & 0.133 \\
   0.116 & 0.181 & 0.177 & 0.176 & 0.211 & 0.139 \\
   0.116 & 0.180 & 0.174 & 0.174 & 0.215 & 0.141 \\
\end{pmatrix}
$$

- Si las 5 líneas están ocupadas en un instante de tiempo concreto.

$$\boxed{P[\text{No hay líneas ocupadas tres instantes después}] = 0.116}$$

- Si una línea está ocupada en un instante de tiempo.

$$\boxed{P[\text{Tres instantes después haya de nuevo una línea ocupada}] = 0.207}$$

- Al inicio del proceso de observación (instante $n = 1$)

$$\boxed{P [ \text{No haya líneas ocupadas}] = 0.5}$$

$$\boxed{P [ \text{Haya una línea ocupada}] = 0.3}$$

$$\boxed{P [ \text{Haya dos líneas ocupadas}] = 0.2}$$

$$\boxed{\text{Vector de probabilidades iniciales: }v =(0.5,0.3,0.2,0,0,0)}$$

- $vP =(0.13, 0.33, 0.22, 0.12, 0.1, 0.1)$

$$\boxed{P[\text{No haya líneas ocupadas en el instante 2}] = 0.13}$$

- $vP^2 = vPP = (0.1333, 0.227, 0.202, 0.156, 0.162, 0.12)$

$$\boxed{P[\text{Haya 2 líneas ocupadas en el instante 3}] = 0.202}$$

## Distribucion estacionaria

Una distribución de probabilidad $\pi = (\pi_1, \pi_2, \ldots, \pi_k)$ es una **distribución estacionaria** de una cadena de Markov con matriz de transición $P$ si
$$\boxed{\pi P = \pi}$$
con
$$\boxed{\pi_1 + \pi_2 + \ldots + \pi_k = 1}$$

### Ejemplo

Calcular la distribución estacionaria de la cadena de Markov del locutorio telefónico con matriz de transición

$$
P = \begin{pmatrix}
 0.1 & 0.4 & 0.2 & 0.1 & 0.1 & 0.1 \\
 0.2 & 0.3 & 0.2 & 0.1 & 0.1 & 0.1 \\
 0.1 & 0.2 & 0.3 & 0.2 & 0.1 & 0.1 \\
 0.1 & 0.1 & 0.2 & 0.3 & 0.2 & 0.1 \\
 0.1 & 0.1 & 0.1 & 0.2 & 0.3 & 0.2 \\
 0.1 & 0.1 & 0.1 & 0.1 & 0.4 & 0.2
\end{pmatrix}
$$

La distribución estacionaria $/pi = (\pi_1, \pi_2, \pi_3, \pi_4, \pi_5, \pi_6)$, cumpliendo que $/pi P = /pi$, esto es:

$$
\begin{pmatrix}
\pi_1, \pi_2, \pi_3, \pi_4, \pi_5, \pi_6
\end{pmatrix}
\begin{pmatrix}
 0.1 & 0.4 & 0.2 & 0.1 & 0.1 & 0.1 \\
 0.2 & 0.3 & 0.2 & 0.1 & 0.1 & 0.1 \\
 0.1 & 0.2 & 0.3 & 0.2 & 0.1 & 0.1 \\
 0.1 & 0.1 & 0.2 & 0.3 & 0.2 & 0.1 \\
 0.1 & 0.1 & 0.1 & 0.2 & 0.3 & 0.2 \\
 0.1 & 0.1 & 0.1 & 0.1 & 0.4 & 0.2
\end{pmatrix}
=
\begin{pmatrix}
\pi_1, \pi_2, \pi_3, \pi_4, \pi_5, \pi_6
\end{pmatrix}
$$

y que $\pi_1 + \pi_2 + \pi_3 + \pi_4 + \pi_5 + \pi_6 = 1$

Si llamamos I a la matriz identidad, la ecuación $\pi P = \pi$ se puede expresar como:

$$\boxed{\pi (P - I) = 0}$$

En este caso, la matriz $P - I$ es:

$$
P - I =
\begin{pmatrix}
 -0.9 & 0.4 & 0.2 & 0.1 & 0.1 & 0.1 \\
 0.2 & -0.7 & 0.2 & 0.1 & 0.1 & 0.1 \\
 0.1 & 0.2 & -0.7 & 0.2 & 0.1 & 0.1 \\
 0.1 & 0.1 & 0.2 & -0.7 & 0.2 & 0.1 \\
 0.1 & 0.1 & 0.1 & 0.2 & -0.7 & 0.2 \\
 0.1 & 0.1 & 0.1 & 0.1 & 0.4 & -0.8
\end{pmatrix}
$$

Por tantos todas las filas suman 0; es fácil ver que el rango de P - I es 5.

Ello significa que en el sistem $/pi (P - I) = 0$ solo hay dos ecuaciones independientes.

Si nos quedamos con las dos primeras y le añadimos la condición $\pi_1 + \pi_2 + \pi_3 + \pi_4 + \pi_5 + \pi_6 = 1$, obtenemos el sistema:

$$
\begin{cases}
-0.9 \pi_1 + 0.2 \pi_2 + 0.1 \pi_3 + 0.1 \pi_4 + 0.1 \pi_5 + 0.1 \pi_6 = 0 \\
0.4 \pi_1 - 0.7 \pi_2 + 0.2 \pi_3 + 0.1 \pi_4 + 0.1 \pi_5 + 0.1 \pi_6 = 0 \\
0.2 \pi_1 + 0.2 \pi_2 - 0.7 \pi_3 + 0.2 \pi_4 + 0.1 \pi_5 + 0.1 \pi_6 = 0 \\
0.1 \pi_1 + 0.1 \pi_2 + 0.2 \pi_3 - 0.7 \pi_4 + 0.2 \pi_5 + 0.1 \pi_6 = 0 \\
0.1 \pi_1 + 0.1 \pi_2 + 0.1 \pi_3 + 0.2 \pi_4 - 0.7 \pi_5 + 0.2 \pi_6 = 0 \\
\pi_1 + \pi_2 + \pi_3 + \pi_4 + \pi_5 + \pi_6 = 1
\end{cases}
$$

Expresando nuevamente el sistema de forma matricial:

$$
\begin{cases}
-0.9 \pi_1 + 0.2 \pi_2 + 0.1 \pi_3 + 0.1 \pi_4 + 0.1 \pi_5 + 0.1 \pi_6 = 0 \\
0.4 \pi_1 - 0.7 \pi_2 + 0.2 \pi_3 + 0.1 \pi_4 + 0.1 \pi_5 + 0.1 \pi_6 = 0 \\
0.2 \pi_1 + 0.2 \pi_2 - 0.7 \pi_3 + 0.2 \pi_4 + 0.1 \pi_5 + 0.1 \pi_6 = 0 \\
0.1 \pi_1 + 0.1 \pi_2 + 0.2 \pi_3 - 0.7 \pi_4 + 0.2 \pi_5 + 0.1 \pi_6 = 0 \\
0.1 \pi_1 + 0.1 \pi_2 + 0.1 \pi_3 + 0.2 \pi_4 - 0.7 \pi_5 + 0.2 \pi_6 = 0 \\
\pi_1 + \pi_2 + \pi_3 + \pi_4 + \pi_5 + \pi_6 = 1
\end{cases}
\Longrightarrow
\begin{pmatrix}
\pi_1, \pi_2, \pi_3, \pi_4, \pi_5, \pi_6
\end{pmatrix}
\begin{pmatrix}
 -0.9 & 0.4 & 0.2 & 0.1 & 0.1 & 1\\
 0.2 & -0.7 & 0.2 & 0.1 & 0.1 & 1\\
 0.1 & 0.2 & -0.7 & 0.2 & 0.1 & 1\\
 0.1 & 0.1 & 0.2 & -0.7 & 0.2 & 1\\
 0.1 & 0.1 & 0.1 & 0.2 & -0.7 & 1\\
 0.1 & 0.1 & 0.1 & 0.1 & 0.2 & 1
\end{pmatrix}
=
\begin{pmatrix}0, 0, 0, 0, 0, 1\end{pmatrix}
$$

obtenemos que su solución (distribución estacionaria) es:
$$\boxed{\pi = (0.1261, 0.1899, 0.1824, 0.1709, 0.1924, 0.1380)}$$

Podemos comprobar que efectivamente, $\pi P = \pi$:

$$
\begin{pmatrix}
0.1261, 0.1899, 0.1824, 0.1709, 0.1924, 0.1380
\end{pmatrix}
\begin{pmatrix}
 0.1 & 0.4 & 0.2 & 0.1 & 0.1 & 0.1 \\
 0.2 & 0.3 & 0.2 & 0.1 & 0.1 & 0.1 \\
 0.1 & 0.2 & 0.3 & 0.2 & 0.1 & 0.1 \\
 0.1 & 0.1 & 0.2 & 0.3 & 0.2 & 0.1 \\
 0.1 & 0.1 & 0.1 & 0.2 & 0.3 & 0.2 \\
 0.1 & 0.1 & 0.1 & 0.1 & 0.4 & 0.2
\end{pmatrix}
=
\begin{pmatrix}
0.1261, 0.1899, 0.1824, 0.1709, 0.1924, 0.1380
\end{pmatrix}
$$
