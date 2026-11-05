# next/no-typos — cases

### case: a missing final s on a paths function

```js path=pages/index.js
export const getstaticpath = async () => ({ paths: [], fallback: false });
```

```expect
1:14 next/no-typos
```

```fixed
export const getStaticPaths = async () => ({ paths: [], fallback: false });
```

### case: a props function

```js path=pages/index.js
export function GetStaticProp() {
  return { props: {} };
}
```

```expect
1:8 next/no-typos
```

```fixed
export function getStaticProps() {
  return { props: {} };
}
```
