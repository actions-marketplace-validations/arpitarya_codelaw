### case: a heading role gets aria-level

```jsx
const h = <div role="heading">Title</div>;
```

```expect
1:11 react/a11y-role-has-required-aria-props
```

```fixed
const h = <div role="heading" aria-level="2">Title</div>;
```
