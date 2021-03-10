import { common, electronCommon } from "./webpack.common";
import webpack from "webpack";
import { merge } from "webpack-merge";
import path from "path";

const dev: webpack.Configuration = merge(common, {
  mode: "development",
  devtool: "inline-source-map",
  devServer: {
    contentBase: path.resolve(__dirname, "app"),
    compress: true,
    port: 9000,
    historyApiFallback: true,
    writeToDisk: true,
  },
});

const electronDev: webpack.Configuration = merge(electronCommon, {
  mode: "development",
  devtool: "inline-source-map",
});

export default [dev, electronDev];
