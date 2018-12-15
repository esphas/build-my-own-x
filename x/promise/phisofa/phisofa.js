/**
 * Build My Own X
 * Where X = Promise: Phisofa
 *
 * @module
 *
 * ---
 *
 * Terminology:
 * - Refer to [Promise/A+](https://promisesaplus.com/) first
 * - Use `resolve` as a synonym for `fulfill`, so as `resolved` and `fulfilled`
 *
 * Related Links:
 * - [Promise/A+](https://promisesaplus.com/)
 * - [Promise](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise)
 * - [The Node.js Event Loop, Timers, and `process.nextTick()`](https://nodejs.org/en/docs/guides/event-loop-timers-and-nexttick/)
 * - [Concurrency model and Event Loop](https://developer.mozilla.org/en-US/docs/Web/JavaScript/EventLoop)
 *
 * ---
 *
 * Types:
 *
 * @callback executor
 * @param {resolve} resolve
 * @param {reject} reject
 * @returns {void}
 *
 * @callback resolve
 * @param {*} value
 * @returns {void}
 *
 * @callback reject
 * @param {*} reason
 * @returns {void}
 *
 * @callback onFulfilled
 * @param {*} value
 *
 * @callback onRejected
 * @param {*} reason
 */
'use strict';

/** Phisofa Promise */
class XPromise {

  /**
   * Create a promise.
   * @param {executor} executor
   */
  constructor(executor) {

    // A promise must be in one of three states:
    // <pending>, <fulfilled>, or <rejected>.
    this.state = 'pending';
    // When fulfilled, a promise must have a value
    this.value = undefined;
    // When rejected, a promise must have a reason
    this.reason = undefined;

    this.thens = [];

    // For convenience, expose these variables

    // Now build callbacks for executor: resolve and reject

    // When a promise is settled (fulfilled/rejected),
    // it must not transition to any other state,
    // and its value/reason must not change.
    const settle = () => {
      this.thens.forEach(setImmediate);
    };
    const resolve = (value) => {
      if (this.state !== 'pending') {
        throw 'Promise already settled!';
      }
      // when resolved, set value and transition to <fulfilled>
      this.value = value;
      this.state = 'fulfilled';
      settle();
    };
    const reject = (reason) => {
      if (this.state !== 'pending') {
        throw 'Promise already settled!';
      }
      // when rejected, set reason and transition to <rejected>
      this.reason = reason;
      this.state = 'rejected';
      settle();
    };

    executor(resolve, reject);
  }

  /**
   * @param {onFulfilled} [onFulfilled]
   * @param {onRejected} [onRejected]
   * @returns {XPromise}
   */
  then(onFulfilled, onRejected) {
    // A promise must provide a `then` method to
    // access its current or eventual value or reason.
    // `then` method accepts two arguments, both are optional,
    // and must be ignored if not functions
    if (typeof onFulfilled !== 'function') {
      onFulfilled = null;
    }
    if (typeof onRejected !== 'function') {
      onRejected = null;
    }
    // `then` should return a new promise
    // build executor for the new promise
    const promise = new XPromise((resolve, reject) => {
      const onSettled = () => {
        const prp = (x) => {
          if (promise === x) {
            reject(new TypeError);
            return;
          } else if (x instanceof XPromise) {
            if (x.state === 'pending') {
              x.thens.push(onSettled);
              return;
            } else if (x.state === 'fulfilled') {
              resolve(x.value);
              return;
            } else if (x.state === 'rejected') {
              reject(x.reason);
              return;
            }
          } else if (typeof x === 'object' || typeof x === 'function') {
            let then;
            try {
              then = x.then;
            } catch (e) {
              reject(e);
              return;
            }
            if (typeof then === 'function') {
              const settlerCalled = false;
              const resolvePromise = (y) => {
                if (settlerCalled) {
                  return;
                }
                settlerCalled = true;
                prp(y);
              };
              const rejectPromise = (r) => {
                if (settlerCalled) {
                  return;
                }
                settlerCalled = true;
                reject(r);
              };
              try {
                then.call(x, resolvePromise, rejectPromise);
              } catch (e) {
                if (!settlerCalled) {
                  reject(e);
                }
              }
              return;
            }
          } else {
            resolve(x);
          }
        };
        if (this.state === 'fulfilled') {
          if (onFulfilled) {
            try {
              prp(onFulfilled(this.value));
            } catch (e) {
              reject(e);
            }
          } else {
            resolve(this.value);
          }
        } else if (this.state === 'rejected') {
          if (onRejected) {
            try {
              prp(onRejected(this.reason));
            } catch (e) {
              reject(e);
            }
          } else {
            reject(this.reason);
          }
        } else {
          throw 'Unexpected call on onSettled callback!';
        }
      };
      // push to thens, waiting for call
      this.thens.push(onSettled);
      // if already settled, call immediately
      if (this.state !== 'pending') {
        setImmediate(onSettled);
      }
    });
    return promise;
  }

}

module.exports = XPromise;
