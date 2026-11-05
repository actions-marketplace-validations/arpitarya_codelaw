# next/google-font-preconnect — cases

### case: a link with only an href gets the hint

```jsx
const l = <link href="https://fonts.gstatic.com" />;
```

```expect
1:11 next/google-font-preconnect
```

```fixed
const l = <link href="https://fonts.gstatic.com" rel="preconnect" crossOrigin="" />;
```

### case: a link with another rel carries no edit

```jsx
const l = <link href="https://fonts.gstatic.com" rel="stylesheet" />;
```

```expect
1:11 next/google-font-preconnect
```
