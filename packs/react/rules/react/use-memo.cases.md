# react/use-memo — cases

### case: a callback that returns nothing stays manual

```jsx
function Report({ rows }) {
  const sorted = useMemo(() => {
    rows.sort(byDate);
  }, [rows]);
  return <Table rows={sorted} />;
}
```

```expect
2:18 react/use-memo
```

### case: a parameter the callback reads stays manual

```ts
function useLabel(id: string) {
  return useMemo((prefix: string) => prefix + id, [id]);
}
```

```expect
2:10 react/use-memo
```

### case: a result nothing uses stays manual

```tsx
function Log({ value }: { value: number }) {
  useMemo(() => console.log(value), [value]);
  return <p>{value}</p>;
}
```

```expect
2:3 react/use-memo
```
