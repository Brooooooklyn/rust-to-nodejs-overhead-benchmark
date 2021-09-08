import { createRequire } from 'module'
import { fileURLToPath } from 'url'

import b from 'benny'

const require = createRequire(fileURLToPath(import.meta.url))

const { sum: napiSum } = require('./napi-rs.node')
const { sum: napiCompactSum } = require('./napi-rs-compact.node')
const { sum: neonSum } = require('./neon.node')
const { sum: nodeBindgenSum } = require('./node-bindgen.node')

const benchSum = () =>
  b.suite(
    'Sum',
    b.add('napi-rs', () => napiSum(1, 2)),
    b.add('napi-rs-compact', () => napiCompactSum(1, 2)),
    b.add('neon', () => neonSum(1, 2)),
    b.add('node-bindgen', () => nodeBindgenSum(1, 2)),
    b.cycle(),
    b.complete()
  )

await benchSum()
