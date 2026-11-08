---
id: js/no-canvas-to-image
title: "No round trip of canvas pixels through an image"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "drawing the canvas directly changes the call's arguments and may remove an `await`"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/no-canvas-to-image]
since: 0.5.0
---

Two round trips are reported at the argument: `ctx.drawImage(img, …)` where `img` is `await loadImage(<canvas>.toDataURL())` (directly, or a `const` used only there; the receiver's name contains `canvas`), and `ctx.putImageData(data, …)` where `data` is a `getImageData()` result used for nothing else.

**Why:** Encoding a canvas to a data URL and decoding it again, or reading pixels only to write them back, costs time and memory for what `drawImage(canvas, …)` does directly.

```codelaw
native: js/no-canvas-to-image
```

## How to fix

1. Pass the source canvas to `drawImage(sourceCanvas, …)`.

## Examples

### JavaScript

#### Don't

```js
const image = await loadImage(sourceCanvas.toDataURL());
ctx.drawImage(image, 0, 0);
```

#### Do

```js
ctx.drawImage(sourceCanvas, 0, 0);
```


### JSX

#### Don't

```jsx
const image = await loadImage(sourceCanvas.toDataURL());
ctx.drawImage(image, 0, 0);
```

#### Do

```jsx
ctx.drawImage(sourceCanvas, 0, 0);
```


### TypeScript

#### Don't

```ts
const image = await loadImage(sourceCanvas.toDataURL());
ctx.drawImage(image, 0, 0);
```

#### Do

```ts
ctx.drawImage(sourceCanvas, 0, 0);
```


### TSX

#### Don't

```tsx
const image = await loadImage(sourceCanvas.toDataURL());
ctx.drawImage(image, 0, 0);
```

#### Do

```tsx
ctx.drawImage(sourceCanvas, 0, 0);
```
