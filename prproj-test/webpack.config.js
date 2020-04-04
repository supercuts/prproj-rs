const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
    entry: "./bootstrap.js",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "bootstrap.js",
    },
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                use: 'ts-loader',
                exclude: /node_modules/,
            },
        ]
    },
    resolve: {
        extensions: ['.tsx', '.ts', '.js'],
        modules: [
            path.join(__dirname, '../../prproj-ts/src'),
            path.join(__dirname, '../../prproj-ts/node_modules'),
            'node_modules'
        ]
    },
    node: {
        fs: "empty"
    },
    mode: "development",
    plugins: [
        new CopyWebpackPlugin(['index.html'])
    ],
};
