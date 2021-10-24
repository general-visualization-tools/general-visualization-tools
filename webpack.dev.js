const path = require("path");
const { merge } = require("webpack-merge");
const common = require("./webpack.common.js");

module.exports = merge(common, {
  mode: "development",
  devtool: "eval-source-map",
  output: {
    path: path.resolve(__dirname, "dist_dev/"),
    filename: "bundle.js",
  },
  devServer: {
    static: "./dist_dev",
    open: true,
  },
});
