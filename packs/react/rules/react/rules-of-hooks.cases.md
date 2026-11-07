# react/rules-of-hooks — cases

### case: a hook returned after an early return stays manual

```ts
function useProfile(id: string) {
  if (!id) {
    return null;
  }
  return useState(id);
}
```

```expect
5:10 react/rules-of-hooks
```
