---
id: js/no-fixed-wait-in-tests
title: "No fixed wait in a UI test"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "what to wait for is the author's to name"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-sonarjs/no-fixed-wait-in-tests]
since: 0.5.0
---

A Cypress or Playwright test does not wait a fixed time — `cy.wait(1000)`, `page.waitForTimeout(1000)` — but for the condition it needs.

**Why:** A fixed wait is too long on a fast machine and too short on a slow one, so the suite is slow and flaky at once.

```codelaw
match:
  any:
    - pattern: "cy.wait($N)"
    - pattern: "$P.waitForTimeout($N)"
constraints:
  N: { kind: number }
message: "Wait for a condition, not a fixed time"
```

## How to fix

1. Wait for the element, route or state the test needs: `cy.get(…).should(…)`, `cy.wait("@alias")`, `await expect(locator).toBeVisible()`.

## Examples

### JavaScript

#### Don't

```js
cy.wait(2000);
```

#### Do

```js
cy.wait("@loadUsers");
```


### JSX

#### Don't

```jsx
cy.wait(2000);
```

#### Do

```jsx
cy.wait("@loadUsers");
```


### TypeScript

#### Don't

```ts
cy.wait(2000);
```

#### Do

```ts
cy.wait("@loadUsers");
```


### TSX

#### Don't

```tsx
cy.wait(2000);
```

#### Do

```tsx
cy.wait("@loadUsers");
```
