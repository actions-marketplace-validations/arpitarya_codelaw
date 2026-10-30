# Cases: ts/switch-exhaustiveness-check

### case: every enum member and both booleans handled

```ts
enum Color { Red, Green }
export function f(c: Color, b: boolean): number {
  switch (c) {
    case Color.Red:
      return 1;
    case Color.Green:
      return 2;
  }
  switch (b) {
    case true:
      return 3;
    case false:
      return 4;
  }
}
```

```expect
(none)
```

### case: a boolean with one case

```ts
export function f(b: boolean): number {
  switch (b) {
    case true:
      return 1;
  }
  return 0;
}
```

```expect
2:3 ts/switch-exhaustiveness-check
```

### case: a value that is not a union of literals is not judged

```ts
export function f(s: string): number {
  switch (s) {
    case "a":
      return 1;
  }
  return 0;
}
```

```expect
(none)
```

### case: a default makes it exhaustive

```ts
type Shape = "circle" | "square";
export function f(s: Shape): number {
  switch (s) {
    case "circle":
      return 1;
    default:
      return 0;
  }
}
```

```expect
(none)
```
