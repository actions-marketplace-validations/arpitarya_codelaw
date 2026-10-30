# react/a11y-anchor-is-valid — cases

### case: a fragment link is fine

```jsx
const a = <a href="#top">Top</a>;
```

```expect

```

### case: a javascript: URL

```jsx
const a = <a href="javascript:void(0)">x</a>;
```

```expect
1:11 react/a11y-anchor-is-valid
```
