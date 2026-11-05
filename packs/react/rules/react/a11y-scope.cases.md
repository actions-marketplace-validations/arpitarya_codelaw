### case: scope on a div is removed

```jsx
const c = <div scope="row">Total</div>;
```

```expect
1:16 react/a11y-scope
```

```fixed
const c = <div >Total</div>;
```
