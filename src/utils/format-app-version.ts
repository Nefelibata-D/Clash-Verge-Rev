export const formatAppVersion = (version: string) =>
  version.replace(/^(\d+\.\d+\.\d+)-(\d+)$/, '$1.$2')
