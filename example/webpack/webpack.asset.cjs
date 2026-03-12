const path = require("path");

module.exports = {
    entry: "./index.asset.js",
    output: {
        filename: "index.js",
        path: path.resolve(__dirname, "dist"),
    },
    module: {
        rules: [
            {
                test: /\.wasm/,
                type: "asset/resource",
            },
        ],
    },
};
