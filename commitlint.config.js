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
    ]]
  }
};
