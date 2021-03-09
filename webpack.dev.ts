import common from "./webpack.common";
import * as webpack from "webpack";
import { merge } from "webpack-merge";

const config: webpack.Configuration = merge(common, {
  mode: "production",
});

export default config;
