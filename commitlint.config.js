module.exports = {
  extends: ['@commitlint/config-conventional'],
  rules: {
    'scope-enum': [2, 'always', [
      'deploy',
      'scheduler',
      'notify',
      'ui',
      'frontend',
      'db',
      'pwa',
      'design',
      'ntfy',
      'docker',
      'ci',
      'test',
      'release'
    ]],
    // Disable line length limits for semantic-release commits with long changelogs
    'body-max-line-length': [0, 'always'],
    'footer-max-line-length': [0, 'always']
  }
};
