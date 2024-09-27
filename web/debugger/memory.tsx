import * as preact from 'preact';
import { h } from 'preact';
import { hex } from './util';

/**
 * Props used for any place in the UI where we show hex numbers,
 * to add click handlers for addresses.
 */
export interface MemoryView {
  showMemory(addr: number): void;
  highlightMemory(addr: number): void;
}

/** Wraps a displayed number such that it can be hovered and clicked to show memory. */
export namespace Number {
  export interface Props extends MemoryView {
    /** How many digits to show, 2 by default */
    digits?: number;
    /** Text to show, hex of the number by default; used for `123h` rendering in disasm. */
    text?: string;
    children: number;
  }
}
export class Number extends preact.Component<Number.Props> {
  render() {
    let { digits, text, children: number } = this.props;
    if (text === undefined) text = hex(number, digits);
    return (
      <span
        class='number'
        onMouseOver={() => {
          this.props.highlightMemory(number);
        }}
        onClick={() => {
          this.props.showMemory(number);
        }}
      >
        {text}
      </span>
    );
  }
}

function isPrintable(byte: number): boolean {
  return byte >= 0x20 && byte < 0x7F;
}

namespace Memory {
  export interface Props {
    mem: DataView;
    base: number;
    highlight?: number;
    jumpTo: (addr: number) => void;
  }
}
export class Memory extends preact.Component<Memory.Props> {
  onSubmit = (e: Event) => {
    e.preventDefault();
    const form = e.target as HTMLFormElement;
    const addr = (form.elements.namedItem('addr') as HTMLInputElement).value;
    this.props.jumpTo(parseInt(addr, 16));
  };

  jump(e: PointerEvent, direction: number) {
    let step = 0x100;
    if (e.shiftKey) step *= 0x10;
    if (e.altKey) step *= 0x100;
    step *= direction;
    this.props.jumpTo(this.props.base + step);
  }
  onJumpForward = (e: Event) => {
    this.jump(e as PointerEvent, 1);
  };
  onJumpBack = (e: Event) => {
    this.jump(e as PointerEvent, -1);
  };

  render() {
    const { mem } = this.props;

    const addrs = [];
    const hexRows = [];
    const asciiRows = [];

    const base = this.props.base & ~0xf;
    // Somehow the above can go negative on overflow(?).
    if (base >= 0) {
      for (let rowAddr = 0; rowAddr < 0x100; rowAddr += 0x10) {
        if (base + rowAddr >= mem.byteLength) break;
        addrs.push(<div>{hex(base + rowAddr, 8)}</div>);

        const hexRow = [];
        const asciiRow = [];
        for (let offset = 0; offset < 0x10; offset++) {
          const addr = base + rowAddr + offset;
          if (addr >= mem.byteLength) break;

          const klass = addr === this.props.highlight ? 'highlight' : undefined;
          const value = mem.getUint8(addr);
          const hexText = hex(value);
          hexRow.push(<span class={klass}>{hexText}</span>);

          const asciiText = isPrintable(value) ? String.fromCharCode(value) : '.';
          asciiRow.push(<span class={klass}>{asciiText}</span>);
        }
        hexRows.push(<div>{hexRow}</div>);
        asciiRows.push(<div>{asciiRow}</div>);
      }
    }

    return (
      <section style={{ minHeight: 0, overflow: 'hidden', display: 'flex', flexDirection: 'column', gap: '1ex' }}>
        <form style={{ display: 'flex', justifyContent: 'center' }} onSubmit={this.onSubmit}>
          <button type='button' onClick={this.onJumpBack}>&lt;</button>
          <input name='addr' size={8} value={hex(this.props.base, 8)} />
          <button type='button' onClick={this.onJumpForward}>&gt;</button>
        </form>
        <div style={{ display: 'flex', gap: '2ex' }}>
          <code>{addrs}</code>
          <code class='grid'>{hexRows}</code>
          <code>{asciiRows}</code>
        </div>
      </section>
    );
  }
}
