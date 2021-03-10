import * as path from "path";
import CopyWebpackPlugin from "copy-webpack-plugin";
import HtmlWebpackPlugin from "html-webpack-plugin";
import webpack from "webpack";

export const common: webpack.Configuration = {
  target: "web",
  entry: {
    app: path.resolve(__dirname, "./ui/index.tsx"),
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
    ],
  },
  plugins: [
    new HtmlWebpackPlugin({
      bask: false,
      title: "Ceres",
      chunks: ["app"],
      templateContent: `<div id="root"></div>`,
      favicon: "public/favicon.ico",
    }),
  ],
  output: {
    filename: "[name].bundle.js",
    path: path.resolve(__dirname, "app"),
    clean: {
      dry: true,
    },
  },
};

export const electronCommon: webpack.Configuration = {
  target: "electron-main",
  entry: {
    main: path.resolve(__dirname, "./native/main.ts"),
    preload: path.resolve(__dirname, "./native/preload.ts"),
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
      {
        test: /\.node$/,
        use: "node-loader",
      },
    ],
  },
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        { from: "electron.json", to: "package.json" },
        { from: "public", to: "public" },
      ],
    }),
  ],
  output: {
    filename: "[name].bundle.js",
    path: path.resolve(__dirname, "app"),
    clean: {
      dry: true,
    },
  },
};
