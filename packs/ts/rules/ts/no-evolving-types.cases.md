# ts/no-evolving-types — cases

### case: an empty array

```ts
const items = [];
items.push(1);
```

```expect
1:7 ts/no-evolving-types
```

### case: annotated

```ts
const items: number[] = [];
```

```expect
(none)
```

### case: a const null

```ts
const none = null;
```

```expect
(none)
```
