#!/usr/bin/env node
// Render a scene HTML file into a LinkedIn-ready GIF (and optionally MP4 / stills).
//
// Usage:
//   node render.mjs <scene.html> --out <file.gif> [--fps 20] [--mp4]
//   node render.mjs <scene.html> --still 0,12,last --stills-dir <dir>
//
// The scene defines window.SCENE (see assets/engine.js); this script injects
// assets/theme.css + assets/engine.js, sizes the canvas, scrubs every CSS
// transition deterministically, screenshots each frame, and encodes with ffmpeg.
import { chromium } from 'playwright';
import { execFileSync } from 'node:child_process';
import { mkdtempSync, mkdirSync, rmSync, statSync, existsSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const SKILL_DIR = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const WIDTH = 1080;
const MIN_H = 1350;   // 4:5 — LinkedIn's recommended portrait size
const MAX_H = 1920;   // 9:16 — tallest that still reads well on a phone
const SOFT_LIMIT_MB = 5;

function parseArgs(argv) {
  const opts = { scene: null, out: null, fps: null, mp4: false, still: null, stillsDir: null, keepFrames: false };
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    if (a === '--out') opts.out = argv[++i];
    else if (a === '--fps') opts.fps = Number(argv[++i]);
    else if (a === '--mp4') opts.mp4 = true;
    else if (a === '--still') opts.still = argv[++i];
    else if (a === '--stills-dir') opts.stillsDir = argv[++i];
    else if (a === '--keep-frames') opts.keepFrames = true;
    else if (!opts.scene) opts.scene = a;
    else throw new Error(`unknown argument: ${a}`);
  }
  if (!opts.scene) throw new Error('missing <scene.html>');
  if (!opts.out && !opts.still) throw new Error('pass --out <file.gif> or --still <steps>');
  return opts;
}

async function main() {
  const opts = parseArgs(process.argv.slice(2));
  const scene = resolve(opts.scene);
  if (!existsSync(scene)) throw new Error(`scene not found: ${scene}`);

  const browser = await chromium.launch();
  const page = await browser.newPage({ viewport: { width: WIDTH, height: MIN_H }, deviceScaleFactor: 1 });
  const errors = [];
  page.on('pageerror', e => errors.push(e.message));
  page.on('console', m => { if (m.type() === 'error') errors.push(m.text()); });

  await page.goto(pathToFileURL(scene).href);
  await page.addStyleTag({ path: join(SKILL_DIR, 'assets/theme.css') });
  await page.addScriptTag({ path: join(SKILL_DIR, 'assets/engine.js') });
  await page.evaluate(() => Promise.race([document.fonts.ready, new Promise(r => setTimeout(r, 4000))]));
  const ok = await page.evaluate(() => typeof window.GIF === 'object');
  if (!ok) throw new Error(`scene failed to load:\n${errors.join('\n')}`);

  // Size the canvas to the tallest step, within LinkedIn-friendly bounds.
  const natural = await page.evaluate(() => GIF.measure());
  const height = Math.max(MIN_H, Math.min(MAX_H, natural));
  if (natural > MAX_H) {
    console.warn(`WARNING: content is ${natural}px tall (> ${MAX_H}). It will be cut off — shorten the code or shrink the stage.`);
  }
  await page.evaluate(h => GIF.setHeight(h), height);
  await page.setViewportSize({ width: WIDTH, height });

  const info = await page.evaluate(() => ({ steps: GIF.steps, fps: GIF.fps }));
  console.log(`canvas ${WIDTH}x${height} (content ${natural}px) · ${info.steps} steps`);
  const fps = opts.fps || info.fps;

  if (opts.still) {
    const dir = resolve(opts.stillsDir || join(dirname(scene), 'stills'));
    mkdirSync(dir, { recursive: true });
    const wanted = opts.still.split(',').map(s => (s.trim() === 'last' ? info.steps - 1 : Number(s)));
    for (const i of wanted) {
      if (!(i >= 0 && i < info.steps)) throw new Error(`step ${i} out of range (0..${info.steps - 1})`);
      await page.evaluate(n => GIF.jump(n), i);
      const file = join(dir, `step-${String(i).padStart(3, '0')}.png`);
      await page.screenshot({ path: file });
      console.log(file);
    }
  }

  if (opts.out) {
    const frames = mkdtempSync(join(tmpdir(), 'gif-frames-'));
    let n = 0;
    await page.evaluate(() => GIF.jump(0));
    for (let i = 0; i < info.steps; i++) {
      const hold = await page.evaluate(k => GIF.holdMs(k), i);
      const count = Math.max(1, Math.round((hold * fps) / 1000));
      await page.evaluate(k => GIF.begin(k), i);
      for (let f = 0; f < count; f++) {
        await page.evaluate(t => GIF.seek(t), (f * 1000) / fps);
        await page.screenshot({ path: join(frames, `${String(n++).padStart(5, '0')}.png`) });
      }
      await page.evaluate(k => GIF.end(k), i);
    }
    await browser.close();

    const out = resolve(opts.out);
    mkdirSync(dirname(out), { recursive: true });
    const input = ['-y', '-v', 'error', '-framerate', String(fps), '-i', join(frames, '%05d.png')];
    execFileSync('ffmpeg', [
      ...input,
      '-vf', 'split[a][b];[a]palettegen=stats_mode=diff[p];[b][p]paletteuse=dither=bayer:bayer_scale=5:diff_mode=rectangle',
      '-loop', '0', out,
    ], { stdio: 'inherit' });
    if (opts.mp4) {
      execFileSync('ffmpeg', [
        ...input, '-c:v', 'libx264', '-pix_fmt', 'yuv420p', '-crf', '18', '-movflags', '+faststart',
        out.replace(/\.gif$/i, '') + '.mp4',
      ], { stdio: 'inherit' });
    }
    if (opts.keepFrames) console.log(`frames kept in ${frames}`);
    else rmSync(frames, { recursive: true, force: true });

    const mb = statSync(out).size / 1024 / 1024;
    const secs = n / fps;
    console.log(`${out}\n${WIDTH}x${height} · ${n} frames @ ${fps} fps · ${secs.toFixed(1)} s · ${mb.toFixed(2)} MB`);
    if (mb > SOFT_LIMIT_MB) {
      console.warn(`WARNING: ${mb.toFixed(1)} MB is heavy for a LinkedIn post — retry with --fps 15 or fewer/shorter steps.`);
    }
  } else {
    await browser.close();
  }

  if (errors.length) console.warn(`page errors:\n${errors.join('\n')}`);
}

main().catch(e => {
  console.error(e.message || e);
  process.exit(1);
});
