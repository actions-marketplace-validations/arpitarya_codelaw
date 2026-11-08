# stencil/enforce-slot-jsdoc — cases

### case: a rendered slot with no tag needs its words from the author

```tsx
/**
 * A card.
 */
@Component({ tag: 'my-card' })
export class Card {
  render() {
    return <slot name="header" />;
  }
}
```

```expect
5:14 stencil/enforce-slot-jsdoc
```
