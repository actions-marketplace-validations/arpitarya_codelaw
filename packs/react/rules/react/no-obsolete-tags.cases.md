# react/no-obsolete-tags — cases

### case: a self-closing marquee

```jsx
const a = <marquee />;
```

```expect
1:11 react/no-obsolete-tags
```

### case: a component named like one

```jsx
const a = <Font />;
```

```expect
(none)
```
