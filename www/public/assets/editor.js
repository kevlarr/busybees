/**
 * Configuration for the embedded Summernote editor and listeners
 * to enable and disable the submit button
 */
(function() {

  const submitButton = document.getElementById('SubmitEditor');
  const postTitle = document.getElementById('PostTitle');

  let uploading = 0;
  let textDisplay;
  let statusBar;

  function visibleText() {
      return (textDisplay && textDisplay.innerText.trim()) || '';
  }

  function setSubmitState() {
    // TODO This might be too much for every keypress...
    // There will always be markup in the base <textarea> so need to look in the visible display <div>
    !uploading && visibleText() && postTitle.value ?
      submitButton.removeAttribute('disabled') :
      submitButton.setAttribute('disabled', 'true');
  }

  $('#SummernoteEditor').summernote({
    toolbar: [
      ['style', ['style']],
      ['font', ['forecolor', 'backcolor', 'bold', 'italic', 'underline', 'strikethrough', 'superscript', 'subscript']],
      ['paragraph', ['ol', 'ul', 'paragraph']],
      ['insert', ['picture', 'link', 'video', 'table', 'hr']],
      ['misc', ['fullscreen', 'codeview', 'undo', 'redo', 'help']],
    ],

    placeholder: 'First step: Lie about having stopped it...',

    dialogsInBody: true,

    styleTags: ['h2', 'h3', 'h4', 'p', 'blockquote', 'code'],

    callbacks: {
      onChange(contents, $editable) {
        setSubmitState();
      },

      onImageUpload(files) {
        submitButton.setAttribute('disabled', 'true');
        uploading++;

        const data = new FormData()

        for (const file of files) {
          data.append('files', file, file.name);
        }

        let filenames = Array.from(files).map(f => f.name).join(', ');
        let uploadingAlert = document.createElement('div');

        uploadingAlert.classList.add('alert', 'alert-primary');
        uploadingAlert.innerHTML = `<progress value=1 max=2></progress> Uploading ${filenames}`;

        statusBar.appendChild(uploadingAlert);

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
            let errorAlert = document.createElement('div');

            errorAlert.classList.add('alert', 'alert-danger');
            errorAlert.innerHTML = `There was an err: ${e}... tell Kevin to fix his shit`;

            statusBar.appendChild(errorAlert);
          })
          .finally(() => {
            uploadingAlert.remove();
            uploading--;

            setSubmitState();
          });
      },
    },
  });

  textDisplay = document.getElementsByClassName('note-editable')[0];
  statusBar = document.getElementsByClassName('note-status-output')[0];

  postTitle.addEventListener('input', function(evt) {
    !uploading && evt.target.value && visibleText() ?
      submitButton.removeAttribute('disabled') :
      submitButton.setAttribute('disabled', 'true');
  });
})();
