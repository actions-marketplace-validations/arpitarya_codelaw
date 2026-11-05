# next/no-script-component-in-head — cases

### case: a Script with children carries no edit

```jsx
const p = <><Head><Script>{`x`}</Script></Head></>;
```

```expect
1:19 next/no-script-component-in-head
```
