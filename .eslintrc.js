'use strict'

const typescriptPlugin = require('@typescript-eslint/eslint-plugin')
const prettierConfig = require('eslint-config-prettier')
const jestPlugin = require('eslint-plugin-jest')
const jestDomPlugin = require('eslint-plugin-jest-dom')
const testingLibraryPlugin = require('eslint-plugin-testing-library')

module.exports = {
  plugins: [
    'formatjs',
    'import',
    'jsx-a11y',
    'prettier',
    'promise',
    'react',
    'react-hooks',
    'unicorn',
  ],
  env: { node: true },
  extends: [
    'eslint:recommended',
    'plugin:import/recommended',
    'plugin:jsx-a11y/recommended',
    'plugin:promise/recommended',
    'plugin:react/recommended',
    'plugin:react-hooks/recommended',
    'plugin:unicorn/recommended',
    'plugin:prettier/recommended',
  ],
  rules: {
    'no-console': ['error', { allow: ['error', 'info', 'warn'] }],
    'no-param-reassign': ['error', { props: true }],
    'formatjs/enforce-default-message': 'error',
    'formatjs/enforce-description': 'error',
    'formatjs/enforce-placeholders': 'error',
    'formatjs/enforce-plural-rules': ['error', { other: true }],
    'formatjs/no-multiple-whitespaces': 'error',
    'import/order': [
      'error',
      {
        groups: [
          ['builtin', 'external'],
          ['internal', 'parent', 'sibling', 'index', 'unknown'],
        ],
        'newlines-between': 'always',
      },
    ],
    'react/jsx-uses-react': 'off',
    'react/react-in-jsx-scope': 'off',
    'unicorn/prefer-module': 'off',
    'unicorn/prefer-node-protocol': 'off',
    'unicorn/prevent-abbreviations': 'off',
    'prettier/prettier': 'error',
  },
  overrides: [
    {
      files: ['*.ts?(x)'],
      parser: typescriptPlugin.configs.base.parser,
      parserOptions: {
        ...typescriptPlugin.configs.base.parserOptions,
        project: 'tsconfig.json',
      },
      plugins: typescriptPlugin.configs.base.plugins,
      rules: {
        ...typescriptPlugin.configs.recommended.rules,
        ...typescriptPlugin.configs['eslint-recommended'].overrides[0].rules,
        ...typescriptPlugin.configs['recommended-requiring-type-checking']
          .rules,
        'import/named': 'off',
        ...prettierConfig.rules,
      },
    },
    {
      files: ['?(*.)test.ts?(x)'],
      globals: jestPlugin.environments.globals.globals,
      plugins: [
        ...jestPlugin.configs.recommended.plugins,
        ...jestDomPlugin.configs.recommended.plugins,
        ...testingLibraryPlugin.configs.react.plugins,
      ],
      rules: {
        ...jestPlugin.configs.recommended.rules,
        ...jestDomPlugin.configs.recommended.rules,
        ...testingLibraryPlugin.configs.react.rules,
      },
    },
  ],
  settings: {
    'import/extensions': ['.d.ts', '.js', '.ts', '.tsx'],
    'import/parsers': { '@typescript-eslint/parser': ['.d.ts', '.ts', '.tsx'] },
    'import/resolver': {
      typescript: { alwaysTryTypes: true, project: 'tsconfig.json' },
    },
    react: { version: 'latest' },
  },
}
