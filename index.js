const { loadBinding } = require('@node-rs/helper')

const { watch: rawWatch, unwatch } = loadBinding(__dirname, 'notify', '@napi-rs/notify')

module.exports = {
  watch: function watch(dir, cb) {
    const watcher = rawWatch(dir, (err, evt) => {
      cb(err, JSON.parse(evt))
    })

    return () => unwatch(watcher, dir)
  },
}
