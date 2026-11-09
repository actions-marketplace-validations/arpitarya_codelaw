# js/prefer-type-guard — cases

### case: Boolean call and no annotation

```ts
function isFish(pet: Pet) {
  return Boolean((pet as Fish).swim);
}
```

```expect
1:10 js/prefer-type-guard
```

```fixed
function isFish(pet: Pet): pet is Fish {
  return Boolean((pet as Fish).swim);
}
```

### case: a double negation

```ts
function isFish(pet: Pet): boolean {
  return !!(pet as Fish).swim;
}
```

```expect
1:10 js/prefer-type-guard
```

```fixed
function isFish(pet: Pet): pet is Fish {
  return !!(pet as Fish).swim;
}
```

### case: a method on this

```ts
class Pet {
  isFish(): boolean {
    return (<Fish>this).swim != undefined;
  }
}
```

```expect
2:3 js/prefer-type-guard
```

```fixed
class Pet {
  isFish(): this is Fish {
    return (<Fish>this).swim != undefined;
  }
}
```

### case: already a predicate

```ts
function isFish(pet: Pet): pet is Fish {
  return (pet as Fish).swim !== undefined;
}
```

```expect
(none)
```

### case: a cast to any

```ts
function f(pet: Pet) {
  return (pet as any).swim !== undefined;
}
```

```expect
(none)
```

### case: two parameters

```ts
function f(pet: Pet, other: Pet) {
  return (pet as Fish).swim !== undefined;
}
```

```expect
(none)
```

### case: more than the return

```ts
function f(pet: Pet) {
  log(pet);
  return (pet as Fish).swim !== undefined;
}
```

```expect
(none)
```

### case: no cast

```ts
function f(pet: Pet) {
  return pet.swim !== undefined;
}
```

```expect
(none)
```
