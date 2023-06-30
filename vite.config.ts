/// <reference types="vitest" />
import vue from '@vitejs/plugin-vue';
import path, { resolve } from 'path';
import AutoImport from 'unplugin-auto-import/vite';
import { defineConfig } from 'vite';
import vueJsx from '@vitejs/plugin-vue-jsx';

export default defineConfig(() => {
  const root = process.cwd();
  return {
    server: {
      strictPort: true,
    },
    clearScreen: false,
    envPrefix: ['VITE_', 'TAURI_'],
    resolve: {
      alias: {
        '@': path.resolve(root, 'src'),
        '#': path.resolve(root, 'types'),
      },
    },
    build: {
      outDir: './dist',
      target: ['es2021', 'chrome100', 'safari13'],
      minify: !!!process.env.TAURI_DEBUG,
      sourcemap: !!process.env.TAURI_DEBUG,
      emptyOutDir: true,
    },
    optimizeDeps: {
      include: ['ant-design-vue/es/locale/zh_CN', 'ant-design-vue/es/locale/en_US'],
    },
    test: {
      include: ['tests/unit/**/*.{test,spec}.{js,mjs,cjs,ts,mts,cts,jsx,tsx}'],
    },
    css: {
      preprocessorOptions: {
        less: {},
      },
    },
    plugins: [
      vue(),
      vueJsx(),
      AutoImport({
        imports: ['vue'],
        dts: './src/auto-imports.d.ts',
        eslintrc: {
          enabled: true,
          filepath: resolve(__dirname, '.eslintrc-auto-import.json'),
        },
      }),
    ],
  };
});
