# js/no-useless-computed-key — cases

### case: a TypeScript interface member is not checked

```ts
interface M {
  ['a/b']: string;
}
```

```expect

```
