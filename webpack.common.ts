import * as path from "path";
import * as CopyWebpackPlugin from "copy-webpack-plugin";
import * as HtmlWebpackPlugin from "html-webpack-plugin";
import * as webpack from "webpack";

export const electronCommon: webpack.Configuration = {
  target: "electron-main",
  entry: {
    main: path.resolve(__dirname, "./native/main.ts"),
  },
  output: {
    filename: "[name].bundle.js",
    path: path.resolve(__dirname, "app/native"),
    clean: {
      dry: true,
    },
  },
};

export const common: webpack.Configuration = {
  target: "web",
  entry: {
    app: path.resolve(__dirname, "./ui/index.ts"),
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
      title: "Ceres",
      chunks: ["app"],
      // ptemplate: "./public/index.html",
      favicon: "./public/favicon.ico",
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
