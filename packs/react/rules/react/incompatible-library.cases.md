# react/incompatible-library — cases

### case: TanStack Table

```jsx
import { useReactTable } from "@tanstack/react-table";
function T(o) {
  const t = useReactTable(o);
  return t;
}
```

```expect
3:13 react/incompatible-library
```

### case: a local function of the same name

```jsx
function useReactTable() {}
function T() {
  return useReactTable();
}
```

```expect
```
