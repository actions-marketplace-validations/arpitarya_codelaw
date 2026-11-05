# next/no-async-client-component — cases

### case: a server component may be async

```jsx
export default async function Page() {
  return <p>{await load()}</p>;
}
```

```expect

```
