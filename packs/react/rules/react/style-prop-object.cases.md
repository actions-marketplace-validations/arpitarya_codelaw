### case: several declarations, one with a dash

```jsx
const d = <div style="color: red; margin-top: 4px;" />;
```

```expect
1:16 react/style-prop-object
```

```fixed
const d = <div style={{ color: "red", marginTop: "4px" }} />;
```

### case: a value with quotes is rewritten by hand

```jsx
const d = <div style="font-family: 'Inter'" />;
```

```expect
1:16 react/style-prop-object
```
