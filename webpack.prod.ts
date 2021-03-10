import { common, electronCommon } from "./webpack.common";
import webpack from "webpack";
import { merge } from "webpack-merge";

const prod: webpack.Configuration = merge(common, {
  mode: "development",
  devtool: "inline-source-map",
});

const electronProd: webpack.Configuration = merge(electronCommon, {
  mode: "development",
  devtool: "inline-source-map",
});

export default [prod, electronProd];
