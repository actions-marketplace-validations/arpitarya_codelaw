# react/immutability — cases

### case: state from useState

```jsx
function C() {
  const [user, setUser] = useState({});
  const rename = () => { user.name = "x"; };
  return <b onClick={rename} />;
}
```

```expect
3:26 react/immutability
```

### case: a local copy is fine

```jsx
function C({ items }) {
  const copy = [...items];
  copy.sort();
  return copy.length;
}
```

```expect
```
