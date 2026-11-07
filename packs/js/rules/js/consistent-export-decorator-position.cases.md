# js/consistent-export-decorator-position — cases

### case: a named export

```ts
export @sealed class Store {}
```

```expect
1:8 js/consistent-export-decorator-position
```

```fixed
@sealed
export class Store {}
```

### case: a decorator before export stays

```ts
@sealed export class Store {}
```

```expect
(none)
```
