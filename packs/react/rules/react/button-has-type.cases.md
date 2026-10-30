# react/button-has-type — cases

### case: a spread may carry the type

```jsx
const b = <button {...props}>x</button>;
```

```expect
```

### case: self-closing

```jsx
const b = <button />;
```

```expect
1:11 react/button-has-type
```

```fixed
const b = <button type="button" />;
```
