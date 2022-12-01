module.exports = {
  root: true,
  parser: '@typescript-eslint/parser',
  parserOptions: {
    tsconfigRootDir: __dirname,
    project: ['./tsconfig.json'],
  },
  plugins: ['@typescript-eslint'],
  env: {
    node: true,
    es6: true,
  },
  extends: [
    'eslint:recommended',
    'plugin:@typescript-eslint/recommended',
    'plugin:@typescript-eslint/recommended-requiring-type-checking',
  ],
  rules: {
    'comma-dangle': ['error', 'always-multiline'],
    'semi-style': ['error', 'last'],
    'no-extra-semi': 'error',
    semi: ['error', 'always'],
    'no-unexpected-multiline': 'error',
    'func-call-spacing': ['error', 'never'],
    'space-unary-ops': [
      'error',
      {
        words: true,
        nonwords: false,
      },
    ],
    'semi-spacing': [
      'error',
      {
        before: false,
        after: true,
      },
    ],
    'block-spacing': ['error', 'always'],
    'space-in-parens': ['error', 'never'],
    'array-bracket-spacing': ['error', 'never'],
    'object-curly-spacing': ['error', 'always'],
    'comma-spacing': [
      'error',
      {
        before: false,
        after: true,
      },
    ],
    'space-infix-ops': ['error'],
    'keyword-spacing': [
      'error',
      {
        before: true,
        after: true,
      },
    ],
    'computed-property-spacing': ['error', 'never'],
    'prefer-arrow-callback': ['error'],
    'no-confusing-arrow': 'error',
    'no-constant-condition': 'error',
    'arrow-parens': ['error', 'always'],
    quotes: ['error', 'single'],
    '@typescript-eslint/no-floating-promises': ['error'],
  },
};
