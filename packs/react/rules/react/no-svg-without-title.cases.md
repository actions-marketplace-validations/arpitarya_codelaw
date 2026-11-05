### case: a self-closing svg is hidden

```jsx
const icon = <svg viewBox="0 0 8 8" />;
```

```expect
1:14 react/no-svg-without-title
```

```fixed
const icon = <svg aria-hidden="true" viewBox="0 0 8 8" />;
```
