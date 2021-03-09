import common from "./webpack.common";
import * as webpack from "webpack";
import { merge } from "webpack-merge";

const config: webpack.Configuration = merge(common, {
  mode: "development",
  devtool: "inline-source-map",
});

export default config;
