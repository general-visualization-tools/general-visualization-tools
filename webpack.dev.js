const path = require("path");
const { merge } = require("webpack-merge");
const common = require("./webpack.common.js");
const ReactRefreshWebpackPlugin = require("@pmmmwh/react-refresh-webpack-plugin");

module.exports = merge(common, {
  mode: "development",
  devtool: "eval-source-map",
  module: {
    rules: [
      {
        test: /\.(js|ts|tsx)$/,
        exclude: /(node_modules|bower_components)/,
        loader: "babel-loader",
        options: {
          presets: ["@babel/env"],
          // この設定を本番ビルドに含めてしまうと、動かなくなるため、dev.jsとprod.js両方にbabel-loaderの設定を書いている
          plugins: ["react-refresh/babel"],
        },
      },
    ],
  },
  output: {
    path: path.resolve(__dirname, "dist_dev/"),
    filename: "bundle.js",
  },
  devServer: {
    static: "./dist_dev",
    open: true,
  },
  plugins: [new ReactRefreshWebpackPlugin()],
});
