---
id: js/no-debug-commands-in-ui-tests
title: "No pause or debug command left in a UI test"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "Deletes a debugging aid put there on purpose while debugging, so save and the agent hook leave it for codelaw fix."
on_edit: false
stability: preview
ported_from: [eslint-plugin-sonarjs/no-debug-commands-in-ui-tests]
since: 0.5.0
---

A Cypress test does not call `cy.pause()` or `cy.debug()`, and a Playwright test does not `await page.pause()`.

**Why:** They stop the run for a person to step through, so a suite that has one hangs in CI.

```codelaw
match:
  kind: expression_statement
  any:
    - pattern: "cy.pause();"
    - pattern: "cy.debug();"
    - pattern: "await page.pause();"
message: "Remove this debugging command"
fix:
  - remove
```

## Examples

### JavaScript

#### Don't

```js
cy.visit("/");
cy.pause();
cy.get("button").click();
```

#### Do

```js
cy.visit("/");
cy.get("button").click();
```


### JSX

#### Don't

```jsx
cy.visit("/");
cy.pause();
cy.get("button").click();
```

#### Do

```jsx
cy.visit("/");
cy.get("button").click();
```


### TypeScript

#### Don't

```ts
cy.visit("/");
cy.pause();
cy.get("button").click();
```

#### Do

```ts
cy.visit("/");
cy.get("button").click();
```


### TSX

#### Don't

```tsx
cy.visit("/");
cy.pause();
cy.get("button").click();
```

#### Do

```tsx
cy.visit("/");
cy.get("button").click();
```
