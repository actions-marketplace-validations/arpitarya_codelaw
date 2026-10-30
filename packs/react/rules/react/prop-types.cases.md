# react/prop-types — cases

### case: a typed parameter is declared

```tsx
function Hello({ name }: { name: string }) {
  return <p>{name}</p>;
}
```

```expect
```

### case: a class component

```jsx
class Hello extends Component {
  render() {
    return <p>{this.props.name}</p>;
  }
}
```

```expect
3:16 react/prop-types
```
