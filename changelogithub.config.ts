import { defineConfig } from 'changelogithub';

export default defineConfig({
  types: {
    feat: { title: 'Features' },
    fix: { title: 'Bug Fixes' },
    perf: { title: 'Performance' },
    refactor: { title: 'Refactor' },
    docs: { title: 'Documentation' },
  },
  titles: {
    breakingChanges: '💥 BREAKING CHANGES 💥',
  },
});
