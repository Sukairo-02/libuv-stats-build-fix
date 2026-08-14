import { bench, env } from "measure-loop";
import reporter from "measure-loop/reporter";

bench().run({ env, reporter });
