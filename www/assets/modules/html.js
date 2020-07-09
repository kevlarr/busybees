/**
 * Constructors to make creating DOM nodes more ergonomic.
 */
const HTML = {
  _create(tagName, props = null, children = null) {
    let element = document.createElement(tagName);

    if (props) {
      Object.entries(props).forEach(([key, val]) => {
        if (key === 'events') {
          Object.entries(val).forEach(([eventName, callback]) => {
            element.addEventListener(eventName, callback);
          });
        } else {
          element[key] = val;
        }
      });
    }

    if (children) {
      children.forEach(c => element.append(c));
    }

    return element;
  },

  Div      (...args) { return this._create('div',      ...args); },
  Img      (...args) { return this._create('img',      ...args); },
  Input    (...args) { return this._create('input',    ...args); },
  Label    (...args) { return this._create('label',    ...args); },
  Li       (...args) { return this._create('li',       ...args); },
  Progress (...args) { return this._create('progress', ...args); },
  Ul       (...args) { return this._create('ul',       ...args); },
};
