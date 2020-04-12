const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');

const distPath = path.resolve(__dirname, 'dist');
module.exports = (env, argv) => {
  return {
    devServer: {
      contentBase: distPath,
      compress: argv.mode === 'production',
      port: 8000,
    },
    entry: './bootstrap.js',
    output: {
      path: distPath,
      filename: 'seed-testbed.js',
      webassemblyModuleFilename: 'seed-testbed.wasm'
    },
    plugins: [
      argv.mode !== 'production' ? function(compiler) {
        // This plugin enables recompilation whenever stuff in rust has changed
        compiler.hooks.afterCompile.tap('CSSinRust', (compilation) => {
          compilation.contextDependencies.add(path.resolve(__dirname, '../../src'));
          return true;
        });
      } : undefined,
      new CopyWebpackPlugin([
        { from: './static', to: distPath }
      ]),
      new WasmPackPlugin({
        crateDirectory: '.',
        extraArgs: '--no-typescript',
      }),
    ],
    watch: argv.mode !== 'production',
  };
};
