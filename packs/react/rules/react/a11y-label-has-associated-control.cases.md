### case: a label before a control with a static id gets htmlFor

```jsx
const f = <><label>Name</label><input id="name" /></>;
```

```expect
1:13 react/a11y-label-has-associated-control
```

```fixed
const f = <><label htmlFor="name">Name</label><input id="name" /></>;
```
