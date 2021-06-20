import path from "path";
import webpack from "webpack";
import HtmlPlugin from "html-webpack-plugin";

const config: webpack.Configuration = {
    mode: "production",
    entry: path.resolve(__dirname, "./index.ts"),
    output: {
        filename: "index.js",
        path: path.resolve(__dirname, "dist"),
        wasmLoading: "fetch",
    },
    resolve: {
        extensions: [".ts", ".js"],
    },
    plugins: [
        new HtmlPlugin({
            title: "Ceres Template",
            templateContent: "<div id='app'></div>",
        }),
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
