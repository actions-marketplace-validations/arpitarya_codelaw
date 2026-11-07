# react/no-ref-in-render — cases

### case: a ref changed in place during render stays manual

```jsx
function Counter() {
  const renders = useRef(0);
  renders.current += 1;
  return <p>{renders.current}</p>;
}
```

```expect
3:3 react/no-ref-in-render
4:14 react/no-ref-in-render
```

### case: a ref changed in place during render stays manual (TSX)

```tsx
function Counter() {
  const renders = useRef(0);
  renders.current += 1;
  return <p>{renders.current}</p>;
}
```

```expect
3:3 react/no-ref-in-render
4:14 react/no-ref-in-render
```
