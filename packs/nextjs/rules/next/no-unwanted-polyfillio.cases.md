# next/no-unwanted-polyfillio — cases

### case: a script that is not a child carries no edit

```jsx
const s = <script src="https://cdn.polyfill.io/v3/polyfill.min.js" />;
```

```expect
1:11 next/no-unwanted-polyfillio
```
