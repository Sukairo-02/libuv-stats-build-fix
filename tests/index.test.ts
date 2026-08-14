import { afterEach, beforeEach, it } from 'node:test';
import assert from 'node:assert';

import { setImmediate } from 'node:timers/promises';

import { lastLoopIters, track, untrack } from '../index.js';

beforeEach(track);
afterEach(untrack);

it('iters()', async () => {
  assert.ok(lastLoopIters() === 0);

  await setImmediate();
  assert.ok(lastLoopIters() === 1);

  await setImmediate();
  await setImmediate();
  assert.ok(lastLoopIters() === 2);
});
