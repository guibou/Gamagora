# Transformations

Les transformatinos de base sont:

- translation
- rotation
- mise à l'échelle
- projection

Pourquoi transformer ?

# Outils

(Math niveau collège)
- Les trucs simples (ça c'est simple)

(math niveau lycée)
- Les matrices homogenes (ça c'est pas mal)

(math niveau fouuoerueosnahioaouensthaoeu)
- Les quaternions (c'est cool!)


# Shader d'example dans shader toy

```
void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
    vec3 pointC = vec3(fragCoord/iResolution.xy, 0);
    
    vec2 points[3];

    points[0] = vec2(0.5, 0.5);
    points[1] = vec2 (0.5, 1);
    points[2] = vec2 (0.7, 0.7);
    
    int nbPoints = 3;
    bool inside = true;
    
    for(int i = 0; i < nbPoints; i++)
    {
      vec3 pointA = vec3(points[i], 0);
      vec3 pointB = vec3(points[(i+1) % nbPoints], 0);
      vec3 res = cross(pointB - pointA, pointC - pointA);
    
      if(res.z > 0.0)
         inside = false;
    }
    
    
    if(inside)
    {
    
       fragColor = vec4(1, 0, 0, 1);
    }
    else
    {
      fragColor = vec4(0, 1, 0, 1);
    }
}
```

# Translation

- Soit un point `p = (x, y, z)` et une transformation `t = (dx, dy, dz)`, alors on obtient:
- Un exemple, soit `p = (5, 2)`, et `t = (3, 1)`, alors `p' = (8, 3)`.

```
p' = p + t
```

# Scale

- Soit un point `p = (x, y, z)` et une transformation `s = (sx, sy, sz)`, alors on obtient:
- Exemple, soit un point `p = (5, 3)`, et une mise à l'echelle `s = (2, 0.5)`, alors `p' = (10, 1.5)`.

```
p' = p * s
```


# Non communitativé

Les transformations ne sont pas commutatives si pas du même type.

D'abord une transaltion, puis ensuite le scale

```
p' = p + t
p'' = p' * s = p * s + t * s
```

Dans l'autre sens:

```
p' = p * s
p'' = p' + t = p * s + t
```

# Rotation

En 2D:

Coordonnée carthesienne: `(x, y)`
Coordonée polaires: `(rayon, angle)`

Carthésienne to polaire:
   - rayon: `sqrt(x^2 + y^2)`
   - angle: `sin(angle) = y / rayon`
            `cos(angle) = x / rayon`
            `angle = atan2(y, x)`
            `SOHCAHTOA`

Polaire to carthesian:
  - `x = rayon * cos(angle)`
  - `y = rayon * sin(angle)`

Quesqu'e c'est qu'une rotation dans le plan (e.g selon l'axe `z`).

Soit un point `p = (x, y)`, on veut le faire tourner autour du centre d'un angle `dw`.

- On convertit `p` en coordonnée polaire `p = (rayon, w)`
- On ajoute l'angle: `p' = (rayon, w + dw)`
- On reconvertit en cartésienne: `p' = (x', y')`.

# Note sur les aspect ratio

L'écran fait e.g. `(800, 600)`, met on fait le mapping vers `(-1, 1)` sur les
deux axes. Il faut faire autrement pour conserver les proportions.


# C'est l'horreur

Pour 3 transforamtions differents, on a 3 operations mathemathiques differentes:

- Une addittion
- Une multiplitaino
- La rotation c'est deux conversion de base et une addition.


# Ce n'est pas facilement associatif

(Rappel: associativité: `(a x b) x c = a x (b x c)`)

Example:

- l'addition est assioative!
  ```
  (1 + 2) + (3 + 4)
  =
  1 + (2 + (3 + 4))
  =
  ((1 + 2) + 3) + 4
  ```

1 milliard de nombre:

- on coupe en deux et on fait la somme sur chacun des CPU disponibles
- On fait la somme finale

- Youpi, on peut le faire car les réel sont associatifs.

(On va supposer que pour les besoin de cet exercice, les floattants (float,
double) sont associatifs, ce qui, comme vous n'avez plus le droit de l'ignorer
maintenant, est faux!)

# En rendu, on va faire quoi ?

- On va avoir des quantité dingues de points à transformer
- Sur lesquels, on va appliquer pleins de transformations.

- On a un truc qui ressemble à:

```
p' = t(t'(t''(t'''(t''''(t'''''(p))))))
```

- 1 translation
- 1 mise à l'échele

```
p' = (p + translation) * scale
```

Avec une rotation:

```
p'' = convert_to_carthesanio(addAngle(convertToPolar(scale(translate(p)))))
```

Question?

Comment je passe de facon COMPACT la transformation au shader.


-->

```
let transformation = convert_to_carthesanio . addAngle . convertToPolar . scale . translate
# TRANSFORMATION EST COMPACT et rapide à appliquer

p' = transformation(p)
```

Motivation:

- Passer efficacement une transformation
- Appliquer efficasement


## Big shader


```
vec2 translation(vec2 i)
{
    return i + vec2(-0.5 * sin(iTime), -0.5 * sin(iTime));
}

vec2 scale(vec2 i)
{
    return vec2(abs(sin(iTime)), abs(sin(iTime))) * i;
}


void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
    vec3 pointC = vec3(fragCoord/iResolution.xy, 0);
    
    vec2 points[3];

    points[0] = vec2(0.5, 0.5);
    points[1] = vec2 (0.5, 1);
    points[2] = vec2 (0.7, 0.7);
        
    int nbPoints = 3;
    
    // Applique la transformation
    for(int i = 0; i < nbPoints; i++)
    {
        // points[i] = translation(scale(points[i]));
        points[i] = scale(translation(points[i]));
    }
   

    bool inside = true;
    
    for(int i = 0; i < nbPoints; i++)
    {
      vec3 pointA = vec3(points[i], 0);
      vec3 pointB = vec3(points[(i+1) % nbPoints], 0);
      vec3 res = cross(pointB - pointA, pointC - pointA);
    
      if(res.z > 0.0)
         inside = false;
    }
    
    
    if(inside)
    {
    
       fragColor = vec4(1, 0, 0, 1);
    }
    else
    {
      fragColor = vec4(0, 1, 0, 1);
    }

}

```


# Notes de GLSL


- Les vecteurs en GLSL ont pleins de syntax / méthodes sympas.
- On peut convertir de VecX à VexX+ en fournissant un vecteur plus petits et les composantes manquantes:

```
vec2 titi = vec2(1, 2);
vec3 toto = vec3(titi, 3);
// ici toto = vec3(1, 2, 3)

vec4 tutu = vec4(titi, 3, 4);
// ici tutu = vec4(1, 2, 3, 4);
```

- Attention, en GLSL, les literaux sont plus ou moins polymorphics.

`1` c'est un entier, mais des fois cela passe en floatant...
`1.0` c'est un flottant, OU un `vecX` de même taille.

```
vec3(1, 2, 3) + 1.0
~
vec3(1, 2, 3) + vec3(1, 1, 1)
~
vec3(1)
```

- Par défaut, toutes les operations sur des `vecN` sont "termes à termes"

- On peut extraire un sous vecteur / une coordonnée avec `.xyz`.

```
float x = myVec.x;
float y = myVec.y;

vec3 toto = vec3(1,2,3);
vec2 titi = toto.xy;
```

- On peut aussi faire n'importe quoi, comme `.xxxx`, ou `.yzxw`


# Coordonnée homogenes

- Representer TOUTES les transformations par des matrices

On a 3 transformations à representer


## Mise à l'echelle

Matrice de scale:

```
s = 

sx 0  0
0  sy 0
0  0 sz
```

```
p = (x, y, z) *^* s

p = (
    x * sx + y * 0 + z * 0,
    x * 0 + y * sy + z * 0,
    x * 0 + y * 0 + z * sz
    )
```

# Rotation

Soit `p = (x, y)`, et une rotation selon `z` d'angle `dw`.

```
// Convertir p en coordonée polaire

rayon = sqrt(x * x + y * y)
angle = atan(y, x);

// Ajouter l'angle
angle' = angle + dw

// Converti en carthesienne
x' = rayon * cos(angle')
y' = rayon * sin(angle')


x' = sqrt(x * x + y * y) * cos(angle + dw)

"Le cosinus est raciste et menteur"

cos(a + b) = cos(a) * cos(b) - sin(b) * sin(a)

"Le sinus il est cool"

sin(a + b) = cos(a) * sin(b) + cos(b) * sin(a)

x' = rayon * cos(angle) * cos(dw) - rayon * sin(angle) * sin(dw)
y' = rayon * cos(angle) * sin(dw) + rayon * cos(dw) * sin(angle)

==>

x' = x * cos(dw) - y * sin(dw)
y' = x * sin(dw) + y * cos(dw)


rotation = 

    cos(dw)    -sin(dw)
    sin(dw)    cos(dw)
```

Je suis trop heureux, cela marche!

On a reussi à avoir scale et rotation sous la forme d'une matrice.


Code de transformation:

```
p' = p *^* scaleMatrix *^* rotationMatrix


// Implicitement c'est:
p' = (p *^* scaleMatrix) *^* rotationMatrix

// Implicitement, si on a 12 operations:

p' = (((p *^* m1) ^*^ m2) ^*^ m3) ...
```

Mais, rappelez vous. L'associativité.

La multiplication de matrice est associative!

```
p' = (p *^* scaleMatrix) *^* rotationMatrix

<=>

p' = p *^* (scaleMatrix *^* rotationMatrix)

<=>
myTransform = scaleMatrix *^* rotationMatrix

...

p' = p *^* myTransform
```

a) On peut representer tout un ensemble de transformation (scales + rotations) avec une unique matrice! (ici `2x2` en 2D).
b) Cette unique matrice est compact (ici 4 float en 2D).
c) L'application est efficace (1 multiplication de matrice par transformation !)


Ok, c'est bien gentil, mais quid des translations ?

# Coordonnée homogenes


Soit `p = (x, y)` et une translation `dp = (dx, dy)`.

On voudrait:

```
p' =
     (
        x + dx
        y + dy
        w
     )
```

Matrice de translation:

```
t = 

1 0 a
0 1 b
c d e
```

```
p ^*^ t
= (
     x * 1 + 0 * y     (a * w = dx)
     x * 0 + 1 * y     + (b * w = dy)
     c * x + d * y + e * w
  )
```

si `a = dx` et `w = 1`, et pareil pour `b = dy`.

```

p' =
     (
        x + dx
        y + dy
        1
     )


t = 

1 0 dx
0 1 dy
0 0 1
```

```
p ^*^ t
= (
     x * 1 + 0 * y + 
     x * 0 + 1 * y     + (b * w = dy)
     0 * x + 0 * y + 1 * 1
  )
```

# Conclusion:

- On a fait rentrer la translation, le scale, et la rotation dans une matrice 3x3 (en dimension 2).
- Pour passer en dimension 4, il suffit d'avoir une matrix 4x4
- Pour les rotations, on garde une rotatino dans le plan `z = 0`, et on pourra faire des changements de base.

# Perspective

x' = x / z
y' = y / z

On va poser que la dernière composante des coordonnées homogenes, c'est un facteur d'echelle.


```
p = (x, y z, w) (en homogene)
= (x / w, y / w, z / w) en 3D
```

si:

```
perspective = 
[ 1 0 0 0
  0 1 0 0
  0 0 1 0
  0 0 1 0
]
```

Alors `p = (x, y, z, 1) *^* perspective = `

```
p' = (x
      y
      z
      z
     )
     ===> (x / z, y / z, z / z, 1)
```

```glsl


vec2 to_screen_space(vec2 i)
{
   float factor = iResolution.y;
   vec2 centered = i - iResolution.xy / 2.0;
   
   return centered / factor * 2.0;
}

vec2 translation(vec2 i)
{
    return i + vec2(sin(iTime), sin(iTime * 2.0)) ;
}

mat3 translation_matrix(vec2 d)
{
    return mat3(
       vec3(1, 0, d.x),
       vec3(0, 1, d.y),
       vec3(0, 0, 1));
}


vec2 translation_mouse(vec2 i)
{
    return i + to_screen_space(iMouse.xy);
}

vec2 scale(vec2 i)
{
    return i * vec2(0.5, 0.5);
}

mat3 scale_matrix(vec2 s)
{
    return mat3(
                 vec3(s.x, 0, 0),
                 vec3(0, s.y, 0),
                 vec3(0, 0, 1)
                 );
}


vec2 scale_mouse(vec2 i)
{
    return i * iMouse.xy / iResolution.xy;
}

vec2 rotation(vec2 i, float dw)
{
   // En coordonnée polaire:
   float rayon = sqrt(i.x * i.x + i.y * i.y);
   float angle = atan(i.y, i.x);
   
   // On fait la rotation
   float angle2 = angle + dw;
   
   // On convertie en carthesienne
   vec2 res = vec2(rayon * cos(angle2), rayon * sin(angle2));
   
   return res;
}

mat3 rotation_matrix(float dw)
{
   return mat3(
                 vec3(cos(dw), -sin(dw), 0),
                 vec3(sin(dw), cos(dw), 0),
                 vec3(0, 0, 1)
                 );
}

        
int nbPoints = 4;

bool isInside(vec2 points[4], vec3 pointC)
{
    bool inside = true;
    
    for(int i = 0; i < nbPoints; i++)
    {
      vec3 pointA = vec3(points[i], 0);
      vec3 pointB = vec3(points[(i+1) % nbPoints], 0);
      vec3 res = cross(pointB - pointA, pointC - pointA);
    
      if(res.z > 0.0)
         inside = false;
    }
    
    return inside;
}


void mainImage( out vec4 fragColor, in vec2 fragCoord )
{
    vec3 pointC = vec3(to_screen_space(fragCoord.xy), 2);
    
    vec2 points[4];

    points[3] = vec2(-0.25, -0.25);
    points[2] = vec2 (0.25, -0.25);
    points[1] = vec2 (0.25, 0.25);
    points[0] = vec2 (-0.25, 0.25);
    
    
    mat3 trans = scale_matrix(vec2(0.5, 0.5))
               * rotation_matrix(iTime)
               * translation_matrix(to_screen_space(iMouse.xy));
    
    // Applique la transformation
    for(int i = 0; i < nbPoints; i++)
    {
        // points[i] = translation_mouse(scale(points[i]));
        // points[i] = translation_mouse(rotation(points[i], iTime));
        points[i] = (vec3(points[i], 1) * trans).xy;
    }
   
    bool inside = isInside(points, pointC);
 
    
    if(inside)
    {
       fragColor = vec4(1, 0, 0, 1);
    }
    else
    {
      fragColor = vec4(0, 1, 0, 1);
    }

}
```



















