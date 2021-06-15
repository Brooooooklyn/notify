import test from 'ava'

import { watch } from '../index'

const noop = () => {}

test('should watch and unwatch', (t) => {
  const unwatch = watch(process.cwd(), noop)
  t.notThrows(() => {
    unwatch()
  })
})
