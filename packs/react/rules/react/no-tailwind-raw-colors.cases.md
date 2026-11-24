# react/no-tailwind-raw-colors — cases

### case: a variant and an opacity

```jsx
const b = <div className="hover:text-red-500/80" />;
```

```expect
1:16 react/no-tailwind-raw-colors
```

### case: inside cn()

```jsx
const b = <div className={cn("p-2", on && "border-slate-200")} />;
```

```expect
1:16 react/no-tailwind-raw-colors
```

### case: tokens and white

```jsx
const b = <div className="bg-white text-black bg-primary border-transparent" />;
```

```expect
(none)
```
