export function hex(i: number, digits = 2): string {
  return i.toString(16).padStart(digits, '0');
}
