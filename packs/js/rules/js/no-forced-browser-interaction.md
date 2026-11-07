---
id: js/no-forced-browser-interaction
title: "No forced click or type in a UI test"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "what makes the element actionable is the author's to find"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-sonarjs/no-forced-browser-interaction]
since: 0.5.0
---

A Cypress or Playwright interaction (`click`, `type`, `check`, `fill`, `hover`, …) is not passed `{ force: true }`.

**Why:** Forcing skips the checks that the element is visible and enabled, so the test passes where a user could not act.

```codelaw
match:
  pattern: "$X.$M($$$A)"
  has:
    field: arguments
    has: { kind: object, has: { kind: pair, regex: '^force\s*:\s*true$' } }
constraints:
  M: { regex: '^(click|dblclick|rightclick|type|check|uncheck|select|selectOption|clear|trigger|fill|hover|tap|setChecked|dragTo|press)$' }
message: "Don't force the interaction; make the element actionable"
```

## How to fix

1. Remove `force: true` and wait for, scroll to or reveal the element the way a user would.

## Examples

### JavaScript

#### Don't

```js
cy.get("#save").click({ force: true });
```

#### Do

```js
cy.get("#save").should("be.visible").click();
```


### JSX

#### Don't

```jsx
cy.get("#save").click({ force: true });
```

#### Do

```jsx
cy.get("#save").should("be.visible").click();
```


### TypeScript

#### Don't

```ts
cy.get("#save").click({ force: true });
```

#### Do

```ts
cy.get("#save").should("be.visible").click();
```


### TSX

#### Don't

```tsx
cy.get("#save").click({ force: true });
```

#### Do

```tsx
cy.get("#save").should("be.visible").click();
```
