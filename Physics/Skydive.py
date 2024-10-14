import dataclasses
import random

@dataclasses.dataclass
class Vec3:
    x: float
    y: float
    z: float

    def __add__(self, other):
        return Vec3(x = self.x + other.x,
                    y = self.y + other.y,
                    z = self.z + other.z)

    def __rmul__(self, otherf):
        return Vec3(x = otherf * self.x,
                    y = otherf * self.y,
                    z = otherf * self.z)

p0 = Vec3(x = 0, y = 0, z = 0)
v0 = Vec3(x = 100, y = 0, z = 100)

g = 10
m = 1

a = Vec3(x = 0, y = 0, z = -1 * m * g)

t = 0
dt = 0.01

p = p0
v = v0

while t < 100:
    p = p + dt * v
    v = v + dt * a

    t = t + dt

    print(f"{t} {p.x} {p.y} {p.z} {v.x} {v.y} {v.z}")

