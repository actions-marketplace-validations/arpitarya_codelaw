# react/no-non-scalable-viewport — cases

### case: a maximum scale of one

```jsx
const m = <meta name="viewport" content="width=device-width, maximum-scale=1.0" />;
```

```expect
1:11 react/no-non-scalable-viewport
```

### case: a maximum scale of five

```jsx
const m = <meta name="viewport" content="width=device-width, maximum-scale=5" />;
```

```expect
(none)
```
