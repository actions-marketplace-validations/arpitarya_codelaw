---
id: js/aws-apigateway-public-api
title: "API Gateway methods require authorisation"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether the endpoint is meant to be public is the owner's call"
stability: preview
ported_from: [eslint-plugin-sonarjs/aws-apigateway-public-api]
since: 0.5.0
---

An AWS CDK API Gateway method is not given `AuthorizationType.NONE`.

**Why:** An unauthenticated endpoint is callable by anyone who finds it.

```codelaw
match:
  kind: pair
  has:
    field: key
    regex: ^["']?(authorizationType)["']?$
  all:
  - has:
      field: value
      regex: AuthorizationType\.NONE$|^.NONE.$
message: This API method needs no authorisation; anyone can call it
```

## How to fix

1. Set the property to its secure value, or remove it to take the secure default, and confirm nothing relies on the open setting.

## Examples

### JavaScript

#### Don't

```js
api.root.addMethod("GET", integration, { authorizationType: apigateway.AuthorizationType.NONE });
```

#### Do

```js
api.root.addMethod("GET", integration, { authorizationType: apigateway.AuthorizationType.IAM });
```

### JSX

#### Don't

```jsx
api.root.addMethod("GET", integration, { authorizationType: apigateway.AuthorizationType.NONE });
```

#### Do

```jsx
api.root.addMethod("GET", integration, { authorizationType: apigateway.AuthorizationType.IAM });
```

### TypeScript

#### Don't

```ts
api.root.addMethod("GET", integration, { authorizationType: apigateway.AuthorizationType.NONE });
```

#### Do

```ts
api.root.addMethod("GET", integration, { authorizationType: apigateway.AuthorizationType.IAM });
```

### TSX

#### Don't

```tsx
api.root.addMethod("GET", integration, { authorizationType: apigateway.AuthorizationType.NONE });
```

#### Do

```tsx
api.root.addMethod("GET", integration, { authorizationType: apigateway.AuthorizationType.IAM });
```
