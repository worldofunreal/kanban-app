import js from '@eslint/js';
import globals from 'globals';
import tseslint from '@typescript-eslint/eslint-plugin';
import tsparser from '@typescript-eslint/parser';
import pluginVue from 'eslint-plugin-vue';
import prettier from 'eslint-config-prettier';

export default [
  // Global ignores
  {
    ignores: ['dist/**', 'node_modules/**'],
  },
  
  // Base JavaScript/TypeScript config
  {
    files: ['**/*.{js,jsx,ts,tsx}'],
    languageOptions: {
      ecmaVersion: 'latest',
      sourceType: 'module',
      globals: {
        ...globals.browser,
        ...globals.node,
      },
      parser: tsparser,
      parserOptions: {
        ecmaVersion: 'latest',
        sourceType: 'module',
      },
    },
    plugins: {
      '@typescript-eslint': tseslint,
    },
    rules: {
      ...js.configs.recommended.rules,
      'no-console': process.env.NODE_ENV === 'production' ? 'warn' : 'off',
      'no-debugger': process.env.NODE_ENV === 'production' ? 'warn' : 'off',
      'prefer-const': 'error',
      'no-var': 'error',
      'no-unused-vars': 'off', // Use TypeScript version
      '@typescript-eslint/no-unused-vars': 'error',
      'no-undef': 'off', // TypeScript handles this
    },
  },
  
  // Vue files config using the proper flat config approach
  ...pluginVue.configs['flat/recommended'],
  {
    files: ['**/*.vue'],
    languageOptions: {
      ecmaVersion: 'latest',
      sourceType: 'module',
      globals: {
        ...globals.browser,
        ...globals.node,
        defineProps: 'readonly',
        defineEmits: 'readonly',
        defineExpose: 'readonly',
        withDefaults: 'readonly',
      },
      parser: pluginVue.parser,
      parserOptions: {
        ecmaVersion: 'latest',
        sourceType: 'module',
        parser: tsparser,
      },
    },
    plugins: {
      vue: pluginVue,
      '@typescript-eslint': tseslint,
    },
    rules: {
      'no-console': process.env.NODE_ENV === 'production' ? 'warn' : 'off',
      'no-debugger': process.env.NODE_ENV === 'production' ? 'warn' : 'off',
      'vue/multi-word-component-names': 'off',
      'vue/no-unused-vars': 'error',
      'vue/no-unused-components': 'error',
      'prefer-const': 'error',
      'no-var': 'error',
      'no-unused-vars': 'off', // Use TypeScript version
      '@typescript-eslint/no-unused-vars': 'error',
      'no-undef': 'off', // TypeScript handles this
    },
  },
  
  // Prettier config (must be last)
  prettier,
]; 