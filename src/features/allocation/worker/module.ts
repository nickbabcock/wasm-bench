import wasmPath from "../../../../packages/alloc/pkg/alloc_bg.wasm";
import { timeit } from "@/lib/timeit";
import { load } from "@/lib/wasm";

export type BenchmarkResult = {
  elapsedMs: number;
};

const alloc = load({
  js: () => import("../../../../packages/alloc/pkg/alloc"),
  wasm: wasmPath,
});

export async function abc(
  corpus: string,
  iterations: number,
) {
  const mod = await alloc();
  const input = await fetch("https://pdx.tools/eu4/saves/l3mDIfueYIB-gjB0gOliK/file").then((x) => x.arrayBuffer());
  const [def, elapsedMs] = await timeit(() => mod.abc(new Uint8Array(input)));
  console.log(def.data_at(0), def.data_at(1), def.data_at(2));
  def.free();
  return { def, elapsedMs };
}

export async function allocation(
  corpus: string,
  iterations: number,
): Promise<BenchmarkResult> {
  const mod = await alloc();
  const [_, elapsedMs] = await timeit(() => mod.allocation(corpus, iterations));

  return { elapsedMs };
}
