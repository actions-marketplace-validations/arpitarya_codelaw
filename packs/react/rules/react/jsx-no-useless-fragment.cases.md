# react/jsx-no-useless-fragment — cases

### case: two children

```jsx
const r = <><A /><B /></>;
```

```expect
```

### case: an expression inside another element

```jsx
const r = <div><>{items}</></div>;
```

```expect
1:16 react/jsx-no-useless-fragment
```

```fixed
const r = <div>{items}</div>;
```

### case: an expression returned bare

```jsx
const r = <>{items}</>;
```

```expect
1:11 react/jsx-no-useless-fragment
```

```fixed
const r = items;
```

### case: an expression and text

```jsx
const r = <div><>{items} more</></div>;
```

```expect
```

### case: two expressions

```jsx
const r = <>{a}{b}</>;
```

```expect
```

### case: a spread, a comment or an empty expression

```jsx
const a = <>{...rest}</>;
const b = <>{/* none */}</>;
const c = <>{}</>;
```

```expect
```
