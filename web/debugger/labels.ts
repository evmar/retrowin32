/**
 * For debugging purposes we support labelling addresses.
 * These labels come either from ghidra CSV dumps, or from import tables.
 * When displaying an address we find the nearest label and display at as e.g.
 *   SomeFunction+3c
 */

import { hex } from './util';

/**
 * Parses a Ghidra-generated .csv file contains labelled addresses.
 */
export function* parseCSV(text: string): Iterable<[number, string]> {
  for (const line of text.split('\n')) {
    const [name, addr] = line.split('\t');
    yield [parseInt(addr, 16), name];
  }
}

/** Manages the collection of labels, as an ordered list. */
export class Labels {
  byAddr: Array<[number, string]> = [];

  load(labels: Iterable<[number, string]>) {
    this.byAddr.push(...labels);
    // Avoid labelling small numbers.
    this.byAddr = this.byAddr.filter(([addr, _]) => addr > 0x1000);
    this.byAddr.sort(([a, _], [b, __]) => a - b);
  }

  find(target: number): [string, number] | undefined {
    // binary search for addr
    if (this.byAddr.length === 0) return undefined;
    let lo = 0, hi = this.byAddr.length;
    while (lo < hi - 1) {
      const mid = Math.floor((lo + hi) / 2);
      const [cur, label] = this.byAddr[mid];
      if (cur < target) {
        lo = mid;
      } else if (cur > target) {
        hi = mid;
      } else if (cur === target) {
        return [label, 0];
      }
    }
    const [cur, label] = this.byAddr[lo];
    if (cur <= target) {
      // Show the offset relative to the nearest labelled entry.
      const delta = target - cur;
      // We don't want very high addresses to appear as last+largenumber, so cap delta.
      if (delta < 0x100) {
        return [label, delta];
      }
    }
    return undefined;
  }

  get(addr: number): string | undefined {
    const match = this.find(addr);
    if (!match) return;
    let str = match[0];
    if (match[1]) str += `+${hex(match[1], 0)}`;
    return str;
  }
}
