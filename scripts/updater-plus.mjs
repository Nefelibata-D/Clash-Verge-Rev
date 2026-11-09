import { copyFile, mkdir, readdir, readFile, writeFile } from 'node:fs/promises'
import path from 'node:path'

const [input, output] = process.argv.slice(2)
const version = process.env.RELEASE_VERSION
const tag = version?.replace(/^(\d+\.\d+\.\d+)-(\d+)$/, 'v$1.$2')
const repository = process.env.GITHUB_REPOSITORY
if (!input || !output || !/^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)-([1-9]\d*)$/.test(version ?? '')) {
  throw new Error(
    'Usage: RELEASE_VERSION=X.Y.Z-N node scripts/updater-plus.mjs <bundles> <output>',
  )
}
if (!/^[\w.-]+\/[\w.-]+$/.test(repository ?? '')) {
  throw new Error('GITHUB_REPOSITORY must be owner/repository')
}

async function filesIn(directory) {
  const files = []
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const file = path.join(directory, entry.name)
    if (entry.isDirectory()) files.push(...(await filesIn(file)))
    else if (entry.isFile()) files.push(file)
  }
  return files
}

const files = await filesIn(input)
const platforms = {}
const packages = []
for (const [extension, targets] of [
  ['.exe', ['windows-x86_64', 'windows-x86_64-nsis']],
  ['.deb', ['linux-x86_64', 'linux-x86_64-deb']],
]) {
  const matches = files.filter((file) => file.endsWith(extension))
  if (matches.length !== 1)
    throw new Error(
      `Expected one ${extension} installer, found ${matches.length}`,
    )
  const file = matches[0]
  const signature = (await readFile(`${file}.sig`, 'utf8')).trim()
  if (!signature) throw new Error(`Empty signature for ${file}`)
  const sourceName = path.basename(file)
  const versionField = `_${version}_`
  if (!sourceName.includes(versionField))
    throw new Error(`Installer name does not contain ${versionField}: ${sourceName}`)
  const name = sourceName.replace(versionField, `_${tag.slice(1)}_`)
  const entry = {
    signature,
    url: `https://github.com/${repository}/releases/download/${tag}/${encodeURIComponent(name)}`,
  }
  for (const target of targets) platforms[target] = entry
  packages.push([file, name], [`${file}.sig`, `${name}.sig`])
}

await mkdir(output, { recursive: true })
for (const [file, name] of packages)
  await copyFile(file, path.join(output, name))
await writeFile(
  path.join(output, 'latest.json'),
  JSON.stringify(
    {
      version,
      notes: `Clash Verge Rev ${tag.slice(1)} for Windows and Linux AMD64.`,
      pub_date: new Date().toISOString(),
      platforms,
    },
    null,
    2,
  ) + '\n',
)
console.log(`Prepared ${packages.length / 2} signed installers and latest.json`)
