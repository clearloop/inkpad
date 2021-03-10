import { common, electronCommon } from "./webpack.common";
import * as webpack from "webpack";
import { merge } from "webpack-merge";
import * as path from "path";

const dev: webpack.Configuration = merge(common, {
  mode: "production",
  devtool: "inline-source-map",
  devServer: {
    contentBase: path.join(__dirname, "app"),
    compress: true,
    port: 9000,
    lazy: true,
    filename: "app.bundle.js",
    historyApiFallback: true,
  },
});

const electronDev: webpack.Configuration = merge(electronCommon, {
  mode: "development",
  devtool: "inline-source-map",
});

export default [dev, electronDev];
