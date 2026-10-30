# react/no-array-index-key — cases

### case: in a template literal

```jsx
const l = items.map((item, i) => <li key={`row-${i}`}>{item.name}</li>);
```

```expect
1:38 react/no-array-index-key
```

### case: an index used elsewhere

```jsx
const l = items.map((item, i) => <li key={item.id}>{i}</li>);
```

```expect
```
