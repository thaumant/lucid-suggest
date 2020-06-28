
module.exports = {
  entry: './src/index.js',
  mode: 'development',
  devtool: 'inline-source-map',
  output: {
    path: __dirname + '/build',
    filename: 'bundle.js'
  },
  devServer: {
    contentBase: __dirname,
    compress: true,
    port: 8000,
  },
}
