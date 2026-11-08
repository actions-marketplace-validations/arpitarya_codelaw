# Cases: css/keyframe-block-no-duplicate-selectors

### case: a selector shared with another keyframe is reported without a fix

```css
@keyframes fade {
  0%,
  50% {
    opacity: 0;
  }

  50% {
    opacity: 1;
  }
}
```

```expect
7:3 css/keyframe-block-no-duplicate-selectors
```
