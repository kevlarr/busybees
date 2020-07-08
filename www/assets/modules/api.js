/**
 * Singleton utility object to ease the experience with `fetch`,
 * primarily by setting headers and rejecting the response
 * if the status code does not match expectation.
 */
const API = {
  request(method, url, { expect = 200, headers, body }) {
    if (headers === undefined) {
      headers = {'Content-Type': 'application/json'};
    }

    if (headers['Content-Type'] === 'application/json') {
      body = JSON.stringify(body);
    }

    let status, statusText;

    return fetch(url, { method, headers, body })
      .then(resp => new Promise((resolve, reject) => {
        if (resp.status !== expect) {
          status = resp.status;
          statusText = resp.statusText;

          return resp.text().then(text => reject(text))
        }

        return resolve(resp);
      }))
      .catch(text => new Promise((_, reject) => {
        return reject({
            expected: expect,
            received: {
              code: status,
              reason: statusText,
              text,
            },
        });
      }))
  },

  get(url, { expect, headers }) {
    return this.request('GET', url, { expect, headers });
  },

  post(url, { expect, headers, body }) {
    return this.request('POST', url, { expect, headers, body });
  },

  patch(url, { expect, headers, body }) {
    return this.request('PATCH', url, { expect, headers, body });
  },
};
