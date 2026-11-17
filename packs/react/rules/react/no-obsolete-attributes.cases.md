# react/no-obsolete-attributes — cases

### case: a component prop of that name

```jsx
const t = <Table align="left" />;
```

```expect
(none)
```

### case: the same attribute on an element it is not obsolete on

```jsx
const t = <td colSpan={2} />;
```

```expect
(none)
```

### case: align on a div

```jsx
const t = <div align="center" />;
```

```expect
1:11 react/no-obsolete-attributes
```
