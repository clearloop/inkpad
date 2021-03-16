import type webpack from "webpack";
import * as path from "path";
import CopyWebpackPlugin from "copy-webpack-plugin";
import MiniCssExtractPlugin from "mini-css-extract-plugin";
import AutoPreprocess from "svelte-preprocess";

const common: webpack.Configuration = {
  target: "electron-renderer",
  entry: path.resolve(__dirname, "./ui/main.ts"),
  devtool: "cheap-module-source-map",
  module: {
    rules: [
      {
        test: /\.css$/,
        use: [MiniCssExtractPlugin.loader, "css-loader"],
      },
      {
        test: /\.(html|svelte)$/,
        use: {
          loader: "svelte-loader",
          options: {
            preprocess: AutoPreprocess(),
            emitCss: true,
          },
        },
      },
      {
        test: /\.tsx?$/,
        use: "ts-loader",
      },
      {
        test: /\.(woff|woff2|eot|ttf|otf)$/i,
        type: "asset/resource",
      },
    ],
  },
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        "native/preload.js",
        { from: "electron.json", to: "package.json" },
      ],
    }),
    new MiniCssExtractPlugin({
      filename: "bundle.css",
    }),
  ],
  output: {
    filename: "bundle.js",
    path: path.resolve(__dirname, "public"),
    clean: {
      dry: true,
    },
  },
  resolve: {
    extensions: [".tsx", ".ts", ".js"],
    alias: {
      "@ceres": [
        path.resolve(__dirname, "ui/"),
        path.resolve(__dirname, "ui/src"),
      ],
      "@design": path.resolve(__dirname, "ui/DesignSystem/"),
      "@native": path.resolve(__dirname, "native/"),
    },
  },
};

export default common;
