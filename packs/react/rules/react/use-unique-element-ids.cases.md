# react/use-unique-element-ids — cases

### case: a string in braces

```jsx
const f = <input id={"email"} />;
```

```expect
1:18 react/use-unique-element-ids
```

### case: a variable

```jsx
const f = <input id={id} />;
```

```expect
(none)
```
