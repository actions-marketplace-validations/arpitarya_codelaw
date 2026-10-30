type Shape = "circle" | "square";
export function sides(s: Shape): number {
  switch (s) {
    case "circle":
      return 0;
  }
  return -1;
}
