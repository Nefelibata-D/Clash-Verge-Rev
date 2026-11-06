import { readFile, writeFile } from 'node:fs/promises'

const version = process.argv[2]?.replace(/^v/, '')
if (!/^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)-([1-9]\d*)$/.test(version ?? '')) {
  throw new Error(
    'Expected an official stable version with a revision, e.g. 2.5.10-1',
  )
}

const files = [
  ['package.json', /("version"\s*:\s*")[^"]+(")/],
  ['src-tauri/tauri.conf.json', /("version"\s*:\s*")[^"]+(")/],
  ['src-tauri/Cargo.toml', /(^version\s*=\s*")[^"]+(")/],
  [
    'Cargo.lock',
    /(\[\[package\]\]\r?\nname = "clash-verge"\r?\nversion = ")[^"]+(")/,
  ],
]
const updates = []
for (const [file, pattern] of files) {
  const source = await readFile(file, 'utf8')
  const expression = new RegExp(pattern, 'm')
  if (!expression.test(source))
    throw new Error(`Missing application version in ${file}`)
  updates.push([
    file,
    source.replace(expression, (_, before, after) => before + version + after),
  ])
}
for (const [file, source] of updates) await writeFile(file, source)
console.log(`Application version: ${version}`)
