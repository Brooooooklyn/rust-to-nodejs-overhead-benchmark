import { createRequire } from 'module'
import { fileURLToPath } from 'url'

import b from 'benny'

const require = createRequire(fileURLToPath(import.meta.url))

const {
  sum: napiSum,
  helloPlusWorld: napiHelloPlusWorld,
  area: napiArea,
} = require('./napi-rs.node')
const {
  sum: napiCompactSum,
  helloPlusWorld: napiCompactHelloPlusWorld,
  area: napiCompactArea,
} = require('./napi-rs-compact.node')
const {
  sum: neonSum,
  helloPlusWorld: neonHelloPlusWorld,
  area: neonArea,
} = require('./neon.node')

function jsArea(rect) {
  return rect.width * rect.height
}

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
  b.cycle(),
  b.complete()
)

await b.suite(
  'Hello Plus World',
  b.add('JavaScript', () => jsHelloPlusWorld('Hello')),
  b.add('napi-rs', () => napiHelloPlusWorld('Hello')),
  b.add('napi-rs-compact', () => napiCompactHelloPlusWorld('Hello')),
  b.add('neon', () => neonHelloPlusWorld('Hello')),
  b.cycle(),
  b.complete()
)

await b.suite(
  'Rect Area',
  b.add('JavaScript', () => jsArea({ width: 200, height: 400 })),
  b.add('napi-rs', () => napiArea({ width: 200, height: 400 })),
  b.add('napi-rs-compact', () => napiCompactArea({ width: 200, height: 400 })),
  b.add('neon', () => neonArea({ width: 200, height: 400 })),
  b.cycle(),
  b.complete()
)
