import { lastLoopIters, track, untrack } from "../../index.js";
import assert from "node:assert";
import { setImmediate } from "node:timers/promises";

track();
assert.strictEqual(lastLoopIters(), 0);

await setImmediate();
assert.strictEqual(lastLoopIters(), 1);

await setImmediate();
await setImmediate();
assert.strictEqual(lastLoopIters(), 2);
untrack();
