# A discussion about type safety and sentinel values

The question started with "what can be the correct return value for the intersection function, which (for now) returns a floating point number `t` such that:

```rust
X = ray.origin + t * ray.direction
```

What should we do if there is no intersection?

It was suggested to either:

- return `null`. This is not possible in rust, but this is doable in other language. If your language ensure that nullable value need to be checked for null before usage, you are good to go. But if not, you may forget about checking and get exception, or undefined behavior (see at the end of this document).
- return `-1`. This is called a "sentinel value". The drawback with this approach is that we may forget to check for it, and the value can be used, leading to incorrect results. For example, if we compute the intersection point using `t = -1`, we'll get a point behind the origin of the ray (e.g. in the wrong direction).
- return `(-1, -1, -1)`, because anyway, we want the position of the intersection point `X`. This is even worse than the `-1` sentinel value, because it is possible to have a correct intersection at `(-1, -1, -1)` (with some camera and object position). This could lead to an hidden error which will just explode in your face in a few year with a "dark" pixel in the middle of the screen.

The implementation I choose (in rust) is to use the `Option<f32>` type, which either represents an intersection with `Some(f32)` or no intersection with `None`, and forces you to pattern match on it:

```rust
match get_intersection(&ray, &scene)
  Some(t) => // The intersection exists and is at `t`
  None => // No intersection
```

With this approach, we cannot use an incorrect value.

Keep this in mind when developping, what can be done in order to ensure that I
won't use incorrect values. One strategy is to not generate incorrect values in
the first place, and as such, ban "sentinel" values.

# Undefined behavior

In C/C++/..., it exists the concept of undefined behavior. For example, reading the value pointed by a `NULL` / `nullptr` pointer is undefined.

What does it mean?

After a quick pool, most students think that it will generate a crash, or some random value. This is partially correct.

The correct answer is that it can mean ANYTHING, so *DO NOT RELY on UNDEFINED BEHAVIOR*. Period.

## A more involved definition

Your compiler is making optimisations / transformations on your code. For
example, it can compute operations which are known at compile time, or "inline"
functions by replacing their content at call site.

For example, the following piece of code:

```c
bool is_even(int x)
{
   return x % 2 == 0;
}

...
bool c = is_even(100);

if(c)
{
   printf("hello\n");
}
else
{
   printf("world\n");
}
```

The `is_even(100)` call can be inlined and replaced by `100 % 2 == 0`, which
can then be computed at compile time and replaced by `true`. Hence `if(c)` is
always `true` and the first branch is always taken. Hence the compiler can
optimise the code to:

```c
printf("hello\n")
```
## find a title for this section

In order to do such optimisations, the compiler have rules defining
transformatinons and the context in which they are correct. This context does
not know about "undefined behavior", it only knows about what is correct.

For example, the following code:

```c
int *ptr = foo(..);

if(ptr)
{
   doSomething1(*ptr);
}
if(ptr)
{
   doSomething2(*ptr);
}
```

The compiler is free to group the values together:

```c
if(ptr)
{
  doSomethngi1(*ptr)
  doSomtehngi2(*ptr)
}
```

Or even remove the test if logical reasoning tells that `ptr` is never `NULL`.

And this can break in dramatic way, here is a more convoluted example:

```c
int tab[8];

int i = 0;
while(i <= 8)
{
   tab[i] = 0;
   i + i = 1;
}
```

This example is supposed to fill an array of 8 values with 0. However it does
have an error in the while condition `i <= 8` should be `i < 8`, which could
lead to `tab[8]` which is a value which does not exists.

The program can crash, or write somewhere else, or... Actually, we don't know.
That's un undefined behavior. Reading an array at an offset which does not
exists, is undefined, and the compiler is free to use this fact in order to
change the code.

But it does not mean that the compiler is doing stupid thing by toying with undefined behavior. Actually it just follows rules (which are not aware of undefined behavior).

For example, by writing `tab[8]` and `tab[i]`, you explicitely tell to the compiler that `i` (on the `tab[i]` line) is `0 <= i < 8`. Following the next line where `i` is incremented, at the end of the loop, `1 <= i < 9`. The condition of the loop `i <= 8` is hence ALWAYS satisfied, so it can be simplified, leading to an infinite loop.
