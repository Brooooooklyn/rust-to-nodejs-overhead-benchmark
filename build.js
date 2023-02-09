const { execSync } = require('child_process')
const { copyFileSync } = require('fs')
const { platform } = require('os')
const { join } = require('path')

execSync('cargo build --release', {
  stdio: 'inherit',
})

const libraries = ['napi-rs', 'napi-rs-compact', 'neon', 'node-bindgen']

const PLATFORM = platform()

const prefix = PLATFORM === 'win32' ? '' : 'lib'

let ext

switch (PLATFORM) {
  case 'win32':
    ext = 'dll'
    break
  case 'darwin':
    ext = 'dylib'
    break
  default:
    ext = 'so'
    break
}

for (const lib of libraries) {
  const filename = `${prefix}${lib}-example.${ext}`
  copyFileSync(
    join(__dirname, 'target', 'release', filename.replace(/-/g, '_')),
    join(__dirname, `${lib}.node`)
  )
}
