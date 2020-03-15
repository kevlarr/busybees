function toBase36(byte) {
  return ('0' + byte.toString(36)).slice(-2);
}

function generateId(len = 12) {
  const arr = new Uint8Array(len / 2);
  window.crypto.getRandomValues(arr);
  return Array.from(arr, toBase36).join("");
}

(function() {
  const submitButton = document.getElementById('SubmitEditor');
  const cancelButton = document.getElementById('CancelEditor');

  const postTitle = document.getElementById('PostTitle');
  const postAlphaId = document.getElementById('PostAlpha');
  postAlphaId.value = generateId();

  let textDisplay;
  let timeout;

  function visibleText() {
      return (textDisplay && textDisplay.innerText.trim()) || '';
  }

  $('#SummernoteEditor').summernote({
    focus: true,

    toolbar: [
      ['style', ['style']],
      ['font', ['forecolor', 'backcolor', 'bold', 'italic', 'underline', 'strikethrough', 'superscript', 'subscript']],
      ['paragraph', ['ol', 'ul', 'paragraph']],
      ['insert', ['picture', 'link', 'video', 'table', 'hr']],
      ['misc', ['fullscreen', 'codeview', 'undo', 'redo', 'help']],
    ],

    styleTags: ['h2', 'h3', 'h4', 'p', 'blockquote', 'code'],

    callbacks: {
      /**
       * Observes change event to determine whether `submit` button
       * should be enabled or disabled.
       */
      onChange(contents, $editable) {
        // There will always be markup in the base <textarea> so need to look in the visible display <div>
        const text = visibleText();

        if (text) {
          submitButton.removeAttribute('disabled');
        } else {
          postTitle.value = '';
          submitButton.setAttribute('disabled', 'true');
        }

        if (timeout) {
          window.clearTimeout(timeout);
        }

        if (!text) {
          return;
        }

        timeout = window.setTimeout(function() {
          const title = text
            .substring(0, 64)
            .replace(/[^a-z0-9\s]*/gi, "")
            .replace(/\s{2,}/g, " ")
            .trim()
            .split(" ")
            .slice(0, 12)
            .join('-');

          postTitle.value = encodeURIComponent(title);
        }, 500);
      },

      /**
       * Posts file(s) to server and inserts image nodes with returned URL(s).
       */
      onImageUpload(files) {
        const data = new FormData()

        for (const file of files) {
          data.append('files', file, file.name);
        }

        resp = fetch('/images', { method: 'POST', body: data })
          .then(resp => {
            if (resp.status >= 400) {
              throw new Error(JSON.stringify(resp.statusText));
            }
            return resp.json()
          })
          .then(json => {
            json.filepaths.forEach(f => {
              $(this).summernote('insertImage', `/${f}`)
            });
          })
          .catch(e => {
            alert(`there was an err: ${e}... tell Kevin to fix his shit`);
          });
      },
    },
  });

  textDisplay = document.getElementsByClassName('note-editable')[0];

  cancelButton.addEventListener('click', function() {
    if (timeout) {
      window.clearTimeout(timeout);
    }
    window.location.pathname = '';
  });
})();
