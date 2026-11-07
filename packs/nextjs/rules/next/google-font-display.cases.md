### case: a URL with no query gains one

```jsx
const l = <link href="https://fonts.googleapis.com/css2" rel="stylesheet" />;
```

```expect
1:11 next/google-font-display
```

### case: another display value is replaced

```jsx
const l = <link href="https://fonts.googleapis.com/css2?display=block&amp;family=Inter" rel="stylesheet" />;
```

```expect
1:11 next/google-font-display
```
