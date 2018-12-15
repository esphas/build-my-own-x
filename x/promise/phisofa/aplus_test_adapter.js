
const XPromise = require('./phisofa');

function deferred() {
  let promise, resolve, reject;
  promise = new XPromise((_resolve, _reject) => {
    resolve = _resolve;
    reject = _reject;
  });
  return { promise, resolve, reject };
}

module.exports = { deferred };
