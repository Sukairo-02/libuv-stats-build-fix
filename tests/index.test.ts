import { afterEach, beforeEach, it } from 'node:test';
import assert from 'node:assert';

import { setImmediate } from 'node:timers/promises';

import { lastLoopIters, register, unregister } from '../index.js';

beforeEach(register);
afterEach(unregister);

it('iters()', async () => {
  await setImmediate();
  assert.ok(lastLoopIters() === 1);

  await setImmediate();
  assert.ok(lastLoopIters() === 1);
});
