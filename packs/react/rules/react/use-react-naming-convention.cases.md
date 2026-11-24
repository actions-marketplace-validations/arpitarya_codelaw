# react/use-react-naming-convention — cases

### case: a ref

```jsx
function F() {
  const input = useRef(null);
  return <input ref={input} />;
}
```

```expect
2:9 react/use-react-naming-convention
```

### case: good names

```jsx
const ThemeContext = React.createContext(null);
function F() {
  const id = useId();
  const inputRef = useRef(null);
  const nameId = React.useId();
  return <input id={id} ref={inputRef} />;
}
```

```expect
(none)
```

### case: a lowercase context

```jsx
const theme = React.createContext(null);
```

```expect
1:7 react/use-react-naming-convention
```
