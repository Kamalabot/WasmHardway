const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const webpack = require("webpack");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "production",
  entry: {
    index: "./index.js"
  },
  output: {
    path: dist,
    filename: "index.js"
  },
  plugins: [
    new HtmlWebpackPlugin(),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname,".")
    }),
  ],
  mode: "development",
  experiments: {
    asyncWebAssembly: true
  }
};
