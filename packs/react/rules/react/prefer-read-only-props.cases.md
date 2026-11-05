# react/prefer-read-only-props — cases

### case: an interface

```tsx
interface CardProps {
  title: string;
}
```

```expect
2:3 react/prefer-read-only-props
```

```fixed
interface CardProps {
  readonly title: string;
}
```
