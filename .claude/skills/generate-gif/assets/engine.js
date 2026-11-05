// GIF engine: builds the shared chrome (header, code window, legend) around a
// scene's #stage and exposes a deterministic API (window.GIF) that
// scripts/render.mjs drives frame by frame.
//
// A scene defines, before this file is injected:
//   window.SCENE = {
//     title, subtitle, brand?, file?, code, legend?, narration?,
//     steps: [{ line, say?, ms?, ...state }],
//     stepMs?, finalHoldMs?, fps?, height?,
//     setup?(stage),           // build dynamic DOM once
//     render(step, i, steps),  // apply a step's state to the DOM (idempotent)
//   };
(() => {
  const S = window.SCENE;
  if (!S) throw new Error('window.SCENE is not defined');
  if (!Array.isArray(S.steps) || S.steps.length === 0) throw new Error('SCENE.steps is empty');

  const root = document.documentElement;

  // ---------- Rust syntax highlighting ----------
  const KW = new Set(('as async await break const continue crate dyn else enum extern false fn for if impl in let loop ' +
    'match mod move mut pub ref return self Self static struct super trait true type unsafe use where while').split(' '));
  const PRIM = new Set('i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize f32 f64 bool char str'.split(' '));
  const TOKEN = /(\/\/.*)|("(?:\\.|[^"\\])*")|('(?:\\.|[^'\\])')|('[a-z_]\w*)|(\b\d[\d_]*(?:\.\d+)?(?:[iu](?:8|16|32|64|128|size)|f32|f64)?\b)|(\b[a-z_]\w*!)|(\b[A-Za-z_]\w*\b)/g;
  const esc = s => s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
  const span = (cls, s) => `<span class="${cls}">${esc(s)}</span>`;

  function highlight(line) {
    let out = '', last = 0, m;
    TOKEN.lastIndex = 0;
    while ((m = TOKEN.exec(line))) {
      out += esc(line.slice(last, m.index));
      const [t, com, str, chr, life, num, mac, word] = m;
      if (com) out += span('tok-com', t);
      else if (str || chr) out += span('tok-str', t);
      else if (life) out += span('tok-life', t);
      else if (num) out += span('tok-num', t);
      else if (mac) out += span('tok-mac', t);
      else if (word && KW.has(t)) out += span('tok-kw', t);
      else if (word && (PRIM.has(t) || /^[A-Z]/.test(t))) out += span('tok-ty', t);
      else if (word && line[TOKEN.lastIndex] === '(') out += span('tok-fn', t);
      else out += esc(t);
      last = TOKEN.lastIndex;
    }
    return out + esc(line.slice(last));
  }

  // ---------- layout ----------
  const el = (tag, cls, html) => {
    const e = document.createElement(tag);
    if (cls) e.className = cls;
    if (html != null) e.innerHTML = html;
    return e;
  };

  const app = el('div');
  app.id = 'app';

  const header = el('header', 'gif-header');
  header.append(el('h1', '', esc(S.title || 'Untitled')));
  if (S.subtitle) header.append(el('p', '', esc(S.subtitle)));
  if (S.brand) {
    const b = el('div', 'brand');
    b.append(el('span', '', esc(S.brand)));
    header.append(b);
  }
  app.append(header);

  const stage = document.getElementById('stage') || el('div');
  stage.id = 'stage';
  app.append(stage);

  const codeLines = String(S.code || '').replace(/^\n+|\s+$/g, '').split('\n');
  const win = el('div', 'code-window');
  const bar = el('div', 'bar', '<i class="dot"></i><i class="dot"></i><i class="dot"></i>');
  bar.append(el('span', 'file', esc(S.file || 'main.rs')));
  // Narration lives in the title bar so it costs no vertical space.
  let narration = null;
  if (S.narration !== false && S.steps.some(s => s.say)) {
    narration = el('span', 'say');
    bar.append(narration);
  }
  const body = el('div', 'body');
  const hl = el('div', 'hl');
  const pre = el('pre', '', codeLines.map(highlight).join('\n'));
  body.append(hl, pre);
  win.append(bar, body);
  app.append(win);

  if (Array.isArray(S.legend) && S.legend.length) {
    const legend = el('div', 'legend');
    legend.style.setProperty('--cols', String(Math.min(S.legend.length, 3)));
    S.legend.forEach((c, i) => {
      const card = el('div', 'card');
      card.append(el('h3', '', `<b>${i + 1}</b>${esc(c.title)}`), el('p', '', esc(c.text)));
      legend.append(card);
    });
    app.append(legend);
  }

  document.body.prepend(app);
  if (typeof S.setup === 'function') S.setup(stage);

  // ---------- step rendering ----------
  const lh = () => parseFloat(getComputedStyle(root).getPropertyValue('--code-lh')) || 40;

  function show(i) {
    const step = S.steps[i];
    if (step.line) {
      hl.style.opacity = '1';
      hl.style.transform = `translateY(${(step.line - 1) * lh()}px)`;
    } else {
      hl.style.opacity = '0';
    }
    if (narration) narration.textContent = step.say || '';
    S.render(step, i, S.steps);
  }

  function instant(fn) {
    root.classList.add('no-anim');
    fn();
    void document.body.offsetHeight; // flush styles without transitions
    root.classList.remove('no-anim');
    void document.body.offsetHeight;
  }

  // ---------- deterministic animation control ----------
  let current = [];  // finite animations started by the current step
  let clock = 0;     // ms since the first frame, for infinite animations

  const isInfinite = a => {
    try { return a.effect.getComputedTiming().endTime === Infinity; } catch { return false; }
  };

  window.GIF = {
    width: 1080,
    fps: S.fps || 20,
    steps: S.steps.length,

    holdMs(i) {
      const s = S.steps[i];
      const base = s.ms ?? S.stepMs ?? 650;
      return i === S.steps.length - 1 ? base + (S.finalHoldMs ?? 2200) : base;
    },

    // Natural content height across all steps, so nothing ever overflows.
    measure() {
      app.classList.add('measuring');
      let max = 0;
      instant(() => {
        for (let i = 0; i < S.steps.length; i++) {
          show(i);
          max = Math.max(max, Math.ceil(app.getBoundingClientRect().height));
        }
      });
      app.classList.remove('measuring');
      return max;
    },

    setHeight(px) {
      root.style.setProperty('--h', `${px}px`);
    },

    // Jump to a step with no transition (used for stills and the first frame).
    jump(i) {
      instant(() => show(i));
    },

    // Start step i: settle the previous step, apply new state, pause the new
    // transitions so seek() can scrub them.
    begin(i) {
      for (const a of document.getAnimations()) {
        if (!isInfinite(a)) try { a.finish(); } catch { /* ignore */ }
      }
      show(i);
      getComputedStyle(document.body).opacity; // create the CSS transitions now
      current = document.getAnimations().filter(a => !isInfinite(a));
      for (const a of document.getAnimations()) a.pause();
    },

    seek(t) {
      for (const a of current) a.currentTime = t;
      for (const a of document.getAnimations()) if (isInfinite(a)) a.currentTime = clock + t;
    },

    end(i) {
      clock += this.holdMs(i);
    },
  };
})();
