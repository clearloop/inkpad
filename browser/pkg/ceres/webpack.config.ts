import path from "path";
import webpack from "webpack";
import WasmPackPlugin from "@wasm-tool/wasm-pack-plugin";

const config: webpack.Configuration = {
    entry: "./index.ts",
    mode: "production",
    output: {
        filename: "index.js",
        path: path.resolve(__dirname, "lib"),
        wasmLoading: "fetch",
    },
    resolve: {
        extensions: [".ts", ".wasm"],
    },
    plugins: [
        new WasmPackPlugin({
            crateDirectory: path.resolve("../../"),
            extraArgs: "--scope patract",
            outDir: path.resolve("../browser"),
        }) as any,
    ],
    module: {
        rules: [
            {
                test: /\.ts$/,
                use: "ts-loader",
                exclude: /node_modules/,
            },
        ],
    },
    experiments: {
        asyncWebAssembly: true,
    },
};

export default config;
