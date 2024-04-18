const path = require('path')

module.exports = {
  entry: './index.async.js',
  output: {
    filename: 'index.js',
    path: path.resolve(__dirname, 'dist'),
  },
  experiments: {
    asyncWebAssembly: true,
  },
}
