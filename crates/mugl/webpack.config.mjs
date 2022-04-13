import * as path from 'path';
import webpack from 'webpack';
import CopyPlugin from 'copy-webpack-plugin';
import TerserPlugin from 'terser-webpack-plugin';

const PRODUCTION = 'production';

const mode = process.env.NODE_ENV || PRODUCTION;
const isProd = mode === PRODUCTION;
export const debug = process.env.DEBUG || !isProd;

const ASSET_DIR = path.resolve('./examples/assets');
const OUTPUT_DIR = path.resolve('./dist');
const WASM_OUTPUT_DIR = path.resolve(
  OUTPUT_DIR,
  `wasm32-unknown-unknown/${isProd ? 'release' : 'debug'}/examples`
);

export default {
  mode,
  entry: {
    examples: {
      import: './examples/js/index.js'
    },
  },
  output: {
    filename: '[name].min.js',
    path: OUTPUT_DIR,
  },
  module: {
    rules: [
      {
        test: /\.m?js$/,
        enforce: 'pre',
        use: 'source-map-loader',
      },
    ],
  },
  resolve: {
    extensions: [ '.js', '.mjs' ],
    alias: {
      'examples.wasm': path.resolve(WASM_OUTPUT_DIR, 'wasm.wasm'),
    },
  },
  optimization: {
    minimize: isProd,
    minimizer: [
      new TerserPlugin({
        terserOptions: {
          ecma: 2015,
          module: true,
          toplevel: true,
          compress: {
            passes: 5,
            drop_console: !debug
          },
          mangle: {
            properties: false
          }
        },
      }),
    ],
  },
  plugins: [
    new webpack.DefinePlugin({
      'MUGL_DEBUG': debug,
    }),
    new CopyPlugin({
      patterns: [
        { from: ASSET_DIR, to: OUTPUT_DIR }
      ],
    })
  ],
  experiments: {
    asyncWebAssembly: true,
    topLevelAwait: true,
  },
  devtool: isProd ? false : 'source-map',
  devServer: {
    static: ASSET_DIR
  }
};
