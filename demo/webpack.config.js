
module.exports = {
  entry: './src/index.js',
  mode:    process.env.NODE_ENV === 'production' ? 'production' : 'development',
  devtool: process.env.NODE_ENV === 'production' ? false : 'inline-source-map',
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
