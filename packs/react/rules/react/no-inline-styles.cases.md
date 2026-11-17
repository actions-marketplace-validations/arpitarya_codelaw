# react/no-inline-styles — cases

### case: a shared style object

```jsx
const a = <p style={warning}>Hi</p>;
```

```expect
(none)
```

### case: an object literal

```jsx
const a = <p style={{ margin: 0 }}>Hi</p>;
```

```expect
1:14 react/no-inline-styles
```
