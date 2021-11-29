import { createRequire } from 'module'
import { fileURLToPath } from 'url'

import b from 'benny'

const require = createRequire(fileURLToPath(import.meta.url))

const {
  sum: napiSum,
  helloPlusWorld: napiHelloPlusWorld,
} = require('./napi-rs.node')
const {
  sum: napiCompactSum,
  helloPlusWorld: napiCompactHelloPlusWorld,
} = require('./napi-rs-compact.node')
const {
  sum: neonSum,
  helloPlusWorld: neonHelloPlusWorld,
} = require('./neon.node')
const {
  sum: nodeBindgenSum,
  helloPlusWorld: nodeBindgenHelloPlusWorld,
} = require('./node-bindgen.node')

function jsPlus(a, b) {
  return a + b
}

function jsHelloPlusWorld(hello) {
  return `${hello} world`
}

await b.suite(
  'Sum',
  b.add('JavaScript', () => jsPlus(1, 2)),
  b.add('napi-rs', () => napiSum(1, 2)),
  b.add('napi-rs-compact', () => napiCompactSum(1, 2)),
  b.add('neon', () => neonSum(1, 2)),
  b.add('node-bindgen', () => nodeBindgenSum(1, 2)),
  b.cycle(),
  b.complete()
)

await b.suite(
  'Hello Plus World',
  b.add('JavaScript', () => jsHelloPlusWorld('Hello')),
  b.add('napi-rs', () => napiHelloPlusWorld('Hello')),
  b.add('napi-rs-compact', () => napiCompactHelloPlusWorld('Hello')),
  b.add('neon', () => neonHelloPlusWorld('Hello')),
  b.add('node-bindgen', () => nodeBindgenHelloPlusWorld('Hello')),
  b.cycle(),
  b.complete()
)
