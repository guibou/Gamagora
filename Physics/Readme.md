On va s'interesser à la chute d'un point, d'une particule.


- La gravité.
- Accelleration
- Les forces
- La trajectoire (Vecteur)
- La masse

- Position:
  - Choisir un repere


- La position c'est `P`
- La variation de la *position* est la *vitesse*
- La variation de la *vitesse* est l'*accelleration*.

- La variation c'est la derivée

Equations:

.. math::
  
  dP / dt = V
  dV / dt = A

  P' = V
  V' = A

Si on veux affichez un personnage au cours du temps, il nous faut sa fonction `P(t)`.

# Cas simples


Si la vitesse est constante `V(t) = v`.

Alors l'accéleration est nulle `A(t) = 0`.

Alors, la position se calcul trivialement:

`P(t) = P(0) + t * v`

Si l'acceleration n'est pas nulle, mais constant, `A(t) = a`, alors:

```
V(t) = V(0) + t * a
```

Comment on calcul la position?

```
P'(t) = V(t)
P(t) = Integrale(V(t), dt) + P(0)
P(t) = Integrale(V(0) + t * a, dt) + P(0)

P(t) = V(0) * t + t ^ 2 / 2 * a + P(0)
```

## Petit jeu de canon


```
P(0) = (0, 0)
V(0) = (100, 100)

a = (0, -1)
```

## Cas pas simple

```
A(t) = F(P(t), V(t), t)
```

# Simulation numérique

Schema d'integration numérique:

```
P(t + dt) = P(t) + dt * V(t) + Erreur(dt)
V(t + dt) = V(t) + dt * A(t) + Erreur(dt)
A(t) = Mesure de l'acceleration
```

```
P(0) = (0, 0)
V(0) = (100, 100)

a = (0, -1)
```

# Comment faire des bonnes steps de simulation

a) Choisir une valeur pour `dt` qui marche bien
b) NE PAS UTILISER le temps entre chaque frame pour dt
c) A chaque "frame", faire avancer de `N` etapes entière de `dt` fixe.

Example:

dt = 0.1
temps_depuis_dernière frame: 0.34

alors: 3 step de simulation à dt = 0.1
"garde en mémoire" un retard de "0.04"

Attention: si "grand" retard, genre 0.09 (e.g proche de `dt`).

Un peu plus avancé.

faire 3 step à dt = 0.1 -> état que on GARDE!
faire 1 step à dt = 0.04 -> état que on affiche
On conserve le retard de 0.04, pour après.

Frame d'après: 0.19

0.19 + 0.04 = 0.23

- Faire 2 steps à dt = 0.1 -> Etat que garde
- Faire 1 step à dt = 0.03 -> etat affiché
On conserve le retard de 0.03 pour après.

# Mesure de l'acceleration

Newton.

Sommes des forces = acceleration * masse

=>

acceleration = (sommes des forces) / masse

# Deux forces en jeu

## Gravité

Gravité avec Z vers le haut:

```
F_g = m * g * (0, 0, -1)
```

- `m`: masse
- `g`: constante d'acceleration gravitiotanel de la terre

## Résistance à l'air

De quoi dépend la résistance à l'air?

- Densité de l'air (et donc indirectemnt l'altitude): `rho`
- Volume du corps: en fait, la surface projectée!: `A`
  (examples):
    - Parachutiste à plat: 200 km/h
    - Parachutiste tête en bas: 300 km/h (Record à 500 km/h)
- Type de surface: Coeficient de Drag: `C_d`
- La vitesse: `v`

A un moment donnée, la resistance à l'air s'oppose completement à la gravité. Le parachutiste n'accellere plus!

```
F_d = - 1/2 * C_d * A * rho * ||v||^2 * v_u
F_d = - Coef * ||v||^2 * v_u 
```

(Avec `Coef` une valeur magique qui contient tout).

Avec `v_u` le vecteur unitaire de la vitesse (e.g. la direction de déplacement).

(pour les ailes d'avion: Navier-stokes)

# Job pendant la procaine séance:

- Ecrire la simulation d'un parachutiste qui part de:

- `P(0)`: 4000m
- `V(0)`: (200 km /h, 0, 0)
- `m`: 80 kg
- `g`: 10 m / s / s
- `Coef`: faire des essais et "calibrer".

## Calcul du coef

- Simuler / essai erreur, et calibrer en fonction de la vitesse max observée.
- Faire des maths:

A un moment donnée, l'acceleration devient nulle.

Hors, on sait que `a = (somme des forces) / masse`

=> `somme des forces = 0`
=> `mg * (0, 0, -1) - Coef * ||v||^2 * v_u = 0`

On sait que `(0, 0, -1) = v_u`

=> `mg - Coef * || v || ^ 2 = 0`
=> `Coef = mg / || v || ^ 2`

## Calcul du vecteur unitaire `v_u`

```
v_u = V / || V ||
```
Il se passe quoi si `V = (0, 0, 0)` => `0 / 0`.

```
F_d = - Coef * ||v||^2 * v_u 
```

si `||v|| = 0` => `F_d = 0`

```
// Attention, j'écris cela pour éviter de diviser par 0 dans le calcul de
normalization et ainsi avoir satan qui apparait sous la forme d'un NaN.

let norm_v = v.normalize()
if norm_v == 0.0
{
  F_d = 0;
}
else
{
   F_d = - Coef * norm_v ^ 2 * v.normalize()
}
```

Reprenons la formule:

```
F_d = - Coef * ||v||^2 * v_u 
```
=>

Remplacons v_u

```
F_d = - Coef * ||v||^2 * v / || v ||
```

=>
```
F_d = - Coef * ||v|| * v
```

Cette equation a plusieurs avantages:

a) Elle est plus simple!
b) Elle Satanise pas en v nul.


===> Conclusion: Quand on a une `/`, même cachée, essayer de la supprimer ou
quoi qu'il arrive, gerer le cas de figure.


