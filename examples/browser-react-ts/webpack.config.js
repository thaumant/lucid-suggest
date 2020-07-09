
module.exports = {
  entry: './src/index.tsx',
  mode:    process.env.NODE_ENV === 'production' ? 'production' : 'development',
  devtool: process.env.NODE_ENV === 'production' ? false : 'inline-source-map',
  output: {
    path: __dirname + '/build',
    filename: 'bundle.js'
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
      extensions: ['.ts', '.tsx', '.js', '.json']
  },
  devServer: {
    contentBase: __dirname,
    compress: true,
    port: 8000,
  },
}
