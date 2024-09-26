// node_modules/preact/dist/preact.module.js
var n;
var l;
var u;
var t;
var i;
var o;
var r;
var f;
var e;
var c;
var s;
var a;
var h = {};
var v = [];
var p = /acit|ex(?:s|g|n|p|$)|rph|grid|ows|mnc|ntw|ine[ch]|zoo|^ord|itera/i;
var y = Array.isArray;
function d(n2, l2) {
  for (var u2 in l2)
    n2[u2] = l2[u2];
  return n2;
}
function w(n2) {
  n2 && n2.parentNode && n2.parentNode.removeChild(n2);
}
function _(l2, u2, t2) {
  var i2, o2, r2, f2 = {};
  for (r2 in u2)
    "key" == r2 ? i2 = u2[r2] : "ref" == r2 ? o2 = u2[r2] : f2[r2] = u2[r2];
  if (arguments.length > 2 && (f2.children = arguments.length > 3 ? n.call(arguments, 2) : t2), "function" == typeof l2 && null != l2.defaultProps)
    for (r2 in l2.defaultProps)
      void 0 === f2[r2] && (f2[r2] = l2.defaultProps[r2]);
  return g(l2, f2, i2, o2, null);
}
function g(n2, t2, i2, o2, r2) {
  var f2 = { type: n2, props: t2, key: i2, ref: o2, __k: null, __: null, __b: 0, __e: null, __d: void 0, __c: null, constructor: void 0, __v: null == r2 ? ++u : r2, __i: -1, __u: 0 };
  return null == r2 && null != l.vnode && l.vnode(f2), f2;
}
function m() {
  return { current: null };
}
function b(n2) {
  return n2.children;
}
function k(n2, l2) {
  this.props = n2, this.context = l2;
}
function x(n2, l2) {
  if (null == l2)
    return n2.__ ? x(n2.__, n2.__i + 1) : null;
  for (var u2; l2 < n2.__k.length; l2++)
    if (null != (u2 = n2.__k[l2]) && null != u2.__e)
      return u2.__e;
  return "function" == typeof n2.type ? x(n2) : null;
}
function C(n2) {
  var l2, u2;
  if (null != (n2 = n2.__) && null != n2.__c) {
    for (n2.__e = n2.__c.base = null, l2 = 0; l2 < n2.__k.length; l2++)
      if (null != (u2 = n2.__k[l2]) && null != u2.__e) {
        n2.__e = n2.__c.base = u2.__e;
        break;
      }
    return C(n2);
  }
}
function M(n2) {
  (!n2.__d && (n2.__d = true) && i.push(n2) && !P.__r++ || o !== l.debounceRendering) && ((o = l.debounceRendering) || r)(P);
}
function P() {
  var n2, u2, t2, o2, r2, e2, c2, s2;
  for (i.sort(f); n2 = i.shift(); )
    n2.__d && (u2 = i.length, o2 = void 0, e2 = (r2 = (t2 = n2).__v).__e, c2 = [], s2 = [], t2.__P && ((o2 = d({}, r2)).__v = r2.__v + 1, l.vnode && l.vnode(o2), O(t2.__P, o2, r2, t2.__n, t2.__P.namespaceURI, 32 & r2.__u ? [e2] : null, c2, null == e2 ? x(r2) : e2, !!(32 & r2.__u), s2), o2.__v = r2.__v, o2.__.__k[o2.__i] = o2, j(c2, o2, s2), o2.__e != e2 && C(o2)), i.length > u2 && i.sort(f));
  P.__r = 0;
}
function S(n2, l2, u2, t2, i2, o2, r2, f2, e2, c2, s2) {
  var a2, p2, y2, d2, w2, _2 = t2 && t2.__k || v, g2 = l2.length;
  for (u2.__d = e2, $(u2, l2, _2), e2 = u2.__d, a2 = 0; a2 < g2; a2++)
    null != (y2 = u2.__k[a2]) && (p2 = -1 === y2.__i ? h : _2[y2.__i] || h, y2.__i = a2, O(n2, y2, p2, i2, o2, r2, f2, e2, c2, s2), d2 = y2.__e, y2.ref && p2.ref != y2.ref && (p2.ref && N(p2.ref, null, y2), s2.push(y2.ref, y2.__c || d2, y2)), null == w2 && null != d2 && (w2 = d2), 65536 & y2.__u || p2.__k === y2.__k ? e2 = I(y2, e2, n2) : "function" == typeof y2.type && void 0 !== y2.__d ? e2 = y2.__d : d2 && (e2 = d2.nextSibling), y2.__d = void 0, y2.__u &= -196609);
  u2.__d = e2, u2.__e = w2;
}
function $(n2, l2, u2) {
  var t2, i2, o2, r2, f2, e2 = l2.length, c2 = u2.length, s2 = c2, a2 = 0;
  for (n2.__k = [], t2 = 0; t2 < e2; t2++)
    null != (i2 = l2[t2]) && "boolean" != typeof i2 && "function" != typeof i2 ? (r2 = t2 + a2, (i2 = n2.__k[t2] = "string" == typeof i2 || "number" == typeof i2 || "bigint" == typeof i2 || i2.constructor == String ? g(null, i2, null, null, null) : y(i2) ? g(b, { children: i2 }, null, null, null) : void 0 === i2.constructor && i2.__b > 0 ? g(i2.type, i2.props, i2.key, i2.ref ? i2.ref : null, i2.__v) : i2).__ = n2, i2.__b = n2.__b + 1, o2 = null, -1 !== (f2 = i2.__i = L(i2, u2, r2, s2)) && (s2--, (o2 = u2[f2]) && (o2.__u |= 131072)), null == o2 || null === o2.__v ? (-1 == f2 && a2--, "function" != typeof i2.type && (i2.__u |= 65536)) : f2 !== r2 && (f2 == r2 - 1 ? a2-- : f2 == r2 + 1 ? a2++ : (f2 > r2 ? a2-- : a2++, i2.__u |= 65536))) : i2 = n2.__k[t2] = null;
  if (s2)
    for (t2 = 0; t2 < c2; t2++)
      null != (o2 = u2[t2]) && 0 == (131072 & o2.__u) && (o2.__e == n2.__d && (n2.__d = x(o2)), V(o2, o2));
}
function I(n2, l2, u2) {
  var t2, i2;
  if ("function" == typeof n2.type) {
    for (t2 = n2.__k, i2 = 0; t2 && i2 < t2.length; i2++)
      t2[i2] && (t2[i2].__ = n2, l2 = I(t2[i2], l2, u2));
    return l2;
  }
  n2.__e != l2 && (l2 && n2.type && !u2.contains(l2) && (l2 = x(n2)), u2.insertBefore(n2.__e, l2 || null), l2 = n2.__e);
  do {
    l2 = l2 && l2.nextSibling;
  } while (null != l2 && 8 === l2.nodeType);
  return l2;
}
function L(n2, l2, u2, t2) {
  var i2 = n2.key, o2 = n2.type, r2 = u2 - 1, f2 = u2 + 1, e2 = l2[u2];
  if (null === e2 || e2 && i2 == e2.key && o2 === e2.type && 0 == (131072 & e2.__u))
    return u2;
  if (t2 > (null != e2 && 0 == (131072 & e2.__u) ? 1 : 0))
    for (; r2 >= 0 || f2 < l2.length; ) {
      if (r2 >= 0) {
        if ((e2 = l2[r2]) && 0 == (131072 & e2.__u) && i2 == e2.key && o2 === e2.type)
          return r2;
        r2--;
      }
      if (f2 < l2.length) {
        if ((e2 = l2[f2]) && 0 == (131072 & e2.__u) && i2 == e2.key && o2 === e2.type)
          return f2;
        f2++;
      }
    }
  return -1;
}
function T(n2, l2, u2) {
  "-" === l2[0] ? n2.setProperty(l2, null == u2 ? "" : u2) : n2[l2] = null == u2 ? "" : "number" != typeof u2 || p.test(l2) ? u2 : u2 + "px";
}
function A(n2, l2, u2, t2, i2) {
  var o2;
  n:
    if ("style" === l2)
      if ("string" == typeof u2)
        n2.style.cssText = u2;
      else {
        if ("string" == typeof t2 && (n2.style.cssText = t2 = ""), t2)
          for (l2 in t2)
            u2 && l2 in u2 || T(n2.style, l2, "");
        if (u2)
          for (l2 in u2)
            t2 && u2[l2] === t2[l2] || T(n2.style, l2, u2[l2]);
      }
    else if ("o" === l2[0] && "n" === l2[1])
      o2 = l2 !== (l2 = l2.replace(/(PointerCapture)$|Capture$/i, "$1")), l2 = l2.toLowerCase() in n2 || "onFocusOut" === l2 || "onFocusIn" === l2 ? l2.toLowerCase().slice(2) : l2.slice(2), n2.l || (n2.l = {}), n2.l[l2 + o2] = u2, u2 ? t2 ? u2.u = t2.u : (u2.u = e, n2.addEventListener(l2, o2 ? s : c, o2)) : n2.removeEventListener(l2, o2 ? s : c, o2);
    else {
      if ("http://www.w3.org/2000/svg" == i2)
        l2 = l2.replace(/xlink(H|:h)/, "h").replace(/sName$/, "s");
      else if ("width" != l2 && "height" != l2 && "href" != l2 && "list" != l2 && "form" != l2 && "tabIndex" != l2 && "download" != l2 && "rowSpan" != l2 && "colSpan" != l2 && "role" != l2 && "popover" != l2 && l2 in n2)
        try {
          n2[l2] = null == u2 ? "" : u2;
          break n;
        } catch (n3) {
        }
      "function" == typeof u2 || (null == u2 || false === u2 && "-" !== l2[4] ? n2.removeAttribute(l2) : n2.setAttribute(l2, "popover" == l2 && 1 == u2 ? "" : u2));
    }
}
function F(n2) {
  return function(u2) {
    if (this.l) {
      var t2 = this.l[u2.type + n2];
      if (null == u2.t)
        u2.t = e++;
      else if (u2.t < t2.u)
        return;
      return t2(l.event ? l.event(u2) : u2);
    }
  };
}
function O(n2, u2, t2, i2, o2, r2, f2, e2, c2, s2) {
  var a2, h2, v2, p2, w2, _2, g2, m2, x2, C2, M2, P2, $2, I2, H, L2, T2 = u2.type;
  if (void 0 !== u2.constructor)
    return null;
  128 & t2.__u && (c2 = !!(32 & t2.__u), r2 = [e2 = u2.__e = t2.__e]), (a2 = l.__b) && a2(u2);
  n:
    if ("function" == typeof T2)
      try {
        if (m2 = u2.props, x2 = "prototype" in T2 && T2.prototype.render, C2 = (a2 = T2.contextType) && i2[a2.__c], M2 = a2 ? C2 ? C2.props.value : a2.__ : i2, t2.__c ? g2 = (h2 = u2.__c = t2.__c).__ = h2.__E : (x2 ? u2.__c = h2 = new T2(m2, M2) : (u2.__c = h2 = new k(m2, M2), h2.constructor = T2, h2.render = q), C2 && C2.sub(h2), h2.props = m2, h2.state || (h2.state = {}), h2.context = M2, h2.__n = i2, v2 = h2.__d = true, h2.__h = [], h2._sb = []), x2 && null == h2.__s && (h2.__s = h2.state), x2 && null != T2.getDerivedStateFromProps && (h2.__s == h2.state && (h2.__s = d({}, h2.__s)), d(h2.__s, T2.getDerivedStateFromProps(m2, h2.__s))), p2 = h2.props, w2 = h2.state, h2.__v = u2, v2)
          x2 && null == T2.getDerivedStateFromProps && null != h2.componentWillMount && h2.componentWillMount(), x2 && null != h2.componentDidMount && h2.__h.push(h2.componentDidMount);
        else {
          if (x2 && null == T2.getDerivedStateFromProps && m2 !== p2 && null != h2.componentWillReceiveProps && h2.componentWillReceiveProps(m2, M2), !h2.__e && (null != h2.shouldComponentUpdate && false === h2.shouldComponentUpdate(m2, h2.__s, M2) || u2.__v === t2.__v)) {
            for (u2.__v !== t2.__v && (h2.props = m2, h2.state = h2.__s, h2.__d = false), u2.__e = t2.__e, u2.__k = t2.__k, u2.__k.some(function(n3) {
              n3 && (n3.__ = u2);
            }), P2 = 0; P2 < h2._sb.length; P2++)
              h2.__h.push(h2._sb[P2]);
            h2._sb = [], h2.__h.length && f2.push(h2);
            break n;
          }
          null != h2.componentWillUpdate && h2.componentWillUpdate(m2, h2.__s, M2), x2 && null != h2.componentDidUpdate && h2.__h.push(function() {
            h2.componentDidUpdate(p2, w2, _2);
          });
        }
        if (h2.context = M2, h2.props = m2, h2.__P = n2, h2.__e = false, $2 = l.__r, I2 = 0, x2) {
          for (h2.state = h2.__s, h2.__d = false, $2 && $2(u2), a2 = h2.render(h2.props, h2.state, h2.context), H = 0; H < h2._sb.length; H++)
            h2.__h.push(h2._sb[H]);
          h2._sb = [];
        } else
          do {
            h2.__d = false, $2 && $2(u2), a2 = h2.render(h2.props, h2.state, h2.context), h2.state = h2.__s;
          } while (h2.__d && ++I2 < 25);
        h2.state = h2.__s, null != h2.getChildContext && (i2 = d(d({}, i2), h2.getChildContext())), x2 && !v2 && null != h2.getSnapshotBeforeUpdate && (_2 = h2.getSnapshotBeforeUpdate(p2, w2)), S(n2, y(L2 = null != a2 && a2.type === b && null == a2.key ? a2.props.children : a2) ? L2 : [L2], u2, t2, i2, o2, r2, f2, e2, c2, s2), h2.base = u2.__e, u2.__u &= -161, h2.__h.length && f2.push(h2), g2 && (h2.__E = h2.__ = null);
      } catch (n3) {
        if (u2.__v = null, c2 || null != r2) {
          for (u2.__u |= c2 ? 160 : 32; e2 && 8 === e2.nodeType && e2.nextSibling; )
            e2 = e2.nextSibling;
          r2[r2.indexOf(e2)] = null, u2.__e = e2;
        } else
          u2.__e = t2.__e, u2.__k = t2.__k;
        l.__e(n3, u2, t2);
      }
    else
      null == r2 && u2.__v === t2.__v ? (u2.__k = t2.__k, u2.__e = t2.__e) : u2.__e = z(t2.__e, u2, t2, i2, o2, r2, f2, c2, s2);
  (a2 = l.diffed) && a2(u2);
}
function j(n2, u2, t2) {
  u2.__d = void 0;
  for (var i2 = 0; i2 < t2.length; i2++)
    N(t2[i2], t2[++i2], t2[++i2]);
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
function z(u2, t2, i2, o2, r2, f2, e2, c2, s2) {
  var a2, v2, p2, d2, _2, g2, m2, b2 = i2.props, k2 = t2.props, C2 = t2.type;
  if ("svg" === C2 ? r2 = "http://www.w3.org/2000/svg" : "math" === C2 ? r2 = "http://www.w3.org/1998/Math/MathML" : r2 || (r2 = "http://www.w3.org/1999/xhtml"), null != f2) {
    for (a2 = 0; a2 < f2.length; a2++)
      if ((_2 = f2[a2]) && "setAttribute" in _2 == !!C2 && (C2 ? _2.localName === C2 : 3 === _2.nodeType)) {
        u2 = _2, f2[a2] = null;
        break;
      }
  }
  if (null == u2) {
    if (null === C2)
      return document.createTextNode(k2);
    u2 = document.createElementNS(r2, C2, k2.is && k2), c2 && (l.__m && l.__m(t2, f2), c2 = false), f2 = null;
  }
  if (null === C2)
    b2 === k2 || c2 && u2.data === k2 || (u2.data = k2);
  else {
    if (f2 = f2 && n.call(u2.childNodes), b2 = i2.props || h, !c2 && null != f2)
      for (b2 = {}, a2 = 0; a2 < u2.attributes.length; a2++)
        b2[(_2 = u2.attributes[a2]).name] = _2.value;
    for (a2 in b2)
      if (_2 = b2[a2], "children" == a2)
        ;
      else if ("dangerouslySetInnerHTML" == a2)
        p2 = _2;
      else if (!(a2 in k2)) {
        if ("value" == a2 && "defaultValue" in k2 || "checked" == a2 && "defaultChecked" in k2)
          continue;
        A(u2, a2, null, _2, r2);
      }
    for (a2 in k2)
      _2 = k2[a2], "children" == a2 ? d2 = _2 : "dangerouslySetInnerHTML" == a2 ? v2 = _2 : "value" == a2 ? g2 = _2 : "checked" == a2 ? m2 = _2 : c2 && "function" != typeof _2 || b2[a2] === _2 || A(u2, a2, _2, b2[a2], r2);
    if (v2)
      c2 || p2 && (v2.__html === p2.__html || v2.__html === u2.innerHTML) || (u2.innerHTML = v2.__html), t2.__k = [];
    else if (p2 && (u2.innerHTML = ""), S(u2, y(d2) ? d2 : [d2], t2, i2, o2, "foreignObject" === C2 ? "http://www.w3.org/1999/xhtml" : r2, f2, e2, f2 ? f2[0] : i2.__k && x(i2, 0), c2, s2), null != f2)
      for (a2 = f2.length; a2--; )
        w(f2[a2]);
    c2 || (a2 = "value", "progress" === C2 && null == g2 ? u2.removeAttribute("value") : void 0 !== g2 && (g2 !== u2[a2] || "progress" === C2 && !g2 || "option" === C2 && g2 !== b2[a2]) && A(u2, a2, g2, b2[a2], r2), a2 = "checked", void 0 !== m2 && m2 !== u2[a2] && A(u2, a2, m2, b2[a2], r2));
  }
  return u2;
}
function N(n2, u2, t2) {
  try {
    if ("function" == typeof n2) {
      var i2 = "function" == typeof n2.__u;
      i2 && n2.__u(), i2 && null == u2 || (n2.__u = n2(u2));
    } else
      n2.current = u2;
  } catch (n3) {
    l.__e(n3, t2);
  }
}
function V(n2, u2, t2) {
  var i2, o2;
  if (l.unmount && l.unmount(n2), (i2 = n2.ref) && (i2.current && i2.current !== n2.__e || N(i2, null, u2)), null != (i2 = n2.__c)) {
    if (i2.componentWillUnmount)
      try {
        i2.componentWillUnmount();
      } catch (n3) {
        l.__e(n3, u2);
      }
    i2.base = i2.__P = null;
  }
  if (i2 = n2.__k)
    for (o2 = 0; o2 < i2.length; o2++)
      i2[o2] && V(i2[o2], u2, t2 || "function" != typeof n2.type);
  t2 || w(n2.__e), n2.__c = n2.__ = n2.__e = n2.__d = void 0;
}
function q(n2, l2, u2) {
  return this.constructor(n2, u2);
}
function B(u2, t2, i2) {
  var o2, r2, f2, e2;
  l.__ && l.__(u2, t2), r2 = (o2 = "function" == typeof i2) ? null : i2 && i2.__k || t2.__k, f2 = [], e2 = [], O(t2, u2 = (!o2 && i2 || t2).__k = _(b, null, [u2]), r2 || h, h, t2.namespaceURI, !o2 && i2 ? [i2] : r2 ? null : t2.firstChild ? n.call(t2.childNodes) : null, f2, !o2 && i2 ? i2 : r2 ? r2.__e : t2.firstChild, o2, e2), j(f2, u2, e2);
}
n = v.slice, l = { __e: function(n2, l2, u2, t2) {
  for (var i2, o2, r2; l2 = l2.__; )
    if ((i2 = l2.__c) && !i2.__)
      try {
        if ((o2 = i2.constructor) && null != o2.getDerivedStateFromError && (i2.setState(o2.getDerivedStateFromError(n2)), r2 = i2.__d), null != i2.componentDidCatch && (i2.componentDidCatch(n2, t2 || {}), r2 = i2.__d), r2)
          return i2.__E = i2;
      } catch (l3) {
        n2 = l3;
      }
  throw n2;
} }, u = 0, t = function(n2) {
  return null != n2 && null == n2.constructor;
}, k.prototype.setState = function(n2, l2) {
  var u2;
  u2 = null != this.__s && this.__s !== this.state ? this.__s : this.__s = d({}, this.state), "function" == typeof n2 && (n2 = n2(d({}, u2), this.props)), n2 && d(u2, n2), null != n2 && this.__v && (l2 && this._sb.push(l2), M(this));
}, k.prototype.forceUpdate = function(n2) {
  this.__v && (this.__e = true, n2 && this.__h.push(n2), M(this));
}, k.prototype.render = b, i = [], r = "function" == typeof Promise ? Promise.prototype.then.bind(Promise.resolve()) : setTimeout, f = function(n2, l2) {
  return n2.__v.__b - l2.__v.__b;
}, P.__r = 0, e = 0, c = F(false), s = F(true), a = 0;

// debugger/util.ts
function hex(i2, digits = 2) {
  return i2.toString(16).padStart(digits, "0");
}

// debugger/labels.ts
function* parseCSV(text) {
  for (const line of text.split("\n")) {
    const [name, addr] = line.split("	");
    yield [parseInt(addr, 16), name];
  }
}
var Labels = class {
  byAddr;
  constructor(labels) {
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

// debugger/memory.tsx
var Number = class extends k {
  render() {
    return /* @__PURE__ */ _("span", {
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
var Memory = class extends k {
  onSubmit = (e2) => {
    e2.preventDefault();
    const form = e2.target;
    const addr = form.elements.namedItem("addr").value;
    this.props.jumpTo(parseInt(addr, 16));
  };
  jump(e2, direction) {
    let step = 256;
    if (e2.shiftKey)
      step *= 16;
    if (e2.altKey)
      step *= 256;
    step *= direction;
    this.props.jumpTo(this.props.base + step);
  }
  onJumpForward = (e2) => {
    this.jump(e2, 1);
  };
  onJumpBack = (e2) => {
    this.jump(e2, -1);
  };
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
            value = /* @__PURE__ */ _("span", {
              class: "highlight"
            }, value);
          }
          row.push(value);
        }
        rows.push(/* @__PURE__ */ _("div", null, row));
      }
    }
    return /* @__PURE__ */ _("div", null, /* @__PURE__ */ _("form", {
      style: { display: "flex", justifyContent: "center" },
      onSubmit: this.onSubmit
    }, /* @__PURE__ */ _("button", {
      type: "button",
      onClick: this.onJumpBack
    }, "<"), /* @__PURE__ */ _("input", {
      name: "addr",
      size: 8,
      value: hex(this.props.base, 8)
    }), /* @__PURE__ */ _("button", {
      type: "button",
      onClick: this.onJumpForward
    }, ">")), /* @__PURE__ */ _("code", null, rows));
  }
};

// debugger/break.tsx
var Breakpoints = class {
  constructor(storageKey) {
    this.storageKey = storageKey;
    const json = window.localStorage.getItem(storageKey);
    if (!json)
      return;
    const list = JSON.parse(json);
    this.breakpoints = new Map(list.map((bp) => [bp.addr, bp]));
  }
  breakpoints = /* @__PURE__ */ new Map();
  save() {
    window.localStorage.setItem(this.storageKey, JSON.stringify(Array.from(this.breakpoints.values())));
  }
  addBreak(bp) {
    this.breakpoints.set(bp.addr, bp);
    this.save();
  }
  addBreakByName(labels, name) {
    for (const [addr, label] of labels.byAddr) {
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
    this.save();
  }
  toggleBreak(addr) {
    const bp = this.breakpoints.get(addr);
    bp.disabled = !bp.disabled;
    this.save();
  }
  isAtBreakpoint(ip) {
    const bp = this.breakpoints.get(ip);
    if (bp && !bp.disabled) {
      if (bp.oneShot) {
        this.delBreak(bp.addr);
      }
      return bp;
    }
    return void 0;
  }
  install(emu) {
    for (const bp of this.breakpoints.values()) {
      if (!bp.disabled) {
        emu.breakpoint_add(bp.addr);
      }
    }
  }
  uninstall(emu) {
    for (const bp of this.breakpoints.values()) {
      if (!bp.disabled) {
        emu.breakpoint_clear(bp.addr);
      }
    }
  }
};
var BreakpointsComponent = class extends k {
  toggle(addr) {
    this.props.breakpoints.toggleBreak(addr);
    this.forceUpdate();
  }
  remove(addr) {
    this.props.breakpoints.delBreak(addr);
    this.forceUpdate();
  }
  add(text) {
    const ret = this.props.breakpoints.addBreakByName(this.props.labels, text);
    this.forceUpdate();
    return ret;
  }
  render() {
    const rows = [];
    for (const bp of this.props.breakpoints.breakpoints.values()) {
      const className = bp.addr === this.props.highlight ? "highlight" : void 0;
      const label = this.props.labels.get(bp.addr);
      rows.push(
        /* @__PURE__ */ _("div", {
          className,
          style: { display: "flex", alignItems: "center", gap: "0.5ex" }
        }, /* @__PURE__ */ _("input", {
          type: "checkbox",
          checked: !bp.disabled,
          onChange: () => this.toggle(bp.addr)
        }), /* @__PURE__ */ _("div", null, /* @__PURE__ */ _("code", null, /* @__PURE__ */ _(Number, {
          digits: 8,
          ...this.props
        }, bp.addr))), bp.oneShot ? "[once]" : null, label ? /* @__PURE__ */ _("div", null, "(", /* @__PURE__ */ _("code", null, label), ")") : null, /* @__PURE__ */ _("button", {
          class: "x",
          onClick: () => this.remove(bp.addr)
        }, "x"))
      );
    }
    return /* @__PURE__ */ _("section", null, rows, /* @__PURE__ */ _(AddComponent, {
      onAccept: (text) => this.add(text)
    }));
  }
};
var AddComponent = class extends k {
  onInput = (ev) => {
    const text = ev.target.value;
    this.setState({ text });
  };
  onSubmit = (ev) => {
    ev.preventDefault();
    if (this.props.onAccept(this.state.text)) {
      this.setState({ text: "" });
    }
  };
  render() {
    return /* @__PURE__ */ _("form", {
      onSubmit: this.onSubmit
    }, "add: ", /* @__PURE__ */ _("input", {
      value: this.state.text,
      onInput: this.onInput
    }));
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
function isLikeNone(x2) {
  return x2 === void 0 || x2 === null;
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
function getArrayU8FromWasm0(ptr, len) {
  ptr = ptr >>> 0;
  return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}
function addToExternrefTable0(obj) {
  const idx = wasm.__externref_table_alloc();
  wasm.__wbindgen_export_2.set(idx, obj);
  return idx;
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
function new_emulator(host, cmdline) {
  const ptr0 = passStringToWasm0(cmdline, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
  const len0 = WASM_VECTOR_LEN;
  const ret = wasm.new_emulator(host, ptr0, len0);
  return Emulator.__wrap(ret);
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
var Status = Object.freeze({ Running: 0, "0": "Running", Blocked: 1, "1": "Blocked", Error: 2, "2": "Error", DebugBreak: 3, "3": "DebugBreak", Exit: 4, "4": "Exit" });
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
  get exit_code() {
    const ret = wasm.emulator_exit_code(this.__wbg_ptr);
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
  set_tracing_scheme(scheme) {
    const ptr0 = passStringToWasm0(scheme, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.emulator_set_tracing_scheme(this.__wbg_ptr, ptr0, len0);
  }
};
var FileOptionsFinalization = typeof FinalizationRegistry === "undefined" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((ptr) => wasm.__wbg_fileoptions_free(ptr >>> 0));
var FileOptions = class {
  static __wrap(ptr) {
    ptr = ptr >>> 0;
    const obj = Object.create(FileOptions.prototype);
    obj.__wbg_ptr = ptr;
    FileOptionsFinalization.register(obj, obj.__wbg_ptr, obj);
    return obj;
  }
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    FileOptionsFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_fileoptions_free(ptr);
  }
  get read() {
    const ret = wasm.__wbg_get_fileoptions_read(this.__wbg_ptr);
    return ret !== 0;
  }
  set read(arg0) {
    wasm.__wbg_set_fileoptions_read(this.__wbg_ptr, arg0);
  }
  get write() {
    const ret = wasm.__wbg_get_fileoptions_write(this.__wbg_ptr);
    return ret !== 0;
  }
  set write(arg0) {
    wasm.__wbg_set_fileoptions_write(this.__wbg_ptr, arg0);
  }
  get truncate() {
    const ret = wasm.__wbg_get_fileoptions_truncate(this.__wbg_ptr);
    return ret !== 0;
  }
  set truncate(arg0) {
    wasm.__wbg_set_fileoptions_truncate(this.__wbg_ptr, arg0);
  }
  get create() {
    const ret = wasm.__wbg_get_fileoptions_create(this.__wbg_ptr);
    return ret !== 0;
  }
  set create(arg0) {
    wasm.__wbg_set_fileoptions_create(this.__wbg_ptr, arg0);
  }
  get create_new() {
    const ret = wasm.__wbg_get_fileoptions_create_new(this.__wbg_ptr);
    return ret !== 0;
  }
  set create_new(arg0) {
    wasm.__wbg_set_fileoptions_create_new(this.__wbg_ptr, arg0);
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
  imports.wbg.__wbg_read_ca96830ec9aacdcf = function(arg0, arg1, arg2) {
    const ret = arg0.read(getArrayU8FromWasm0(arg1, arg2));
    return ret;
  };
  imports.wbg.__wbg_seek_c5471dc2ba4d64bc = function(arg0, arg1) {
    const ret = arg0.seek(arg1 >>> 0);
    return ret;
  };
  imports.wbg.__wbg_write_3b537d4aeda328c0 = function(arg0, arg1, arg2) {
    const ret = arg0.write(getArrayU8FromWasm0(arg1, arg2));
    return ret;
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
  imports.wbg.__wbg_settitle_b48ee927b9814f5c = function(arg0, arg1, arg2) {
    arg0.title = getStringFromWasm0(arg1, arg2);
  };
  imports.wbg.__wbg_setsize_7bcb3132fd38238f = function(arg0, arg1, arg2) {
    arg0.set_size(arg1 >>> 0, arg2 >>> 0);
  };
  imports.wbg.__wbg_info_2551d10805917111 = function(arg0) {
    const ret = arg0.info();
    return ret;
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
  imports.wbg.__wbg_new0_7d84e5b2cd9fdc73 = function() {
    const ret = new Date();
    return ret;
  };
  imports.wbg.__wbg_getTime_2bc4375165f02d15 = function(arg0) {
    const ret = arg0.getTime();
    return ret;
  };
  imports.wbg.__wbindgen_number_new = function(arg0) {
    const ret = arg0;
    return ret;
  };
  imports.wbg.__wbg_new_cf3ec55744a78578 = function(arg0) {
    const ret = new Date(arg0);
    return ret;
  };
  imports.wbg.__wbg_getTimezoneOffset_38257122e236c190 = function(arg0) {
    const ret = arg0.getTimezoneOffset();
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
  imports.wbg.__wbg_timeStamp_33be24162c74bf21 = function(arg0) {
    const ret = arg0.timeStamp;
    return ret;
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
  imports.wbg.__wbg_open_61490d64358619c7 = function(arg0, arg1, arg2, arg3) {
    const ret = arg0.open(getStringFromWasm0(arg1, arg2), FileOptions.__wrap(arg3));
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
  };
  imports.wbg.__wbg_stdout_e61c589931436d69 = function(arg0, arg1, arg2) {
    arg0.stdout(getArrayU8FromWasm0(arg1, arg2));
  };
  imports.wbg.__wbg_createwindow_79bbfe483866ee8c = function(arg0, arg1) {
    const ret = arg0.create_window(arg1 >>> 0);
    return ret;
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
  imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
    const ret = debugString(arg1);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
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
    const stashEvent = (ev) => {
      ev.hwnd = hwnd;
      jsHost.enqueueEvent(ev);
      return false;
    };
    this.canvas.onmousedown = stashEvent;
    this.canvas.onmouseup = stashEvent;
    this.canvas.onmousemove = stashEvent;
    this.canvas.oncontextmenu = (ev) => {
      return false;
    };
  }
  title = "";
  canvas = document.createElement("canvas");
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
  }
  ofs = 0;
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
  write(buf) {
    console.warn("ignoring write");
    return buf.length;
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
  }
  events = [];
  timer;
  decoder = new TextDecoder();
  log(level, msg) {
    switch (level) {
      case 1:
        console.error(msg);
        if (this.emuHost)
          this.emuHost.onError(msg);
        break;
      case 2:
        console.warn(msg);
        break;
      case 3:
        console.info(msg);
        break;
      case 4:
      case 5:
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
      return null;
    }
    return new File(path, bytes);
  }
  stdout(buf) {
    const text = this.decoder.decode(buf);
    this.emuHost.onStdOut(text);
  }
  windows = [];
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

// emulator.ts
var Emulator2 = class extends JsHost {
  emu;
  imports = [];
  labels;
  running = false;
  breakpoints;
  channel = new MessageChannel();
  constructor(host, files, exePath, cmdLine, bytes, labels, relocate) {
    super(host, files);
    this.emu = new_emulator(this, cmdLine);
    this.emu.load_exe(exePath, bytes, relocate);
    this.breakpoints = new Breakpoints(exePath);
    const importsJSON = JSON.parse(this.emu.labels());
    for (const [jsAddr, jsName] of Object.entries(importsJSON)) {
      const addr = parseInt(jsAddr);
      const name = jsName;
      this.imports.push(`${hex(addr, 8)}: ${name}`);
      labels.set(addr, name);
    }
    this.labels = new Labels(labels);
    this.channel.port2.onmessage = () => this.loop();
  }
  step() {
    this.emu.unblock();
    this.emu.run(1);
  }
  stepSize = 5e3;
  instrPerMs = 0;
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
        this.stepSize = this.instrPerMs * 8;
      }
    }
    return cpuState;
  }
  stepMany() {
    this.breakpoints.install(this.emu);
    const cpuState = this.runBatch();
    this.breakpoints.uninstall(this.emu);
    switch (cpuState) {
      case Status.Running:
        return true;
      case Status.DebugBreak: {
        const bp = this.breakpoints.isAtBreakpoint(this.emu.eip);
        if (bp) {
          if (!bp.oneShot) {
            this.emuHost.showTab("breakpoints");
          }
          this.emuHost.onStopped();
        }
        return false;
      }
      case Status.Blocked:
      case Status.Error:
        this.emuHost.onStopped();
        return false;
      case Status.Exit:
        this.emuHost.onStopped();
        this.emuHost.exit(this.emu.exit_code);
        return false;
    }
  }
  start() {
    if (this.running)
      return;
    this.emu.unblock();
    if (this.breakpoints.isAtBreakpoint(this.emu.eip)) {
      this.step();
    }
    this.running = true;
    this.channel.port1.postMessage(null);
  }
  loop() {
    if (!this.running)
      return;
    if (!this.stepMany()) {
      this.stop();
      return;
    }
    this.channel.port1.postMessage(null);
  }
  stop() {
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
var WindowComponent = class extends k {
  state = {
    pos: [200, 200]
  };
  ref = m();
  beginDrag = (e2) => {
    const node = e2.currentTarget;
    this.setState({ drag: [e2.offsetX, e2.offsetY] });
    node.setPointerCapture(e2.pointerId);
    e2.preventDefault();
  };
  onDrag = (e2) => {
    if (!this.state.drag)
      return;
    this.setState({ pos: [e2.clientX - this.state.drag[0], e2.clientY - this.state.drag[1]] });
    e2.preventDefault();
  };
  endDrag = (e2) => {
    const node = e2.currentTarget;
    this.setState({ drag: void 0 });
    node.releasePointerCapture(e2.pointerId);
    e2.preventDefault();
  };
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
    return /* @__PURE__ */ _("div", {
      class: "window",
      style: { left: `${this.state.pos[0]}px`, top: `${this.state.pos[1]}px` }
    }, /* @__PURE__ */ _("div", {
      class: "titlebar",
      onPointerDown: this.beginDrag,
      onPointerUp: this.endDrag,
      onPointerMove: this.onDrag
    }, this.props.title), /* @__PURE__ */ _("div", {
      ref: this.ref
    }));
  }
};
var EmulatorComponent = class extends k {
  render() {
    return this.props.emulator.windows.map((window2) => {
      return /* @__PURE__ */ _(WindowComponent, {
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
  const cmdLine = query.get("cmdline") || void 0;
  const params = { dir, exe, files, relocate, cmdLine };
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
  const resp = await fetch(params.dir + params.exe + ".csv");
  if (resp.ok) {
    for (const [addr, name] of parseCSV(await resp.text())) {
      csvLabels.set(addr, name);
    }
  }
  const cmdLine = params.cmdLine ?? params.exe;
  const exePath = (params.dir ?? "") + params.exe;
  return new Emulator2(
    null,
    fileset,
    exePath,
    cmdLine,
    fileset.get(params.exe),
    csvLabels,
    params.relocate ?? false
  );
}

// debugger/code.tsx
var Code = class extends k {
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
            return /* @__PURE__ */ _(b, null, /* @__PURE__ */ _(Number, {
              text,
              ...this.props
            }, addr), label);
          }
          default:
            return text;
        }
      });
      return /* @__PURE__ */ _("tr", null, /* @__PURE__ */ _("td", {
        style: { width: "10ch" }
      }, /* @__PURE__ */ _("span", {
        class: "clicky",
        title: "run to this address",
        onClick: (event) => {
          this.props.runTo(instr.addr);
        }
      }, hex(instr.addr, 8))), /* @__PURE__ */ _("td", {
        title: `${instr.bytes} (${instr.ops.join(",")})`
      }, code));
    });
    return /* @__PURE__ */ _("section", {
      class: "code"
    }, /* @__PURE__ */ _("code", {
      class: "disassembly"
    }, /* @__PURE__ */ _("table", null, instrs)));
  }
};

// debugger/mappings.tsx
var Mappings = class extends k {
  render() {
    const rows = this.props.mappings.map((mapping) => {
      let className;
      const highlight = this.props.highlight;
      if (highlight !== void 0 && highlight >= mapping.addr && highlight < mapping.addr + mapping.size) {
        className = "highlight";
      }
      return /* @__PURE__ */ _("tr", {
        class: className
      }, /* @__PURE__ */ _("td", {
        style: { width: "10ch" }
      }, hex(mapping.addr, 8)), /* @__PURE__ */ _("td", {
        style: { width: "8ch" }
      }, hex(mapping.size)), /* @__PURE__ */ _("td", null, mapping.desc));
    });
    return /* @__PURE__ */ _("section", {
      style: { overflow: "scroll", flex: 1 }
    }, /* @__PURE__ */ _("code", null, /* @__PURE__ */ _("table", null, /* @__PURE__ */ _("thead", null, /* @__PURE__ */ _("tr", null, /* @__PURE__ */ _("td", null, "addr"), /* @__PURE__ */ _("td", null, "size"), /* @__PURE__ */ _("td", null, "desc"))), rows)));
  }
};

// debugger/registers.tsx
var RegistersComponent = class extends k {
  render() {
    const { regs } = this.props;
    const st = regs.st;
    return /* @__PURE__ */ _("section", {
      class: "panel"
    }, /* @__PURE__ */ _("code", null, /* @__PURE__ */ _("div", null, "eax\xA0", /* @__PURE__ */ _(Number, {
      digits: 8,
      ...this.props
    }, regs.eax), /* @__PURE__ */ _("br", null), "ebx\xA0", /* @__PURE__ */ _(Number, {
      digits: 8,
      ...this.props
    }, regs.ebx), /* @__PURE__ */ _("br", null), "ecx\xA0", /* @__PURE__ */ _(Number, {
      digits: 8,
      ...this.props
    }, regs.ecx), /* @__PURE__ */ _("br", null), "edx\xA0", /* @__PURE__ */ _(Number, {
      digits: 8,
      ...this.props
    }, regs.edx), /* @__PURE__ */ _("br", null)), /* @__PURE__ */ _("br", null), /* @__PURE__ */ _("div", null, "eip\xA0", /* @__PURE__ */ _(Number, {
      digits: 8,
      ...this.props
    }, regs.eip), /* @__PURE__ */ _("br", null), "esp\xA0", /* @__PURE__ */ _(Number, {
      digits: 8,
      ...this.props
    }, regs.esp), /* @__PURE__ */ _("br", null), "ebp\xA0", /* @__PURE__ */ _(Number, {
      digits: 8,
      ...this.props
    }, regs.ebp), /* @__PURE__ */ _("br", null), "esi\xA0", /* @__PURE__ */ _(Number, {
      digits: 8,
      ...this.props
    }, regs.esi), /* @__PURE__ */ _("br", null), "edi\xA0", /* @__PURE__ */ _(Number, {
      digits: 8,
      ...this.props
    }, regs.edi), /* @__PURE__ */ _("br", null)), /* @__PURE__ */ _("br", null), /* @__PURE__ */ _("div", null, "cs\xA0", /* @__PURE__ */ _(Number, {
      digits: 4,
      ...this.props
    }, regs.cs), " ", "fs\xA0", /* @__PURE__ */ _(Number, {
      digits: 4,
      ...this.props
    }, regs.fs), /* @__PURE__ */ _("br", null), "ds\xA0", /* @__PURE__ */ _(Number, {
      digits: 4,
      ...this.props
    }, regs.ds), " ", "gs\xA0", /* @__PURE__ */ _(Number, {
      digits: 4,
      ...this.props
    }, regs.gs), /* @__PURE__ */ _("br", null), "es\xA0", /* @__PURE__ */ _(Number, {
      digits: 4,
      ...this.props
    }, regs.es), " ", "ss\xA0", /* @__PURE__ */ _(Number, {
      digits: 4,
      ...this.props
    }, regs.ss), /* @__PURE__ */ _("br", null)), /* @__PURE__ */ _("br", null), /* @__PURE__ */ _("div", null, "flags\xA0", hex(regs.flags), " ", regs.flags_str), /* @__PURE__ */ _("br", null), st.length > 0 ? /* @__PURE__ */ _("div", null, "fpu", /* @__PURE__ */ _("br", null), Array.from(regs.st).map((n2) => /* @__PURE__ */ _("span", null, n2, /* @__PURE__ */ _("br", null)))) : null));
  }
};

// debugger/stack.tsx
var Stack = class extends k {
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
      let row = /* @__PURE__ */ _("div", null, /* @__PURE__ */ _(Number, {
        digits: 8,
        ...this.props
      }, addr), "\xA0", /* @__PURE__ */ _(Number, {
        digits: 8,
        ...this.props
      }, value), label);
      if (addr === esp) {
        row = /* @__PURE__ */ _("b", null, row);
      }
      rows.push(row);
    }
    return /* @__PURE__ */ _("section", {
      class: "panel"
    }, /* @__PURE__ */ _("code", null, rows));
  }
};

// debugger/tabs.tsx
var Tabs = class extends k {
  render() {
    const { style, tabs, selected, switchTab } = this.props;
    const content = tabs[selected]();
    return /* @__PURE__ */ _("section", {
      class: "panel",
      style
    }, /* @__PURE__ */ _("div", {
      class: "tabs-strip"
    }, "|", Object.keys(tabs).map((name) => {
      let button = /* @__PURE__ */ _("span", {
        class: "clicky",
        onClick: () => switchTab(name)
      }, name);
      if (name === selected) {
        button = /* @__PURE__ */ _("b", null, button);
      }
      return /* @__PURE__ */ _(b, null, "\xA0", button, "\xA0|");
    })), content);
  }
};

// debugger/debugger.tsx
var StartStop = class extends k {
  render() {
    const { running, start, stop, step, stepOver } = this.props;
    return /* @__PURE__ */ _("span", null, /* @__PURE__ */ _("button", {
      onClick: () => running ? stop() : start()
    }, running ? "stop" : "run"), "\xA0", /* @__PURE__ */ _("button", {
      onClick: () => step()
    }, "step"), "\xA0", /* @__PURE__ */ _("button", {
      onClick: () => stepOver()
    }, "step over"));
  }
};
var Debugger = class extends k {
  state = { error: "", memBase: 4198400, selectedTab: "output" };
  constructor(props) {
    super(props);
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
  onStopped() {
    this.stop();
  }
  step = () => {
    try {
      this.props.emulator.step();
    } finally {
      this.forceUpdate();
    }
  };
  start = () => {
    if (this.state.running)
      return;
    this.setState({
      running: setInterval(() => {
        this.forceUpdate();
      }, 500)
    });
    this.props.emulator.start();
  };
  stop = () => {
    if (!this.state.running)
      return;
    this.props.emulator.stop();
    clearInterval(this.state.running);
    this.setState({ running: void 0 });
  };
  runTo = (addr) => {
    this.props.emulator.breakpoints.addBreak({ addr, oneShot: true });
    this.start();
  };
  memoryView = {
    highlightMemory: (addr) => this.setState({ memHighlight: addr }),
    showMemory: (memBase) => this.setState({ selectedTab: "memory", memBase })
  };
  render() {
    let instrs = [];
    let code;
    const eip = this.props.emulator.emu.eip;
    if (eip >= 4054253568) {
      const label = eip == 4294967280 ? "async" : this.props.emulator.labels.get(eip) ?? "shim";
      code = /* @__PURE__ */ _("section", {
        class: "code"
      }, "(in ", label, ")");
    } else {
      instrs = this.props.emulator.disassemble(eip);
      code = /* @__PURE__ */ _(Code, {
        instrs,
        labels: this.props.emulator.labels,
        ...this.memoryView,
        runTo: this.runTo
      });
    }
    return /* @__PURE__ */ _(b, null, /* @__PURE__ */ _(EmulatorComponent, {
      emulator: this.props.emulator
    }), /* @__PURE__ */ _("section", {
      class: "panel",
      style: { display: "flex", alignItems: "baseline" }
    }, /* @__PURE__ */ _(StartStop, {
      running: this.state.running != null,
      start: this.start,
      stop: this.stop,
      step: this.step,
      stepOver: () => instrs ? this.runTo(instrs[1].addr) : this.step()
    }), "\xA0", /* @__PURE__ */ _("div", null, this.props.emulator.emu.instr_count, " instrs executed | ", Math.floor(this.props.emulator.instrPerMs), "/ms")), /* @__PURE__ */ _("div", {
      style: { display: "flex", gap: "8px" }
    }, code, /* @__PURE__ */ _(RegistersComponent, {
      ...this.memoryView,
      regs: this.props.emulator.emu.regs()
    }), /* @__PURE__ */ _(Stack, {
      ...this.memoryView,
      labels: this.props.emulator.labels,
      emu: this.props.emulator.emu
    })), /* @__PURE__ */ _(Tabs, {
      style: { width: "80ex", flex: 1, minHeight: 0, overflow: "hidden", display: "flex", flexDirection: "column" },
      tabs: {
        output: () => /* @__PURE__ */ _("div", null, /* @__PURE__ */ _("code", null, this.state.stdout, this.state.error ? /* @__PURE__ */ _("div", {
          class: "error"
        }, "ERROR: ", this.state.error) : null)),
        memory: () => /* @__PURE__ */ _(Memory, {
          mem: this.props.emulator.emu.memory(),
          base: this.state.memBase,
          highlight: this.state.memHighlight,
          jumpTo: (addr) => this.setState({ memBase: addr })
        }),
        mappings: () => /* @__PURE__ */ _(Mappings, {
          mappings: this.props.emulator.mappings(),
          highlight: this.state.memHighlight
        }),
        imports: () => /* @__PURE__ */ _("div", null, /* @__PURE__ */ _("code", null, this.props.emulator.imports.map((imp) => /* @__PURE__ */ _("div", null, imp)))),
        breakpoints: () => /* @__PURE__ */ _(BreakpointsComponent, {
          breakpoints: this.props.emulator.breakpoints,
          labels: this.props.emulator.labels,
          highlight: eip,
          ...this.memoryView
        })
      },
      selected: this.state.selectedTab,
      switchTab: (selectedTab) => this.setState({ selectedTab })
    }));
  }
};
async function main() {
  const emulator = await loadEmulator();
  emulator.emu.set_tracing_scheme("*");
  B(/* @__PURE__ */ _(Debugger, {
    emulator
  }), document.body);
}

// run.tsx
var Panel = class extends k {
  debugger() {
    window.location.pathname = window.location.pathname.replace("/run.html", "/debugger.html");
  }
  updateStatus = () => {
    if (!this.props.emulator)
      return;
    this.setState({
      status: {
        instrCount: this.props.emulator.emu.instr_count,
        instrPerMs: Math.floor(this.props.emulator.instrPerMs)
      }
    });
  };
  interval;
  componentDidUpdate() {
    if (this.props.emulator && !this.interval) {
      this.updateStatus();
      this.interval = setInterval(this.updateStatus, 500);
    } else if (!this.props.emulator && this.interval) {
      clearInterval(this.interval);
      this.interval = void 0;
    }
  }
  render() {
    let status;
    if (this.state.status) {
      status = /* @__PURE__ */ _("div", null, this.state.status.instrCount, " instrs executed, ", Math.floor(this.state.status.instrPerMs), "/ms");
    }
    return /* @__PURE__ */ _("header", {
      class: "panel"
    }, /* @__PURE__ */ _("a", {
      style: "font-weight: bold; color: inherit",
      href: "https://evmar.github.io/retrowin32/"
    }, "retrowin32"), ": a windows emulator", /* @__PURE__ */ _("div", {
      style: "width: 2ex"
    }), /* @__PURE__ */ _("button", {
      onClick: this.debugger
    }, "view in debugger"), /* @__PURE__ */ _("div", {
      style: { flex: "1" }
    }), status);
  }
};
var Page = class extends k {
  async load() {
    const emulator = await loadEmulator();
    emulator.emu.set_tracing_scheme("-");
    const host = {
      exit: (code) => {
        this.print(`exited with code ${code}
`);
      },
      onWindowChanged: () => {
        this.forceUpdate();
      },
      showTab: (name) => {
      },
      onError: (msg) => {
        this.print(msg + "\n");
        this.setState({ emulator: void 0 });
      },
      onStdOut: (stdout) => {
        this.print(stdout);
      },
      onStopped: () => {
      }
    };
    emulator.emuHost = host;
    this.setState({ emulator });
    emulator.start();
  }
  print = (text) => {
    this.setState((state) => ({ output: (state.output ?? "") + text }));
  };
  componentDidMount() {
    this.load().catch((e2) => this.print(e2.stack ?? e2.toString()));
  }
  render() {
    return /* @__PURE__ */ _(b, null, /* @__PURE__ */ _(Panel, {
      emulator: this.state.emulator
    }), /* @__PURE__ */ _("main", null, this.state.output ? /* @__PURE__ */ _("pre", {
      class: "stdout"
    }, this.state.output) : null, this.state.emulator ? /* @__PURE__ */ _(EmulatorComponent, {
      emulator: this.state.emulator
    }) : null));
  }
};
function main2() {
  B(/* @__PURE__ */ _(Page, null), document.body);
}
export {
  main as debuggerMain,
  main2 as runMain
};
//# sourceMappingURL=bundle.js.map
