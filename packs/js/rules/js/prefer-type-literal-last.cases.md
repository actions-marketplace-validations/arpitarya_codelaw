# js/prefer-type-literal-last — cases

### case: null stays last

```ts
type Value = { id: string } | null;
```

```expect
(none)
```

### case: a literal already last stays

```ts
type Value = Base | { id: string };
```

```expect
(none)
```
