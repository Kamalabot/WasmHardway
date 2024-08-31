const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const webpack = require("webpack");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "production",
  entry: {
    // index: "./js/index.js"
    index: "./index.js"
  },
  output: {
    path: dist,
    // filename: "[name].js"
    filename: "index.js"
  },
  // devServer: {
  //   contentBase: dist,
  // },
  plugins: [
    new HtmlWebpackPlugin(),
    new CopyPlugin([
      path.resolve(__dirname, "static")
    ]),

    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname,".")
    }),
  ],
  mode: "development",
  experiments: {
    asyncWebAssembly: true
  }
};
