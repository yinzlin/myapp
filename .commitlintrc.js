# Commit Message Lint 配置
# 参考文档: https://commitlint.js.org/

module.exports = {
  extends: ['@commitlint/config-conventional'],
  rules: {
    'type-enum': [
      2,
      'always',
      [
        'feat',     # 新功能
        'fix',      # 修复 bug
        'docs',     # 文档变更
        'style',    # 代码格式（不影响功能）
        'refactor', # 重构（不是修复也不是新功能）
        'perf',     # 性能优化
        'test',     # 测试相关
        'build',    # 构建系统或依赖变更
        'ci',       # CI 配置变更
        'chore',    # 其他变更
        'revert',   # 回滚
      ],
    ],
    'type-case': [2, 'always', 'lower-case'],
    'subject-case': [2, 'always', 'sentence-case'],
    'subject-empty': [2, 'never'],
    'type-empty': [2, 'never'],
  },
};
