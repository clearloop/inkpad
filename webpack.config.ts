import * as path from "path";
import CopyWebpackPlugin from "copy-webpack-plugin";
import HtmlWebpackPlugin from "html-webpack-plugin";
import webpack from "webpack";

const common: webpack.Configuration = {
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

export default common;
