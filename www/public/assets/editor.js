(function() {
  const submitButton = document.getElementById('SubmitEditor');
  const cancelButton = document.getElementById('CancelEditor');
  let textDisplay;

  cancelButton.addEventListener('click', function() {
    window.location.pathname = '';
  });

  $('#SummernoteEditor').summernote({
    focus: true,

    toolbar: [
      ['style', ['style']],
      ['font', ['forecolor', 'backcolor', 'bold', 'italic', 'underline', 'strikethrough', 'superscript', 'subscript']],
      ['paragraph', ['ol', 'ul', 'paragraph']],
      ['insert', ['picture', 'link', 'video', 'table', 'hr']],
      ['misc', ['fullscreen', 'codeview', 'undo', 'redo', 'help']],
    ],

    callbacks: {
      /**
       * Observes change event to determine whether `submit` button
       * should be enabled or disabled.
       */
      onChange(contents, $editable) {
        // There will always be markup in the base <textarea>
        // so need to look in the visible display <div>
        const visibleText = textDisplay.innerText.trim();

        visibleText ?
          submitButton.removeAttribute('disabled') :
          submitButton.setAttribute('disabled', 'true');
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
})();
