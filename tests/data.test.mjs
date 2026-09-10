import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { projects, experience } from '../src/data.js';

test('portfolio destinations and selectable experience have complete content', () => {
  assert.equal(new Set(experience.map(job => job.company)).size, experience.length);
  for (const item of [...projects, ...experience]) {
    assert.ok(item.description && item.tags.length);
    if (item.href) assert.equal(new URL(item.href).protocol, 'https:');
    else assert.ok(item.company && item.role && item.date);
  }
});

test('original portfolio backup matches its verified manifest', () => {
  const folder = new URL('../backup/portfolio-before-react-2026-09-10/', import.meta.url);
  const manifest = JSON.parse(readFileSync(new URL('BACKUP-MANIFEST.json', folder)));
  for (const [file, hash] of Object.entries(manifest)) {
    assert.equal(createHash('sha256').update(readFileSync(new URL(file, folder))).digest('hex'), hash, file);
  }
});
