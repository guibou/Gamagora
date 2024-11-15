# Transformations

Les transformatinos de base sont:

- translation
- rotation
- mise à l'échelle
- project

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

Soit un point `p = (x, y, z)` et une transformation `t = (dx, dy, dz)`, alors on obtient:

```
p' = p + t
```

Un example, soit `p = (5, 2)`, et `t = (2, 0)`, alors `p' = (7, 2)`.

# Scale

Soit un point `p = (x, y, z)` et une transformation `s = (sx, sy, sz)`, alors on obtient:

```
p' = p * s
```

Example, soit un point `p = (5, 3)`, et une mise à l'echelle `s = (2, 0.5)`, alors `p' = (10, 1.5)`.

# Non communitativé

Les transformations ne sont pas commutatives si pas du même type.

D'abord une transaltion, puis ensuite le scale

```
p' = p + t
p'' = s * p' = s * p + s * t
```

Dans l'autre sens:

```
p' = p * s
p'' = p' + t = p * s + t
```

# Rotation

La prochaine fois.
