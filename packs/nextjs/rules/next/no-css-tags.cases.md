# next/no-css-tags — cases

### case: a local stylesheet link

```jsx
const l = <link rel="stylesheet" href="styles.css" />;
```

```expect
1:11 next/no-css-tags
```
