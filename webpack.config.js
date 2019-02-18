const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const CleanWebpackPlugin = require("clean-webpack-plugin");

console.log(__dirname)
module.exports = {
    entry: "./src/index.js",
    output: {
        globalObject: "self",
        path: path.resolve(__dirname, "./dist"),
    },
    plugins: [
        new WasmPackPlugin({ crateDirectory: "./src/crate" }),
        new HtmlWebpackPlugin({ template: "./src/index.html" }),
        new CleanWebpackPlugin(["./dist", "./sc/crate/pkg"]),
    ],
};