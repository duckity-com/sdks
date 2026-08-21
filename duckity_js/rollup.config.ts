import typescript from "@rollup/plugin-typescript";
import { wasm } from "@rollup/plugin-wasm";
import { nodeResolve } from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";
import terser from "@rollup/plugin-terser";
import { dts } from "rollup-plugin-dts";

export default [
  {
    input: "src/index.ts",
    output: [
      {
        file: "dist/index.js",
        format: "esm",
      },
    ],
    plugins: [
      typescript({
        declaration: true,
        declarationDir: "dist/types",
        rootDir: "src",
        tsconfig: "tsconfig.json",
      }),
      nodeResolve(),
      commonjs(),
      terser(),
    ],
  },
  {
    input: "src/processing/worker.ts",
    output: [
      {
        file: "dist/worker.js",
        format: "esm",
      },
    ],
    plugins: [
      typescript({
        declaration: true,
        declarationDir: "dist/types",
        rootDir: "src",
        tsconfig: "tsconfig.json",
      }),
      nodeResolve(),
      commonjs(),
      terser(),
    ],
  },
  {
    input: "dist/types/index.d.ts",
    output: {
      file: "dist/index.d.ts",
      format: "es",
    },
    plugins: [dts()],
  },
];
