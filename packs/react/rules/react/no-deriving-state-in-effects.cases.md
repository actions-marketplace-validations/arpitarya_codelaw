# react/no-deriving-state-in-effects — cases

### case: an updater function is not deriving

```jsx
function P() {
  const [n, setN] = useState(0);
  useEffect(() => {
    setN((c) => c + 1);
  }, [setN]);
  return n;
}
```

```expect
```

### case: a setter from another hook

```jsx
function P({ a }) {
  const [n, setN] = useAtom(x);
  useEffect(() => {
    setN(a);
  }, [a]);
  return n;
}
```

```expect
```
