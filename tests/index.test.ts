import { it, describe } from "node:test";

import { Worker } from "node:worker_threads";

const tests = {
  "lastLoopIters()": "./tests/last-loop-iters.ts",
};

for (const key in tests) {
  const path = tests[key as keyof typeof tests];

  describe(key, () => {
    it("main thread", async () => {
      await import(path);
    });

    it("worker thread", async () => {
      await using _ = new Worker(path, {
        eval: true,
      });
    });
  });
}
