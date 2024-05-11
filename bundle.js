var __defProp = Object.defineProperty;
var __defNormalProp = (obj, key, value) => key in obj ? __defProp(obj, key, { enumerable: true, configurable: true, writable: true, value }) : obj[key] = value;
var __publicField = (obj, key, value) => {
  __defNormalProp(obj, typeof key !== "symbol" ? key + "" : key, value);
  return value;
};

// node_modules/preact/dist/preact.module.js
var n;
var l;
var u;
var i;
var t;
var o;
var r;
var f = {};
var e = [];
var c = /acit|ex(?:s|g|n|p|$)|rph|grid|ows|mnc|ntw|ine[ch]|zoo|^ord|itera/i;
function s(n2, l2) {
  for (var u2 in l2)
    n2[u2] = l2[u2];
  return n2;
}
function a(n2) {
  var l2 = n2.parentNode;
  l2 && l2.removeChild(n2);
}
function h(l2, u2, i2) {
  var t2, o2, r2, f2 = {};
  for (r2 in u2)
    "key" == r2 ? t2 = u2[r2] : "ref" == r2 ? o2 = u2[r2] : f2[r2] = u2[r2];
  if (arguments.length > 2 && (f2.children = arguments.length > 3 ? n.call(arguments, 2) : i2), "function" == typeof l2 && null != l2.defaultProps)
    for (r2 in l2.defaultProps)
      void 0 === f2[r2] && (f2[r2] = l2.defaultProps[r2]);
  return v(l2, f2, t2, o2, null);
}
function v(n2, i2, t2, o2, r2) {
  var f2 = { type: n2, props: i2, key: t2, ref: o2, __k: null, __: null, __b: 0, __e: null, __d: void 0, __c: null, __h: null, constructor: void 0, __v: null == r2 ? ++u : r2 };
  return null == r2 && null != l.vnode && l.vnode(f2), f2;
}
function y() {
  return { current: null };
}
function p(n2) {
  return n2.children;
}
function d(n2, l2) {
  this.props = n2, this.context = l2;
}
function _(n2, l2) {
  if (null == l2)
    return n2.__ ? _(n2.__, n2.__.__k.indexOf(n2) + 1) : null;
  for (var u2; l2 < n2.__k.length; l2++)
    if (null != (u2 = n2.__k[l2]) && null != u2.__e)
      return u2.__e;
  return "function" == typeof n2.type ? _(n2) : null;
}
function k(n2) {
  var l2, u2;
  if (null != (n2 = n2.__) && null != n2.__c) {
    for (n2.__e = n2.__c.base = null, l2 = 0; l2 < n2.__k.length; l2++)
      if (null != (u2 = n2.__k[l2]) && null != u2.__e) {
        n2.__e = n2.__c.base = u2.__e;
        break;
      }
    return k(n2);
  }
}
function b(n2) {
  (!n2.__d && (n2.__d = true) && t.push(n2) && !g.__r++ || o !== l.debounceRendering) && ((o = l.debounceRendering) || setTimeout)(g);
}
function g() {
  for (var n2; g.__r = t.length; )
    n2 = t.sort(function(n3, l2) {
      return n3.__v.__b - l2.__v.__b;
    }), t = [], n2.some(function(n3) {
      var l2, u2, i2, t2, o2, r2;
      n3.__d && (o2 = (t2 = (l2 = n3).__v).__e, (r2 = l2.__P) && (u2 = [], (i2 = s({}, t2)).__v = t2.__v + 1, j(r2, t2, i2, l2.__n, void 0 !== r2.ownerSVGElement, null != t2.__h ? [o2] : null, u2, null == o2 ? _(t2) : o2, t2.__h), z(u2, t2), t2.__e != o2 && k(t2)));
    });
}
function w(n2, l2, u2, i2, t2, o2, r2, c2, s2, a2) {
  var h2, y2, d2, k2, b2, g2, w2, x = i2 && i2.__k || e, C2 = x.length;
  for (u2.__k = [], h2 = 0; h2 < l2.length; h2++)
    if (null != (k2 = u2.__k[h2] = null == (k2 = l2[h2]) || "boolean" == typeof k2 ? null : "string" == typeof k2 || "number" == typeof k2 || "bigint" == typeof k2 ? v(null, k2, null, null, k2) : Array.isArray(k2) ? v(p, { children: k2 }, null, null, null) : k2.__b > 0 ? v(k2.type, k2.props, k2.key, k2.ref ? k2.ref : null, k2.__v) : k2)) {
      if (k2.__ = u2, k2.__b = u2.__b + 1, null === (d2 = x[h2]) || d2 && k2.key == d2.key && k2.type === d2.type)
        x[h2] = void 0;
      else
        for (y2 = 0; y2 < C2; y2++) {
          if ((d2 = x[y2]) && k2.key == d2.key && k2.type === d2.type) {
            x[y2] = void 0;
            break;
          }
          d2 = null;
        }
      j(n2, k2, d2 = d2 || f, t2, o2, r2, c2, s2, a2), b2 = k2.__e, (y2 = k2.ref) && d2.ref != y2 && (w2 || (w2 = []), d2.ref && w2.push(d2.ref, null, k2), w2.push(y2, k2.__c || b2, k2)), null != b2 ? (null == g2 && (g2 = b2), "function" == typeof k2.type && k2.__k === d2.__k ? k2.__d = s2 = m(k2, s2, n2) : s2 = A(n2, k2, d2, x, b2, s2), "function" == typeof u2.type && (u2.__d = s2)) : s2 && d2.__e == s2 && s2.parentNode != n2 && (s2 = _(d2));
    }
  for (u2.__e = g2, h2 = C2; h2--; )
    null != x[h2] && ("function" == typeof u2.type && null != x[h2].__e && x[h2].__e == u2.__d && (u2.__d = _(i2, h2 + 1)), N(x[h2], x[h2]));
  if (w2)
    for (h2 = 0; h2 < w2.length; h2++)
      M(w2[h2], w2[++h2], w2[++h2]);
}
function m(n2, l2, u2) {
  for (var i2, t2 = n2.__k, o2 = 0; t2 && o2 < t2.length; o2++)
    (i2 = t2[o2]) && (i2.__ = n2, l2 = "function" == typeof i2.type ? m(i2, l2, u2) : A(u2, i2, i2, t2, i2.__e, l2));
  return l2;
}
function A(n2, l2, u2, i2, t2, o2) {
  var r2, f2, e2;
  if (void 0 !== l2.__d)
    r2 = l2.__d, l2.__d = void 0;
  else if (null == u2 || t2 != o2 || null == t2.parentNode)
    n:
      if (null == o2 || o2.parentNode !== n2)
        n2.appendChild(t2), r2 = null;
      else {
        for (f2 = o2, e2 = 0; (f2 = f2.nextSibling) && e2 < i2.length; e2 += 2)
          if (f2 == t2)
            break n;
        n2.insertBefore(t2, o2), r2 = o2;
      }
  return void 0 !== r2 ? r2 : t2.nextSibling;
}
function C(n2, l2, u2, i2, t2) {
  var o2;
  for (o2 in u2)
    "children" === o2 || "key" === o2 || o2 in l2 || H(n2, o2, null, u2[o2], i2);
  for (o2 in l2)
    t2 && "function" != typeof l2[o2] || "children" === o2 || "key" === o2 || "value" === o2 || "checked" === o2 || u2[o2] === l2[o2] || H(n2, o2, l2[o2], u2[o2], i2);
}
function $(n2, l2, u2) {
  "-" === l2[0] ? n2.setProperty(l2, u2) : n2[l2] = null == u2 ? "" : "number" != typeof u2 || c.test(l2) ? u2 : u2 + "px";
}
function H(n2, l2, u2, i2, t2) {
  var o2;
  n:
    if ("style" === l2)
      if ("string" == typeof u2)
        n2.style.cssText = u2;
      else {
        if ("string" == typeof i2 && (n2.style.cssText = i2 = ""), i2)
          for (l2 in i2)
            u2 && l2 in u2 || $(n2.style, l2, "");
        if (u2)
          for (l2 in u2)
            i2 && u2[l2] === i2[l2] || $(n2.style, l2, u2[l2]);
      }
    else if ("o" === l2[0] && "n" === l2[1])
      o2 = l2 !== (l2 = l2.replace(/Capture$/, "")), l2 = l2.toLowerCase() in n2 ? l2.toLowerCase().slice(2) : l2.slice(2), n2.l || (n2.l = {}), n2.l[l2 + o2] = u2, u2 ? i2 || n2.addEventListener(l2, o2 ? T : I, o2) : n2.removeEventListener(l2, o2 ? T : I, o2);
    else if ("dangerouslySetInnerHTML" !== l2) {
      if (t2)
        l2 = l2.replace(/xlink(H|:h)/, "h").replace(/sName$/, "s");
      else if ("href" !== l2 && "list" !== l2 && "form" !== l2 && "tabIndex" !== l2 && "download" !== l2 && l2 in n2)
        try {
          n2[l2] = null == u2 ? "" : u2;
          break n;
        } catch (n3) {
        }
      "function" == typeof u2 || (null != u2 && (false !== u2 || "a" === l2[0] && "r" === l2[1]) ? n2.setAttribute(l2, u2) : n2.removeAttribute(l2));
    }
}
function I(n2) {
  this.l[n2.type + false](l.event ? l.event(n2) : n2);
}
function T(n2) {
  this.l[n2.type + true](l.event ? l.event(n2) : n2);
}
function j(n2, u2, i2, t2, o2, r2, f2, e2, c2) {
  var a2, h2, v2, y2, _2, k2, b2, g2, m2, x, A2, C2, $2, H2 = u2.type;
  if (void 0 !== u2.constructor)
    return null;
  null != i2.__h && (c2 = i2.__h, e2 = u2.__e = i2.__e, u2.__h = null, r2 = [e2]), (a2 = l.__b) && a2(u2);
  try {
    n:
      if ("function" == typeof H2) {
        if (g2 = u2.props, m2 = (a2 = H2.contextType) && t2[a2.__c], x = a2 ? m2 ? m2.props.value : a2.__ : t2, i2.__c ? b2 = (h2 = u2.__c = i2.__c).__ = h2.__E : ("prototype" in H2 && H2.prototype.render ? u2.__c = h2 = new H2(g2, x) : (u2.__c = h2 = new d(g2, x), h2.constructor = H2, h2.render = O), m2 && m2.sub(h2), h2.props = g2, h2.state || (h2.state = {}), h2.context = x, h2.__n = t2, v2 = h2.__d = true, h2.__h = []), null == h2.__s && (h2.__s = h2.state), null != H2.getDerivedStateFromProps && (h2.__s == h2.state && (h2.__s = s({}, h2.__s)), s(h2.__s, H2.getDerivedStateFromProps(g2, h2.__s))), y2 = h2.props, _2 = h2.state, v2)
          null == H2.getDerivedStateFromProps && null != h2.componentWillMount && h2.componentWillMount(), null != h2.componentDidMount && h2.__h.push(h2.componentDidMount);
        else {
          if (null == H2.getDerivedStateFromProps && g2 !== y2 && null != h2.componentWillReceiveProps && h2.componentWillReceiveProps(g2, x), !h2.__e && null != h2.shouldComponentUpdate && false === h2.shouldComponentUpdate(g2, h2.__s, x) || u2.__v === i2.__v) {
            h2.props = g2, h2.state = h2.__s, u2.__v !== i2.__v && (h2.__d = false), h2.__v = u2, u2.__e = i2.__e, u2.__k = i2.__k, u2.__k.forEach(function(n3) {
              n3 && (n3.__ = u2);
            }), h2.__h.length && f2.push(h2);
            break n;
          }
          null != h2.componentWillUpdate && h2.componentWillUpdate(g2, h2.__s, x), null != h2.componentDidUpdate && h2.__h.push(function() {
            h2.componentDidUpdate(y2, _2, k2);
          });
        }
        if (h2.context = x, h2.props = g2, h2.__v = u2, h2.__P = n2, A2 = l.__r, C2 = 0, "prototype" in H2 && H2.prototype.render)
          h2.state = h2.__s, h2.__d = false, A2 && A2(u2), a2 = h2.render(h2.props, h2.state, h2.context);
        else
          do {
            h2.__d = false, A2 && A2(u2), a2 = h2.render(h2.props, h2.state, h2.context), h2.state = h2.__s;
          } while (h2.__d && ++C2 < 25);
        h2.state = h2.__s, null != h2.getChildContext && (t2 = s(s({}, t2), h2.getChildContext())), v2 || null == h2.getSnapshotBeforeUpdate || (k2 = h2.getSnapshotBeforeUpdate(y2, _2)), $2 = null != a2 && a2.type === p && null == a2.key ? a2.props.children : a2, w(n2, Array.isArray($2) ? $2 : [$2], u2, i2, t2, o2, r2, f2, e2, c2), h2.base = u2.__e, u2.__h = null, h2.__h.length && f2.push(h2), b2 && (h2.__E = h2.__ = null), h2.__e = false;
      } else
        null == r2 && u2.__v === i2.__v ? (u2.__k = i2.__k, u2.__e = i2.__e) : u2.__e = L(i2.__e, u2, i2, t2, o2, r2, f2, c2);
    (a2 = l.diffed) && a2(u2);
  } catch (n3) {
    u2.__v = null, (c2 || null != r2) && (u2.__e = e2, u2.__h = !!c2, r2[r2.indexOf(e2)] = null), l.__e(n3, u2, i2);
  }
}
function z(n2, u2) {
  l.__c && l.__c(u2, n2), n2.some(function(u3) {
    try {
      n2 = u3.__h, u3.__h = [], n2.some(function(n3) {
        n3.call(u3);
      });
    } catch (n3) {
      l.__e(n3, u3.__v);
    }
  });
}
function L(l2, u2, i2, t2, o2, r2, e2, c2) {
  var s2, h2, v2, y2 = i2.props, p2 = u2.props, d2 = u2.type, k2 = 0;
  if ("svg" === d2 && (o2 = true), null != r2) {
    for (; k2 < r2.length; k2++)
      if ((s2 = r2[k2]) && "setAttribute" in s2 == !!d2 && (d2 ? s2.localName === d2 : 3 === s2.nodeType)) {
        l2 = s2, r2[k2] = null;
        break;
      }
  }
  if (null == l2) {
    if (null === d2)
      return document.createTextNode(p2);
    l2 = o2 ? document.createElementNS("http://www.w3.org/2000/svg", d2) : document.createElement(d2, p2.is && p2), r2 = null, c2 = false;
  }
  if (null === d2)
    y2 === p2 || c2 && l2.data === p2 || (l2.data = p2);
  else {
    if (r2 = r2 && n.call(l2.childNodes), h2 = (y2 = i2.props || f).dangerouslySetInnerHTML, v2 = p2.dangerouslySetInnerHTML, !c2) {
      if (null != r2)
        for (y2 = {}, k2 = 0; k2 < l2.attributes.length; k2++)
          y2[l2.attributes[k2].name] = l2.attributes[k2].value;
      (v2 || h2) && (v2 && (h2 && v2.__html == h2.__html || v2.__html === l2.innerHTML) || (l2.innerHTML = v2 && v2.__html || ""));
    }
    if (C(l2, p2, y2, o2, c2), v2)
      u2.__k = [];
    else if (k2 = u2.props.children, w(l2, Array.isArray(k2) ? k2 : [k2], u2, i2, t2, o2 && "foreignObject" !== d2, r2, e2, r2 ? r2[0] : i2.__k && _(i2, 0), c2), null != r2)
      for (k2 = r2.length; k2--; )
        null != r2[k2] && a(r2[k2]);
    c2 || ("value" in p2 && void 0 !== (k2 = p2.value) && (k2 !== l2.value || "progress" === d2 && !k2 || "option" === d2 && k2 !== y2.value) && H(l2, "value", k2, y2.value, false), "checked" in p2 && void 0 !== (k2 = p2.checked) && k2 !== l2.checked && H(l2, "checked", k2, y2.checked, false));
  }
  return l2;
}
function M(n2, u2, i2) {
  try {
    "function" == typeof n2 ? n2(u2) : n2.current = u2;
  } catch (n3) {
    l.__e(n3, i2);
  }
}
function N(n2, u2, i2) {
  var t2, o2;
  if (l.unmount && l.unmount(n2), (t2 = n2.ref) && (t2.current && t2.current !== n2.__e || M(t2, null, u2)), null != (t2 = n2.__c)) {
    if (t2.componentWillUnmount)
      try {
        t2.componentWillUnmount();
      } catch (n3) {
        l.__e(n3, u2);
      }
    t2.base = t2.__P = null, n2.__c = void 0;
  }
  if (t2 = n2.__k)
    for (o2 = 0; o2 < t2.length; o2++)
      t2[o2] && N(t2[o2], u2, "function" != typeof n2.type);
  i2 || null == n2.__e || a(n2.__e), n2.__ = n2.__e = n2.__d = void 0;
}
function O(n2, l2, u2) {
  return this.constructor(n2, u2);
}
function P(u2, i2, t2) {
  var o2, r2, e2;
  l.__ && l.__(u2, i2), r2 = (o2 = "function" == typeof t2) ? null : t2 && t2.__k || i2.__k, e2 = [], j(i2, u2 = (!o2 && t2 || i2).__k = h(p, null, [u2]), r2 || f, f, void 0 !== i2.ownerSVGElement, !o2 && t2 ? [t2] : r2 ? null : i2.firstChild ? n.call(i2.childNodes) : null, e2, !o2 && t2 ? t2 : r2 ? r2.__e : i2.firstChild, o2), z(e2, u2);
}
n = e.slice, l = { __e: function(n2, l2, u2, i2) {
  for (var t2, o2, r2; l2 = l2.__; )
    if ((t2 = l2.__c) && !t2.__)
      try {
        if ((o2 = t2.constructor) && null != o2.getDerivedStateFromError && (t2.setState(o2.getDerivedStateFromError(n2)), r2 = t2.__d), null != t2.componentDidCatch && (t2.componentDidCatch(n2, i2 || {}), r2 = t2.__d), r2)
          return t2.__E = t2;
      } catch (l3) {
        n2 = l3;
      }
  throw n2;
} }, u = 0, i = function(n2) {
  return null != n2 && void 0 === n2.constructor;
}, d.prototype.setState = function(n2, l2) {
  var u2;
  u2 = null != this.__s && this.__s !== this.state ? this.__s : this.__s = s({}, this.state), "function" == typeof n2 && (n2 = n2(s({}, u2), this.props)), n2 && s(u2, n2), null != n2 && this.__v && (l2 && this.__h.push(l2), b(this));
}, d.prototype.forceUpdate = function(n2) {
  this.__v && (this.__e = true, n2 && this.__h.push(n2), b(this));
}, d.prototype.render = p, t = [], g.__r = 0, r = 0;

// util.ts
function hex(i2, digits = 2) {
  return i2.toString(16).padStart(digits, "0");
}

// memory.tsx
var Number = class extends d {
  render() {
    return /* @__PURE__ */ h("span", {
      class: "clicky",
      title: "show in memory dump",
      onMouseOver: () => {
        this.props.highlightMemory(this.props.children);
      },
      onClick: (event) => {
        this.props.showMemory(this.props.children);
      }
    }, this.props.text ? this.props.text : hex(this.props.children, this.props.digits));
  }
};
var Memory = class extends d {
  constructor() {
    super(...arguments);
    __publicField(this, "onSubmit", (e2) => {
      e2.preventDefault();
      const form = e2.target;
      const addr = form.elements.namedItem("addr").value;
      this.props.jumpTo(parseInt(addr, 16));
    });
    __publicField(this, "onJumpForward", (e2) => {
      this.jump(e2, 1);
    });
    __publicField(this, "onJumpBack", (e2) => {
      this.jump(e2, -1);
    });
  }
  jump(e2, direction) {
    let step = 256;
    if (e2.shiftKey)
      step *= 16;
    if (e2.altKey)
      step *= 256;
    step *= direction;
    this.props.jumpTo(this.props.base + step);
  }
  render() {
    let rows = [];
    const base = this.props.base & ~15;
    if (base >= 0) {
      for (let rowAddr = 0; rowAddr < 256; rowAddr += 16) {
        if (base + rowAddr >= this.props.mem.byteLength)
          break;
        const row = [];
        row.push(hex(base + rowAddr, 8));
        for (let offset = 0; offset < 16; offset++) {
          const addr = base + rowAddr + offset;
          if (addr >= this.props.mem.byteLength)
            break;
          if (offset % 4 === 0)
            row.push("  ");
          else
            row.push(" ");
          let value = hex(this.props.mem.getUint8(addr));
          if (addr === this.props.highlight) {
            value = /* @__PURE__ */ h("span", {
              class: "highlight"
            }, value);
          }
          row.push(value);
        }
        rows.push(/* @__PURE__ */ h("div", null, row));
      }
    }
    return /* @__PURE__ */ h("div", null, /* @__PURE__ */ h("form", {
      style: { display: "flex", justifyContent: "center" },
      onSubmit: this.onSubmit
    }, /* @__PURE__ */ h("button", {
      type: "button",
      onClick: this.onJumpBack
    }, "<"), /* @__PURE__ */ h("input", {
      name: "addr",
      size: 8,
      value: hex(this.props.base, 8)
    }), /* @__PURE__ */ h("button", {
      type: "button",
      onClick: this.onJumpForward
    }, ">")), /* @__PURE__ */ h("code", null, rows));
  }
};

// break.tsx
var BreakpointsComponent = class extends d {
  render() {
    const rows = [];
    for (const bp of this.props.breakpoints) {
      const className = bp.addr === this.props.highlight ? "highlight" : void 0;
      const label = this.props.labels.get(bp.addr);
      rows.push(
        /* @__PURE__ */ h("div", {
          className,
          style: { display: "flex", alignItems: "center", gap: "0.5ex" }
        }, /* @__PURE__ */ h("input", {
          type: "checkbox",
          checked: !bp.disabled,
          onChange: () => this.props.toggle(bp.addr)
        }), /* @__PURE__ */ h("div", null, /* @__PURE__ */ h("code", null, /* @__PURE__ */ h(Number, {
          digits: 8,
          ...this.props
        }, bp.addr))), bp.oneShot ? "[once]" : null, label ? /* @__PURE__ */ h("div", null, "(", /* @__PURE__ */ h("code", null, label), ")") : null, /* @__PURE__ */ h("button", {
          class: "x",
          onClick: () => this.props.remove(bp.addr)
        }, "x"))
      );
    }
    return /* @__PURE__ */ h("section", null, rows, /* @__PURE__ */ h(AddComponent, {
      onAccept: (text) => this.props.add(text)
    }));
  }
};
var AddComponent = class extends d {
  constructor() {
    super(...arguments);
    __publicField(this, "onInput", (ev) => {
      const text = ev.target.value;
      this.setState({ text });
    });
    __publicField(this, "onSubmit", (ev) => {
      ev.preventDefault();
      if (this.props.onAccept(this.state.text)) {
        this.setState({ text: "" });
      }
    });
  }
  render() {
    return /* @__PURE__ */ h("form", {
      onSubmit: this.onSubmit
    }, "add: ", /* @__PURE__ */ h("input", {
      value: this.state.text,
      onInput: this.onInput
    }));
  }
};

// code.tsx
var Code = class extends d {
  render() {
    const instrs = this.props.instrs.map((instr) => {
      let code = instr.code.map(({ kind, text }) => {
        switch (kind) {
          case "FunctionAddress":
          case "LabelAddress":
          case "Number": {
            const addr = parseInt(text, 16);
            let label = this.props.labels.get(addr);
            if (label) {
              label = ` ${label}`;
            }
            return /* @__PURE__ */ h(p, null, /* @__PURE__ */ h(Number, {
              text,
              ...this.props
            }, addr), label);
          }
          default:
            return text;
        }
      });
      return /* @__PURE__ */ h("tr", null, /* @__PURE__ */ h("td", {
        style: { width: "10ch" }
      }, /* @__PURE__ */ h("span", {
        class: "clicky",
        title: "run to this address",
        onClick: (event) => {
          this.props.runTo(instr.addr);
        }
      }, hex(instr.addr, 8))), /* @__PURE__ */ h("td", {
        title: `${instr.bytes} (${instr.ops.join(",")})`
      }, code));
    });
    return /* @__PURE__ */ h("section", {
      class: "code"
    }, /* @__PURE__ */ h("code", {
      class: "disassembly"
    }, /* @__PURE__ */ h("table", null, instrs)));
  }
};

// mappings.tsx
var Mappings = class extends d {
  render() {
    const rows = this.props.mappings.map((mapping) => {
      let className;
      const highlight = this.props.highlight;
      if (highlight !== void 0 && highlight >= mapping.addr && highlight < mapping.addr + mapping.size) {
        className = "highlight";
      }
      return /* @__PURE__ */ h("tr", {
        class: className
      }, /* @__PURE__ */ h("td", {
        style: { width: "10ch" }
      }, hex(mapping.addr, 8)), /* @__PURE__ */ h("td", {
        style: { width: "8ch" }
      }, hex(mapping.size)), /* @__PURE__ */ h("td", null, mapping.desc));
    });
    return /* @__PURE__ */ h("section", null, /* @__PURE__ */ h("code", null, /* @__PURE__ */ h("table", null, /* @__PURE__ */ h("thead", null, /* @__PURE__ */ h("tr", null, /* @__PURE__ */ h("td", null, "addr"), /* @__PURE__ */ h("td", null, "size"), /* @__PURE__ */ h("td", null, "desc"))), rows)));
  }
};

// registers.tsx
var RegistersComponent = class extends d {
  render() {
    const { regs } = this.props;
    const st = regs.st;
    return /* @__PURE__ */ h("section", {
      class: "panel"
    }, /* @__PURE__ */ h("code", null, /* @__PURE__ */ h("div", null, "eax\xA0", /* @__PURE__ */ h(Number, {
      digits: 8,
      ...this.props
    }, regs.eax), /* @__PURE__ */ h("br", null), "ebx\xA0", /* @__PURE__ */ h(Number, {
      digits: 8,
      ...this.props
    }, regs.ebx), /* @__PURE__ */ h("br", null), "ecx\xA0", /* @__PURE__ */ h(Number, {
      digits: 8,
      ...this.props
    }, regs.ecx), /* @__PURE__ */ h("br", null), "edx\xA0", /* @__PURE__ */ h(Number, {
      digits: 8,
      ...this.props
    }, regs.edx), /* @__PURE__ */ h("br", null)), /* @__PURE__ */ h("br", null), /* @__PURE__ */ h("div", null, "eip\xA0", /* @__PURE__ */ h(Number, {
      digits: 8,
      ...this.props
    }, regs.eip), /* @__PURE__ */ h("br", null), "esp\xA0", /* @__PURE__ */ h(Number, {
      digits: 8,
      ...this.props
    }, regs.esp), /* @__PURE__ */ h("br", null), "ebp\xA0", /* @__PURE__ */ h(Number, {
      digits: 8,
      ...this.props
    }, regs.ebp), /* @__PURE__ */ h("br", null), "esi\xA0", /* @__PURE__ */ h(Number, {
      digits: 8,
      ...this.props
    }, regs.esi), /* @__PURE__ */ h("br", null), "edi\xA0", /* @__PURE__ */ h(Number, {
      digits: 8,
      ...this.props
    }, regs.edi), /* @__PURE__ */ h("br", null)), /* @__PURE__ */ h("br", null), /* @__PURE__ */ h("div", null, "cs\xA0", /* @__PURE__ */ h(Number, {
      digits: 4,
      ...this.props
    }, regs.cs), " ", "fs\xA0", /* @__PURE__ */ h(Number, {
      digits: 4,
      ...this.props
    }, regs.fs), /* @__PURE__ */ h("br", null), "ds\xA0", /* @__PURE__ */ h(Number, {
      digits: 4,
      ...this.props
    }, regs.ds), " ", "gs\xA0", /* @__PURE__ */ h(Number, {
      digits: 4,
      ...this.props
    }, regs.gs), /* @__PURE__ */ h("br", null), "es\xA0", /* @__PURE__ */ h(Number, {
      digits: 4,
      ...this.props
    }, regs.es), " ", "ss\xA0", /* @__PURE__ */ h(Number, {
      digits: 4,
      ...this.props
    }, regs.ss), /* @__PURE__ */ h("br", null)), /* @__PURE__ */ h("br", null), /* @__PURE__ */ h("div", null, "flags\xA0", hex(regs.flags), " ", regs.flags_str), /* @__PURE__ */ h("br", null), st.length > 0 ? /* @__PURE__ */ h("div", null, "fpu", /* @__PURE__ */ h("br", null), Array.from(regs.st).map((n2) => /* @__PURE__ */ h("span", null, n2, /* @__PURE__ */ h("br", null)))) : null));
  }
};

// snapshots.tsx
function idbRequest(req) {
  return new Promise((res, rej) => {
    req.onerror = (err) => {
      rej(err);
    };
    req.onsuccess = function() {
      res(this.result);
    };
  });
}
function finishTransaction(t2) {
  return new Promise((res, rej) => {
    t2.onerror = (err) => {
      rej(err);
    };
    t2.oncomplete = function() {
      res();
    };
  });
}
var SnapshotsComponent = class extends d {
  async load() {
    const req = indexedDB.open("retrowin32");
    req.onupgradeneeded = () => {
      const db2 = req.result;
      db2.createObjectStore("image", { autoIncrement: true });
      db2.createObjectStore("snap", { keyPath: "idbKey" });
    };
    const db = await idbRequest(req);
    const snaps = await idbRequest(db.transaction("snap").objectStore("snap").getAll());
    const st = { db, snaps };
    this.setState({ state: "ok", data: st });
    db.onerror = (error) => {
      this.setState({
        state: "error",
        data: error.toString()
      });
    };
    return st;
  }
  componentDidMount() {
    this.load();
  }
  render() {
    if (this.state.state === "ok") {
      return /* @__PURE__ */ h(Snapshots, {
        ...this.props,
        ...this.state.data,
        reload: () => this.load()
      });
    } else if (this.state.state === "error") {
      return /* @__PURE__ */ h("section", null, "error: ", this.state.data);
    } else {
      return /* @__PURE__ */ h("section", null, "loading");
    }
  }
};
var Snapshots = class extends d {
  async save() {
    const image = this.props.take();
    const t2 = this.props.db.transaction(["snap", "image"], "readwrite");
    const idbKey = await idbRequest(t2.objectStore("image").add(image));
    const snap = { idbKey, size: image.length };
    await idbRequest(t2.objectStore("snap").add(snap));
    await finishTransaction(t2);
    this.props.reload();
  }
  async load(key) {
    const t2 = this.props.db.transaction(["image"], "readonly");
    const image = await idbRequest(t2.objectStore("image").get(key));
    await finishTransaction(t2);
    this.props.load(image);
  }
  async clear() {
    const t2 = this.props.db.transaction(["snap", "image"], "readwrite");
    t2.objectStore("snap").clear();
    t2.objectStore("image").clear();
    await finishTransaction(t2);
    this.props.reload();
  }
  render() {
    let snaps = [];
    if (this.props.snaps.length > 0) {
      for (const snap of this.props.snaps) {
        snaps.push(
          /* @__PURE__ */ h("div", null, snap.size, " bytes ", /* @__PURE__ */ h("button", {
            onClick: () => this.load(snap.idbKey)
          }, "load"))
        );
      }
      snaps.push(
        /* @__PURE__ */ h("p", null, /* @__PURE__ */ h("button", {
          onClick: () => this.clear()
        }, "clear snapshots"))
      );
    }
    return /* @__PURE__ */ h("section", null, /* @__PURE__ */ h("p", null, /* @__PURE__ */ h("button", {
      onClick: () => this.save()
    }, "save snapshot")), snaps);
  }
};

// stack.tsx
var Stack = class extends d {
  render() {
    const { emu } = this.props;
    const esp = emu.esp;
    const memory = emu.memory();
    const rows = [];
    for (let addr = esp - 16; addr < esp + 32; addr += 4) {
      const value = memory.getUint32(addr, true);
      let label = this.props.labels.get(value);
      if (label) {
        label = ` ${label}`;
      }
      let row = /* @__PURE__ */ h("div", null, /* @__PURE__ */ h(Number, {
        digits: 8,
        ...this.props
      }, addr), "\xA0", /* @__PURE__ */ h(Number, {
        digits: 8,
        ...this.props
      }, value), label);
      if (addr === esp) {
        row = /* @__PURE__ */ h("b", null, row);
      }
      rows.push(row);
    }
    return /* @__PURE__ */ h("section", {
      class: "panel"
    }, /* @__PURE__ */ h("code", null, rows));
  }
};

// tabs.tsx
var Tabs = class extends d {
  constructor(props) {
    super(props);
    this.state = { cur: Object.keys(props.tabs)[0] };
  }
  render() {
    const tabs = this.props.tabs;
    return /* @__PURE__ */ h("section", {
      class: "panel",
      style: this.props.style
    }, /* @__PURE__ */ h("div", {
      class: "tabs-strip"
    }, Object.keys(tabs).map((name) => {
      let button = /* @__PURE__ */ h("span", {
        class: "clicky",
        onClick: () => this.props.switchTab(name)
      }, name);
      if (name === this.props.selected) {
        button = /* @__PURE__ */ h("b", null, button);
      }
      return /* @__PURE__ */ h(p, null, "\xA0|\xA0", button);
    })), tabs[this.props.selected]);
  }
};

// glue/pkg/glue.js
var wasm;
var cachedTextDecoder = typeof TextDecoder !== "undefined" ? new TextDecoder("utf-8", { ignoreBOM: true, fatal: true }) : { decode: () => {
  throw Error("TextDecoder not available");
} };
if (typeof TextDecoder !== "undefined") {
  cachedTextDecoder.decode();
}
var cachedUint8Memory0 = null;
function getUint8Memory0() {
  if (cachedUint8Memory0 === null || cachedUint8Memory0.byteLength === 0) {
    cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
  }
  return cachedUint8Memory0;
}
function getStringFromWasm0(ptr, len) {
  ptr = ptr >>> 0;
  return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}
function isLikeNone(x) {
  return x === void 0 || x === null;
}
var cachedFloat64Memory0 = null;
function getFloat64Memory0() {
  if (cachedFloat64Memory0 === null || cachedFloat64Memory0.byteLength === 0) {
    cachedFloat64Memory0 = new Float64Array(wasm.memory.buffer);
  }
  return cachedFloat64Memory0;
}
var cachedInt32Memory0 = null;
function getInt32Memory0() {
  if (cachedInt32Memory0 === null || cachedInt32Memory0.byteLength === 0) {
    cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
  }
  return cachedInt32Memory0;
}
function debugString(val) {
  const type = typeof val;
  if (type == "number" || type == "boolean" || val == null) {
    return `${val}`;
  }
  if (type == "string") {
    return `"${val}"`;
  }
  if (type == "symbol") {
    const description = val.description;
    if (description == null) {
      return "Symbol";
    } else {
      return `Symbol(${description})`;
    }
  }
  if (type == "function") {
    const name = val.name;
    if (typeof name == "string" && name.length > 0) {
      return `Function(${name})`;
    } else {
      return "Function";
    }
  }
  if (Array.isArray(val)) {
    const length = val.length;
    let debug = "[";
    if (length > 0) {
      debug += debugString(val[0]);
    }
    for (let i2 = 1; i2 < length; i2++) {
      debug += ", " + debugString(val[i2]);
    }
    debug += "]";
    return debug;
  }
  const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
  let className;
  if (builtInMatches.length > 1) {
    className = builtInMatches[1];
  } else {
    return toString.call(val);
  }
  if (className == "Object") {
    try {
      return "Object(" + JSON.stringify(val) + ")";
    } catch (_2) {
      return "Object";
    }
  }
  if (val instanceof Error) {
    return `${val.name}: ${val.message}
${val.stack}`;
  }
  return className;
}
var WASM_VECTOR_LEN = 0;
var cachedTextEncoder = typeof TextEncoder !== "undefined" ? new TextEncoder("utf-8") : { encode: () => {
  throw Error("TextEncoder not available");
} };
var encodeString = typeof cachedTextEncoder.encodeInto === "function" ? function(arg, view) {
  return cachedTextEncoder.encodeInto(arg, view);
} : function(arg, view) {
  const buf = cachedTextEncoder.encode(arg);
  view.set(buf);
  return {
    read: arg.length,
    written: buf.length
  };
};
function passStringToWasm0(arg, malloc, realloc) {
  if (realloc === void 0) {
    const buf = cachedTextEncoder.encode(arg);
    const ptr2 = malloc(buf.length, 1) >>> 0;
    getUint8Memory0().subarray(ptr2, ptr2 + buf.length).set(buf);
    WASM_VECTOR_LEN = buf.length;
    return ptr2;
  }
  let len = arg.length;
  let ptr = malloc(len, 1) >>> 0;
  const mem = getUint8Memory0();
  let offset = 0;
  for (; offset < len; offset++) {
    const code = arg.charCodeAt(offset);
    if (code > 127)
      break;
    mem[ptr + offset] = code;
  }
  if (offset !== len) {
    if (offset !== 0) {
      arg = arg.slice(offset);
    }
    ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
    const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
    const ret = encodeString(arg, view);
    offset += ret.written;
    ptr = realloc(ptr, len, offset, 1) >>> 0;
  }
  WASM_VECTOR_LEN = offset;
  return ptr;
}
function passArray8ToWasm0(arg, malloc) {
  const ptr = malloc(arg.length * 1, 1) >>> 0;
  getUint8Memory0().set(arg, ptr / 1);
  WASM_VECTOR_LEN = arg.length;
  return ptr;
}
function takeFromExternrefTable0(idx) {
  const value = wasm.__wbindgen_export_2.get(idx);
  wasm.__externref_table_dealloc(idx);
  return value;
}
function getArrayU8FromWasm0(ptr, len) {
  ptr = ptr >>> 0;
  return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}
function new_emulator(host, cmdline) {
  const ptr0 = passStringToWasm0(cmdline, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
  const len0 = WASM_VECTOR_LEN;
  const ret = wasm.new_emulator(host, ptr0, len0);
  return Emulator.__wrap(ret);
}
function addToExternrefTable0(obj) {
  const idx = wasm.__externref_table_alloc();
  wasm.__wbindgen_export_2.set(idx, obj);
  return idx;
}
function handleError(f2, args) {
  try {
    return f2.apply(this, args);
  } catch (e2) {
    const idx = addToExternrefTable0(e2);
    wasm.__wbindgen_exn_store(idx);
  }
}
var cachedUint8ClampedMemory0 = null;
function getUint8ClampedMemory0() {
  if (cachedUint8ClampedMemory0 === null || cachedUint8ClampedMemory0.byteLength === 0) {
    cachedUint8ClampedMemory0 = new Uint8ClampedArray(wasm.memory.buffer);
  }
  return cachedUint8ClampedMemory0;
}
function getClampedArrayU8FromWasm0(ptr, len) {
  ptr = ptr >>> 0;
  return getUint8ClampedMemory0().subarray(ptr / 1, ptr / 1 + len);
}
var CPUState = Object.freeze({ Running: 0, "0": "Running", Blocked: 1, "1": "Blocked", Error: 2, "2": "Error", Exit: 3, "3": "Exit" });
var EmulatorFinalization = typeof FinalizationRegistry === "undefined" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((ptr) => wasm.__wbg_emulator_free(ptr >>> 0));
var Emulator = class {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(Emulator.prototype);
    obj.__wbg_ptr = ptr;
    EmulatorFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    EmulatorFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_emulator_free(ptr);
  }
  load_exe(name, buf, relocate) {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
      const len0 = WASM_VECTOR_LEN;
      const ptr1 = passArray8ToWasm0(buf, wasm.__wbindgen_malloc);
      const len1 = WASM_VECTOR_LEN;
      wasm.emulator_load_exe(retptr, this.__wbg_ptr, ptr0, len0, ptr1, len1, relocate);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      if (r1) {
        throw takeFromExternrefTable0(r0);
      }
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  labels() {
    let deferred2_0;
    let deferred2_1;
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.emulator_labels(retptr, this.__wbg_ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      var r3 = getInt32Memory0()[retptr / 4 + 3];
      var ptr1 = r0;
      var len1 = r1;
      if (r3) {
        ptr1 = 0;
        len1 = 0;
        throw takeFromExternrefTable0(r2);
      }
      deferred2_0 = ptr1;
      deferred2_1 = len1;
      return getStringFromWasm0(ptr1, len1);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
      wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
    }
  }
  memory() {
    const ret = wasm.emulator_memory(this.__wbg_ptr);
    return ret;
  }
  get esp() {
    const ret = wasm.emulator_esp(this.__wbg_ptr);
    return ret >>> 0;
  }
  get eip() {
    const ret = wasm.emulator_eip(this.__wbg_ptr);
    return ret >>> 0;
  }
  regs() {
    const ret = wasm.emulator_regs(this.__wbg_ptr);
    return ret;
  }
  get instr_count() {
    const ret = wasm.emulator_instr_count(this.__wbg_ptr);
    return ret >>> 0;
  }
  disassemble_json(addr, limit) {
    let deferred1_0;
    let deferred1_1;
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.emulator_disassemble_json(retptr, this.__wbg_ptr, addr, limit);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      deferred1_0 = r0;
      deferred1_1 = r1;
      return getStringFromWasm0(r0, r1);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
      wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
  }
  unblock() {
    wasm.emulator_unblock(this.__wbg_ptr);
  }
  run(count) {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.emulator_run(retptr, this.__wbg_ptr, count);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var r2 = getInt32Memory0()[retptr / 4 + 2];
      if (r2) {
        throw takeFromExternrefTable0(r1);
      }
      return r0;
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  breakpoint_add(addr) {
    wasm.emulator_breakpoint_add(this.__wbg_ptr, addr);
  }
  breakpoint_clear(addr) {
    wasm.emulator_breakpoint_clear(this.__wbg_ptr, addr);
  }
  mappings_json() {
    let deferred1_0;
    let deferred1_1;
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.emulator_mappings_json(retptr, this.__wbg_ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      deferred1_0 = r0;
      deferred1_1 = r1;
      return getStringFromWasm0(r0, r1);
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
      wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
  }
  poke(addr, value) {
    wasm.emulator_poke(this.__wbg_ptr, addr, value);
  }
  snapshot() {
    try {
      const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
      wasm.emulator_snapshot(retptr, this.__wbg_ptr);
      var r0 = getInt32Memory0()[retptr / 4 + 0];
      var r1 = getInt32Memory0()[retptr / 4 + 1];
      var v1 = getArrayU8FromWasm0(r0, r1).slice();
      wasm.__wbindgen_free(r0, r1 * 1, 1);
      return v1;
    } finally {
      wasm.__wbindgen_add_to_stack_pointer(16);
    }
  }
  load_snapshot(bytes) {
    const ptr0 = passArray8ToWasm0(bytes, wasm.__wbindgen_malloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.emulator_load_snapshot(this.__wbg_ptr, ptr0, len0);
  }
  set_tracing_scheme(scheme) {
    const ptr0 = passStringToWasm0(scheme, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.emulator_set_tracing_scheme(this.__wbg_ptr, ptr0, len0);
  }
};
var SurfaceOptionsFinalization = typeof FinalizationRegistry === "undefined" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((ptr) => wasm.__wbg_surfaceoptions_free(ptr >>> 0));
async function __wbg_load(module, imports) {
  if (typeof Response === "function" && module instanceof Response) {
    if (typeof WebAssembly.instantiateStreaming === "function") {
      try {
        return await WebAssembly.instantiateStreaming(module, imports);
      } catch (e2) {
        if (module.headers.get("Content-Type") != "application/wasm") {
          console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e2);
        } else {
          throw e2;
        }
      }
    }
    const bytes = await module.arrayBuffer();
    return await WebAssembly.instantiate(bytes, imports);
  } else {
    const instance = await WebAssembly.instantiate(module, imports);
    if (instance instanceof WebAssembly.Instance) {
      return { instance, module };
    } else {
      return instance;
    }
  }
}
function __wbg_get_imports() {
  const imports = {};
  imports.wbg = {};
  imports.wbg.__wbindgen_error_new = function(arg0, arg1) {
    const ret = new Error(getStringFromWasm0(arg0, arg1));
    return ret;
  };
  imports.wbg.__wbindgen_memory = function() {
    const ret = wasm.memory;
    return ret;
  };
  imports.wbg.__wbg_buffer_12d079cc21e14bdb = function(arg0) {
    const ret = arg0.buffer;
    return ret;
  };
  imports.wbg.__wbg_byteLength_2e8dcbbe54bdad62 = function(arg0) {
    const ret = arg0.byteLength;
    return ret;
  };
  imports.wbg.__wbg_new_6308304d72aede55 = function(arg0, arg1, arg2) {
    const ret = new DataView(arg0, arg1 >>> 0, arg2 >>> 0);
    return ret;
  };
  imports.wbg.__wbg_parse_66d1801634e099ac = function() {
    return handleError(function(arg0, arg1) {
      const ret = JSON.parse(getStringFromWasm0(arg0, arg1));
      return ret;
    }, arguments);
  };
  imports.wbg.__wbg_exit_42080a4462444014 = function(arg0, arg1) {
    arg0.exit(arg1 >>> 0);
  };
  imports.wbg.__wbg_instanceof_Window_f401953a2cf86220 = function(arg0) {
    let result;
    try {
      result = arg0 instanceof Window;
    } catch (_2) {
      result = false;
    }
    const ret = result;
    return ret;
  };
  imports.wbg.__wbg_performance_3298a9628a5c8aa4 = function(arg0) {
    const ret = arg0.performance;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
  };
  imports.wbg.__wbg_now_4e659b3d15f470d9 = function(arg0) {
    const ret = arg0.now();
    return ret;
  };
  imports.wbg.__wbg_getevent_4f4de425a52104de = function(arg0) {
    const ret = arg0.get_event();
    return ret;
  };
  imports.wbg.__wbindgen_is_undefined = function(arg0) {
    const ret = arg0 === void 0;
    return ret;
  };
  imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
    const ret = getStringFromWasm0(arg0, arg1);
    return ret;
  };
  imports.wbg.__wbg_get_e3c254076557e348 = function() {
    return handleError(function(arg0, arg1) {
      const ret = Reflect.get(arg0, arg1);
      return ret;
    }, arguments);
  };
  imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
    const obj = arg1;
    const ret = typeof obj === "number" ? obj : void 0;
    getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
    getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
  };
  imports.wbg.__wbg_type_c7f33162571befe7 = function(arg0, arg1) {
    const ret = arg1.type;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
  };
  imports.wbg.__wbg_button_367cdc7303e3cf9b = function(arg0) {
    const ret = arg0.button;
    return ret;
  };
  imports.wbg.__wbg_offsetX_1a40c03298c0d8b6 = function(arg0) {
    const ret = arg0.offsetX;
    return ret;
  };
  imports.wbg.__wbg_offsetY_f75e8c25b9d9b679 = function(arg0) {
    const ret = arg0.offsetY;
    return ret;
  };
  imports.wbg.__wbg_ensuretimer_752ace6fb471cdd4 = function(arg0, arg1) {
    arg0.ensure_timer(arg1 >>> 0);
  };
  imports.wbg.__wbg_open_61490d64358619c7 = function(arg0, arg1, arg2) {
    const ret = arg0.open(getStringFromWasm0(arg1, arg2));
    return ret;
  };
  imports.wbg.__wbg_info_2551d10805917111 = function(arg0) {
    const ret = arg0.info();
    return ret;
  };
  imports.wbg.__wbg_seek_c5471dc2ba4d64bc = function(arg0, arg1) {
    const ret = arg0.seek(arg1 >>> 0);
    return ret;
  };
  imports.wbg.__wbg_read_ca96830ec9aacdcf = function(arg0, arg1, arg2) {
    const ret = arg0.read(getArrayU8FromWasm0(arg1, arg2));
    return ret;
  };
  imports.wbg.__wbg_write_61e5d5b79d83ffda = function(arg0, arg1, arg2) {
    const ret = arg0.write(getArrayU8FromWasm0(arg1, arg2));
    return ret;
  };
  imports.wbg.__wbg_createwindow_79bbfe483866ee8c = function(arg0, arg1) {
    const ret = arg0.create_window(arg1 >>> 0);
    return ret;
  };
  imports.wbg.__wbg_settitle_b48ee927b9814f5c = function(arg0, arg1, arg2) {
    arg0.title = getStringFromWasm0(arg1, arg2);
  };
  imports.wbg.__wbg_setsize_7bcb3132fd38238f = function(arg0, arg1, arg2) {
    arg0.set_size(arg1 >>> 0, arg2 >>> 0);
  };
  imports.wbg.__wbg_screen_0825c896804feac9 = function(arg0) {
    const ret = arg0.screen();
    return ret;
  };
  imports.wbg.__wbg_document_5100775d18896c16 = function(arg0) {
    const ret = arg0.document;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
  };
  imports.wbg.__wbg_createElement_8bae7856a4bb7411 = function() {
    return handleError(function(arg0, arg1, arg2) {
      const ret = arg0.createElement(getStringFromWasm0(arg1, arg2));
      return ret;
    }, arguments);
  };
  imports.wbg.__wbg_setwidth_080107476e633963 = function(arg0, arg1) {
    arg0.width = arg1 >>> 0;
  };
  imports.wbg.__wbg_setheight_dc240617639f1f51 = function(arg0, arg1) {
    arg0.height = arg1 >>> 0;
  };
  imports.wbg.__wbg_getContext_df50fa48a8876636 = function() {
    return handleError(function(arg0, arg1, arg2) {
      const ret = arg0.getContext(getStringFromWasm0(arg1, arg2));
      return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    }, arguments);
  };
  imports.wbg.__wbg_instanceof_CanvasRenderingContext2d_20bf99ccc051643b = function(arg0) {
    let result;
    try {
      result = arg0 instanceof CanvasRenderingContext2D;
    } catch (_2) {
      result = false;
    }
    const ret = result;
    return ret;
  };
  imports.wbg.__wbg_setfillStyle_4de94b275f5761f2 = function(arg0, arg1) {
    arg0.fillStyle = arg1;
  };
  imports.wbg.__wbg_fillRect_b5c8166281bac9df = function(arg0, arg1, arg2, arg3, arg4) {
    arg0.fillRect(arg1, arg2, arg3, arg4);
  };
  imports.wbg.__wbg_fill_7f376d2e52c3054e = function(arg0) {
    arg0.fill();
  };
  imports.wbg.__wbg_newwithu8clampedarray_ae824147b27925fc = function() {
    return handleError(function(arg0, arg1, arg2) {
      const ret = new ImageData(getClampedArrayU8FromWasm0(arg0, arg1), arg2 >>> 0);
      return ret;
    }, arguments);
  };
  imports.wbg.__wbg_putImageData_044c08ad889366e1 = function() {
    return handleError(function(arg0, arg1, arg2, arg3) {
      arg0.putImageData(arg1, arg2, arg3);
    }, arguments);
  };
  imports.wbg.__wbg_drawImage_26ad546f3bb64a22 = function() {
    return handleError(function(arg0, arg1, arg2, arg3) {
      arg0.drawImage(arg1, arg2, arg3);
    }, arguments);
  };
  imports.wbg.__wbg_drawImage_5a754349d9fbf4a6 = function() {
    return handleError(function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9) {
      arg0.drawImage(arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9);
    }, arguments);
  };
  imports.wbg.__wbg_log_21bd4d15c3d236fe = function(arg0, arg1, arg2, arg3) {
    let deferred0_0;
    let deferred0_1;
    try {
      deferred0_0 = arg2;
      deferred0_1 = arg3;
      arg0.log(arg1, getStringFromWasm0(arg2, arg3));
    } finally {
      wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
  };
  imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
    const ret = debugString(arg1);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
  };
  imports.wbg.__wbg_self_ce0dbfc45cf2f5be = function() {
    return handleError(function() {
      const ret = self.self;
      return ret;
    }, arguments);
  };
  imports.wbg.__wbg_window_c6fb939a7f436783 = function() {
    return handleError(function() {
      const ret = window.window;
      return ret;
    }, arguments);
  };
  imports.wbg.__wbg_globalThis_d1e6af4856ba331b = function() {
    return handleError(function() {
      const ret = globalThis.globalThis;
      return ret;
    }, arguments);
  };
  imports.wbg.__wbg_global_207b558942527489 = function() {
    return handleError(function() {
      const ret = global.global;
      return ret;
    }, arguments);
  };
  imports.wbg.__wbg_newnoargs_e258087cd0daa0ea = function(arg0, arg1) {
    const ret = new Function(getStringFromWasm0(arg0, arg1));
    return ret;
  };
  imports.wbg.__wbg_call_27c0f87801dedf93 = function() {
    return handleError(function(arg0, arg1) {
      const ret = arg0.call(arg1);
      return ret;
    }, arguments);
  };
  imports.wbg.__wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
  };
  imports.wbg.__wbindgen_init_externref_table = function() {
    const table = wasm.__wbindgen_export_2;
    const offset = table.grow(4);
    table.set(0, void 0);
    table.set(offset + 0, void 0);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
    ;
  };
  return imports;
}
function __wbg_init_memory(imports, maybe_memory) {
}
function __wbg_finalize_init(instance, module) {
  wasm = instance.exports;
  __wbg_init.__wbindgen_wasm_module = module;
  cachedFloat64Memory0 = null;
  cachedInt32Memory0 = null;
  cachedUint8Memory0 = null;
  cachedUint8ClampedMemory0 = null;
  wasm.__wbindgen_start();
  return wasm;
}
async function __wbg_init(input) {
  if (wasm !== void 0)
    return wasm;
  if (typeof input === "undefined") {
    input = new URL("glue_bg.wasm", import.meta.url);
  }
  const imports = __wbg_get_imports();
  if (typeof input === "string" || typeof Request === "function" && input instanceof Request || typeof URL === "function" && input instanceof URL) {
    input = fetch(input);
  }
  __wbg_init_memory(imports);
  const { instance, module } = await __wbg_load(await input, imports);
  return __wbg_finalize_init(instance, module);
}
var glue_default = __wbg_init;

// host.ts
async function fetchBytes(path) {
  const resp = await fetch(path);
  if (!resp.ok)
    throw new Error(`failed to load ${path}`);
  return new Uint8Array(await resp.arrayBuffer());
}
var Window2 = class {
  constructor(jsHost, hwnd) {
    this.jsHost = jsHost;
    this.hwnd = hwnd;
    __publicField(this, "title", "");
    __publicField(this, "canvas", document.createElement("canvas"));
    const stashEvent = (ev) => {
      ev.hwnd = hwnd;
      jsHost.enqueueEvent(ev);
      return false;
    };
    this.canvas.onmousedown = stashEvent;
    this.canvas.onmouseup = stashEvent;
    this.canvas.oncontextmenu = (ev) => {
      return false;
    };
  }
  set_size(w2, h2) {
    this.canvas.width = w2 * window.devicePixelRatio;
    this.canvas.height = h2 * window.devicePixelRatio;
    this.canvas.style.width = `${w2}px`;
    this.canvas.style.height = `${h2}px`;
    const ctx = this.canvas.getContext("2d");
    ctx.reset();
    ctx.imageSmoothingEnabled = false;
    ctx.scale(window.devicePixelRatio, window.devicePixelRatio);
    this.jsHost.emuHost.onWindowChanged();
  }
};
var File = class {
  constructor(path, bytes) {
    this.path = path;
    this.bytes = bytes;
    __publicField(this, "ofs", 0);
  }
  info() {
    return this.bytes.length;
  }
  seek(ofs) {
    this.ofs = ofs;
    return true;
  }
  read(buf) {
    const n2 = Math.min(buf.length, this.bytes.length - this.ofs);
    buf.set(this.bytes.subarray(this.ofs, this.ofs + n2));
    this.ofs += n2;
    return n2;
  }
};
async function fetchFileSet(files, dir = "") {
  const fileset = /* @__PURE__ */ new Map();
  for (const file of files) {
    const path = dir + file;
    fileset.set(file, await fetchBytes(path));
  }
  return fileset;
}
var JsHost = class {
  constructor(emuHost, files) {
    this.emuHost = emuHost;
    this.files = files;
    __publicField(this, "events", []);
    __publicField(this, "timer");
    __publicField(this, "decoder", new TextDecoder());
    __publicField(this, "windows", []);
  }
  log(level, msg) {
    switch (level) {
      case 5:
        console.error(msg);
        this.emuHost.onError(msg);
        break;
      case 4:
        console.warn(msg);
        break;
      case 3:
        console.info(msg);
        break;
      case 2:
        console.log(msg);
        break;
      case 1:
        console.debug(msg);
        break;
      default:
        throw new Error(`unexpected log level #{level}`);
    }
  }
  exit(code) {
    this.emuHost.exit(code);
  }
  enqueueEvent(event) {
    this.events.push(event);
    this.start();
  }
  ensure_timer(when) {
    if (this.timer) {
      clearTimeout(this.timer);
    }
    const now = performance.now();
    console.log("timer for", when - now);
    const id = setTimeout(() => {
      if (this.timer !== id) {
        return;
      }
      this.timer = void 0;
      this.start();
    }, when - now);
    this.timer = id;
  }
  get_event() {
    return this.events.shift();
  }
  open(path) {
    let bytes = this.files.get(path);
    if (!bytes) {
      console.error(`unknown file ${path}, returning empty file`);
      bytes = new Uint8Array();
    }
    return new File(path, bytes);
  }
  write(buf) {
    const text = this.decoder.decode(buf);
    this.emuHost.onStdOut(text);
    return buf.length;
  }
  create_window(hwnd) {
    let window2 = new Window2(this, hwnd);
    this.windows.push(window2);
    this.emuHost.onWindowChanged();
    return window2;
  }
  screen() {
    return this.windows[this.windows.length - 1].canvas.getContext("2d");
  }
};

// labels.ts
function* parseCSV(text) {
  for (const line of text.split("\n")) {
    const [name, addr] = line.split("	");
    yield [parseInt(addr, 16), name];
  }
}
var Labels = class {
  constructor(labels) {
    __publicField(this, "byAddr");
    this.byAddr = Array.from(labels.entries());
    this.byAddr = this.byAddr.filter(([addr, _2]) => addr > 4096);
    this.byAddr.sort(([a2, _2], [b2, __]) => a2 - b2);
  }
  find(target) {
    if (this.byAddr.length === 0)
      return void 0;
    let lo = 0, hi = this.byAddr.length;
    while (lo < hi - 1) {
      const mid = Math.floor((lo + hi) / 2);
      const [cur2, label2] = this.byAddr[mid];
      if (cur2 < target) {
        lo = mid;
      } else if (cur2 > target) {
        hi = mid;
      } else if (cur2 === target) {
        return [label2, 0];
      }
    }
    const [cur, label] = this.byAddr[lo];
    if (cur <= target) {
      const delta = target - cur;
      if (delta < 4096) {
        return [label, delta];
      }
    }
    return void 0;
  }
  get(addr) {
    const match = this.find(addr);
    if (!match)
      return;
    let str = match[0];
    if (match[1])
      str += `+${hex(match[1], 0)}`;
    return str;
  }
};

// emulator.ts
var Emulator2 = class extends JsHost {
  constructor(host, files, storageKey, bytes, labels, relocate) {
    super(host, files);
    this.storageKey = storageKey;
    __publicField(this, "emu");
    __publicField(this, "breakpoints", /* @__PURE__ */ new Map());
    __publicField(this, "imports", []);
    __publicField(this, "labels");
    __publicField(this, "running", false);
    __publicField(this, "stepSize", 5e3);
    __publicField(this, "instrPerMs", 0);
    this.emu = new_emulator(this, storageKey);
    this.emu.load_exe(storageKey, bytes, relocate);
    const importsJSON = JSON.parse(this.emu.labels());
    for (const [jsAddr, jsName] of Object.entries(importsJSON)) {
      const addr = parseInt(jsAddr);
      const name = jsName;
      this.imports.push(`${hex(addr, 8)}: ${name}`);
      labels.set(addr, name);
    }
    this.labels = new Labels(labels);
    this.loadBreakpoints();
  }
  loadBreakpoints() {
    const json = window.localStorage.getItem(this.storageKey);
    if (!json)
      return;
    const bps = JSON.parse(json);
    for (const bp of bps) {
      this.breakpoints.set(bp.addr, bp);
    }
  }
  saveBreakpoints() {
    window.localStorage.setItem(this.storageKey, JSON.stringify(Array.from(this.breakpoints.values())));
  }
  addBreak(bp) {
    this.breakpoints.set(bp.addr, bp);
    this.saveBreakpoints();
  }
  addBreakByName(name) {
    for (const [addr, label] of this.labels.byAddr) {
      if (label === name) {
        this.addBreak({ addr });
        return true;
      }
    }
    if (name.match(/^[0-9a-fA-F]+$/)) {
      const addr = parseInt(name, 16);
      this.addBreak({ addr });
      return true;
    }
    return false;
  }
  delBreak(addr) {
    const bp = this.breakpoints.get(addr);
    if (!bp)
      return;
    this.breakpoints.delete(addr);
    this.saveBreakpoints();
  }
  toggleBreak(addr) {
    const bp = this.breakpoints.get(addr);
    bp.disabled = !bp.disabled;
    this.saveBreakpoints();
  }
  isAtBreakpoint() {
    const ip = this.emu.eip;
    const bp = this.breakpoints.get(ip);
    if (bp && !bp.disabled) {
      if (bp.oneShot) {
        this.delBreak(bp.addr);
      } else {
        this.emuHost.showTab("breakpoints");
      }
      return true;
    }
    return false;
  }
  step() {
    this.emu.unblock();
    this.emu.run(1);
  }
  runBatch() {
    const startTime = performance.now();
    const startSteps = this.emu.instr_count;
    const cpuState = this.emu.run(this.stepSize);
    const endTime = performance.now();
    const endSteps = this.emu.instr_count;
    const steps = endSteps - startSteps;
    if (steps > 1e3) {
      const deltaTime = endTime - startTime;
      const instrPerMs = steps / deltaTime;
      const alpha = 0.5;
      this.instrPerMs = alpha * instrPerMs + (1 - alpha) * this.instrPerMs;
      if (deltaTime < 8) {
        this.stepSize *= 2;
        console.log(`${steps} instructions in ${deltaTime.toFixed(0)}ms; adjusted step rate: ${this.stepSize}`);
      }
    }
    return cpuState;
  }
  stepMany() {
    for (const bp of this.breakpoints.values()) {
      if (!bp.disabled) {
        this.emu.breakpoint_add(bp.addr);
      }
    }
    const cpuState = this.runBatch();
    for (const bp of this.breakpoints.values()) {
      if (!bp.disabled) {
        this.emu.breakpoint_clear(bp.addr);
      }
    }
    if (this.isAtBreakpoint()) {
      return false;
    }
    return cpuState == CPUState.Running;
  }
  start() {
    if (this.running)
      return;
    this.emu.unblock();
    if (this.isAtBreakpoint()) {
      this.step();
    }
    this.running = true;
    this.runFrame();
  }
  runFrame() {
    if (!this.running)
      return;
    if (!this.stepMany()) {
      this.stop();
      return;
    }
    requestAnimationFrame(() => this.runFrame());
  }
  stop() {
    if (!this.running)
      return;
    this.running = false;
  }
  mappings() {
    return JSON.parse(this.emu.mappings_json());
  }
  disassemble(addr) {
    return JSON.parse(this.emu.disassemble_json(addr, 20));
  }
};

// web.tsx
var WindowComponent = class extends d {
  constructor() {
    super(...arguments);
    __publicField(this, "state", {
      pos: [200, 200]
    });
    __publicField(this, "ref", y());
    __publicField(this, "beginDrag", (e2) => {
      const node = e2.currentTarget;
      this.setState({ drag: [e2.offsetX, e2.offsetY] });
      node.setPointerCapture(e2.pointerId);
      e2.preventDefault();
    });
    __publicField(this, "onDrag", (e2) => {
      if (!this.state.drag)
        return;
      this.setState({ pos: [e2.clientX - this.state.drag[0], e2.clientY - this.state.drag[1]] });
      e2.preventDefault();
    });
    __publicField(this, "endDrag", (e2) => {
      const node = e2.currentTarget;
      this.setState({ drag: void 0 });
      node.releasePointerCapture(e2.pointerId);
      e2.preventDefault();
    });
  }
  ensureCanvas() {
    if (this.props.canvas && this.ref.current && !this.ref.current.firstChild) {
      this.ref.current.appendChild(this.props.canvas);
    }
  }
  componentDidMount() {
    this.ensureCanvas();
  }
  render() {
    this.ensureCanvas();
    return /* @__PURE__ */ h("div", {
      class: "window",
      style: { left: `${this.state.pos[0]}px`, top: `${this.state.pos[1]}px` }
    }, /* @__PURE__ */ h("div", {
      class: "titlebar",
      onPointerDown: this.beginDrag,
      onPointerUp: this.endDrag,
      onPointerMove: this.onDrag
    }, this.props.title), /* @__PURE__ */ h("div", {
      ref: this.ref
    }));
  }
};
var EmulatorComponent = class extends d {
  render() {
    return this.props.emulator.windows.map((window2) => {
      return /* @__PURE__ */ h(WindowComponent, {
        key: window2.hwnd,
        title: window2.title,
        canvas: window2.canvas
      });
    });
  }
};
function parseURL() {
  const query = new URLSearchParams(document.location.search);
  const exe = query.get("exe");
  if (!exe)
    return void 0;
  const dir = query.get("dir") || void 0;
  const files = query.getAll("file");
  const relocate = query.has("relocate");
  const params = { dir, exe, files, relocate };
  return params;
}
async function loadEmulator() {
  const params = parseURL();
  if (!params) {
    throw new Error("invalid URL params");
  }
  const fileset = await fetchFileSet([params.exe, ...params.files], params.dir);
  await glue_default(new URL("wasm.wasm", document.location.href));
  const csvLabels = /* @__PURE__ */ new Map();
  const resp = await fetch(params.exe + ".csv");
  if (resp.ok) {
    for (const [addr, name] of parseCSV(await resp.text())) {
      csvLabels.set(addr, name);
    }
  }
  const storageKey = (params.dir ?? "") + params.exe;
  return new Emulator2(
    null,
    fileset,
    storageKey,
    fileset.get(params.exe),
    csvLabels,
    params.relocate ?? false
  );
}

// debugger.tsx
var Debugger = class extends d {
  constructor(props) {
    super(props);
    __publicField(this, "state", { error: "", memBase: 4198400, selectedTab: "output" });
    __publicField(this, "highlightMemory", (addr) => this.setState({ memHighlight: addr }));
    __publicField(this, "showMemory", (memBase) => {
      this.setState({ selectedTab: "memory", memBase });
    });
    this.props.emulator.emuHost = this;
  }
  print(text) {
    this.setState((state) => ({ stdout: (state.stdout ?? "") + text }));
  }
  exit(code) {
    this.print(`
exited with code ${code}`);
    this.stop();
  }
  onWindowChanged() {
    this.forceUpdate();
  }
  showTab(name) {
    this.setState({ selectedTab: name });
  }
  onError(msg) {
    this.print(msg + "\n");
  }
  onStdOut(msg) {
    this.print(msg);
  }
  step() {
    try {
      this.props.emulator.step();
    } finally {
      this.forceUpdate();
    }
  }
  start() {
    if (this.state.running)
      return;
    this.setState({
      running: setInterval(() => {
        this.forceUpdate();
      }, 500)
    });
    this.props.emulator.start();
  }
  stop() {
    if (!this.state.running)
      return;
    this.props.emulator.stop();
    clearInterval(this.state.running);
    this.setState({ running: void 0 });
  }
  runTo(addr) {
    this.props.emulator.addBreak({ addr, oneShot: true });
    this.start();
  }
  render() {
    let instrs = [];
    let code;
    const eip = this.props.emulator.emu.eip;
    if (eip >= 4054253568) {
      const label = eip == 4294967280 ? "async" : this.props.emulator.labels.get(eip) ?? "shim";
      code = /* @__PURE__ */ h("section", {
        class: "code"
      }, "(in ", label, ")");
    } else {
      instrs = this.props.emulator.disassemble(eip);
      code = /* @__PURE__ */ h(Code, {
        instrs,
        labels: this.props.emulator.labels,
        highlightMemory: this.highlightMemory,
        showMemory: this.showMemory,
        runTo: (addr) => this.runTo(addr)
      });
    }
    return /* @__PURE__ */ h(p, null, /* @__PURE__ */ h(EmulatorComponent, {
      emulator: this.props.emulator
    }), /* @__PURE__ */ h("section", {
      class: "panel",
      style: { display: "flex", alignItems: "baseline" }
    }, /* @__PURE__ */ h("button", {
      onClick: () => this.state.running ? this.stop() : this.start()
    }, this.state.running ? "stop" : "run"), "\xA0", /* @__PURE__ */ h("button", {
      onClick: () => this.step()
    }, "step"), "\xA0", /* @__PURE__ */ h("button", {
      onClick: () => instrs ? this.runTo(instrs[1].addr) : this.step()
    }, "step over"), "\xA0", /* @__PURE__ */ h("div", null, this.props.emulator.emu.instr_count, " instrs executed | ", Math.floor(this.props.emulator.instrPerMs), "/ms")), /* @__PURE__ */ h("div", {
      style: { display: "flex", margin: "1ex" }
    }, code, /* @__PURE__ */ h("div", {
      style: { width: "12ex" }
    }), /* @__PURE__ */ h(RegistersComponent, {
      highlightMemory: this.highlightMemory,
      showMemory: this.showMemory,
      regs: this.props.emulator.emu.regs()
    })), /* @__PURE__ */ h("div", {
      style: { display: "flex" }
    }, /* @__PURE__ */ h(Tabs, {
      style: { width: "80ex" },
      tabs: {
        output: /* @__PURE__ */ h("div", null, /* @__PURE__ */ h("code", null, this.state.stdout, this.state.error ? /* @__PURE__ */ h("div", {
          class: "error"
        }, "ERROR: ", this.state.error) : null)),
        memory: /* @__PURE__ */ h(Memory, {
          mem: this.props.emulator.emu.memory(),
          base: this.state.memBase,
          highlight: this.state.memHighlight,
          jumpTo: (addr) => this.setState({ memBase: addr })
        }),
        mappings: /* @__PURE__ */ h(Mappings, {
          mappings: this.props.emulator.mappings(),
          highlight: this.state.memHighlight
        }),
        imports: /* @__PURE__ */ h("div", null, /* @__PURE__ */ h("code", null, this.props.emulator.imports.map((imp) => /* @__PURE__ */ h("div", null, imp)))),
        breakpoints: /* @__PURE__ */ h(BreakpointsComponent, {
          breakpoints: Array.from(this.props.emulator.breakpoints.values()),
          labels: this.props.emulator.labels,
          highlight: eip,
          highlightMemory: this.highlightMemory,
          showMemory: this.showMemory,
          toggle: (addr) => {
            this.props.emulator.toggleBreak(addr);
            this.forceUpdate();
          },
          add: (text) => {
            const ret = this.props.emulator.addBreakByName(text);
            this.forceUpdate();
            return ret;
          },
          remove: (addr) => {
            this.props.emulator.delBreak(addr);
            this.forceUpdate();
          }
        }),
        snapshots: /* @__PURE__ */ h(SnapshotsComponent, {
          take: () => this.props.emulator.emu.snapshot(),
          load: (snap) => {
            this.props.emulator.emu.load_snapshot(snap);
            this.forceUpdate();
          }
        })
      },
      selected: this.state.selectedTab,
      switchTab: (selectedTab) => this.setState({ selectedTab })
    }), /* @__PURE__ */ h(Stack, {
      highlightMemory: this.highlightMemory,
      showMemory: this.showMemory,
      labels: this.props.emulator.labels,
      emu: this.props.emulator.emu
    })));
  }
};
async function main() {
  const emulator = await loadEmulator();
  emulator.emu.set_tracing_scheme("*");
  P(/* @__PURE__ */ h(Debugger, {
    emulator
  }), document.body);
}

// run.tsx
var Runner = class extends d {
  constructor(props) {
    super(props);
    this.props.emulator.emuHost = this;
  }
  print(text) {
    this.setState((state) => ({ output: (state.output ?? "") + text }));
  }
  componentDidMount() {
    this.props.emulator.start();
  }
  exit(code) {
    this.print(`Exited with code ${code}`);
  }
  onWindowChanged() {
    this.forceUpdate();
  }
  showTab(name) {
    throw new Error("Method not implemented.");
  }
  onError(msg) {
    this.print(msg + "\n");
  }
  onStdOut(stdout) {
    this.print(stdout);
  }
  render() {
    return /* @__PURE__ */ h(p, null, this.state.output ? /* @__PURE__ */ h("pre", {
      class: "stdout"
    }, this.state.output) : null, /* @__PURE__ */ h(EmulatorComponent, {
      emulator: this.props.emulator
    }));
  }
};
async function main2() {
  const emulator = await loadEmulator();
  emulator.emu.set_tracing_scheme("-");
  P(/* @__PURE__ */ h(Runner, {
    emulator
  }), document.getElementById("main"));
}
export {
  main as debuggerMain,
  main2 as runMain
};
//# sourceMappingURL=bundle.js.map
