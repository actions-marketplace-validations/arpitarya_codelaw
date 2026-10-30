# react/jsx-fragments — cases

### case: a keyed fragment needs the long form

```jsx
const l = items.map((i) => <Fragment key={i.id}><A /></Fragment>);
```

```expect
```
