const VueLoaderPlugin = require('vue-loader/lib/plugin')

module.exports = {
    entry: './src/index.js',
    mode:    process.env.NODE_ENV === 'production' ? 'production' : 'development',
    devtool: process.env.NODE_ENV === 'production' ? false : 'inline-source-map',
    module: {
        rules: [
            {
                test: /\.vue$/,
                loader: "vue-loader"
            }
        ]
    },
    output: {
        path: __dirname + '/build',
        filename: 'bundle.js'
    },
    devServer: {
        contentBase: __dirname,
        compress: true,
        port: 8000,
    },
    resolve: {
        alias: {
            vue$: "vue/dist/vue.esm.js"
        },
        extensions: ["*", ".js", ".vue", ".json"]
    },
    plugins: [new VueLoaderPlugin()],
}
