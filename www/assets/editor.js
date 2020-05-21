/**
 * Configuration for the embedded Summernote editor and listeners
 * to enable and disable the submit button
 */
(function() {
  const SAVED = 'Saved';
  const UNSAVED = 'Unsaved';
  const SAVING = 'Saving...';

  const postKey = document.getElementById('EditorForm').getAttribute('data-post-key');
  const postTitle = document.getElementById('PostTitle');
  const postContent = document.getElementById('SummernoteEditor');
  const saveStatus = document.getElementById('saveStatus');

  let textDisplay;
  let statusBar;

  let uploading = 0;

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
        scheduleSave();
      },

      onImageLinkInsert(url) {
        console.log(`[onImageLinkInsert::insertImage] ${url}`);

        if (url.startsWith(`https://${window.location.host}/uploads`)) {
          console.log('TODO: POST');
        }

        $(this).summernote('insertImage', url);
      },

      onImageUpload(files) {
        saveStatus.innerText = UNSAVED;
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

        fetch(`/api/posts/${postKey}/images/new`, { method: 'POST', body: data })
          .then(resp => {
            if (resp.status >= 400) {
              throw new Error(JSON.stringify(resp.statusText));
            }
            return resp.json()
          })
          .then(json => {
            json.srcpaths.forEach(path => {
              console.log(`[onImageUpload::insertImage] ${path}`);
              $(this).summernote('insertImage', `/${path}`)
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
          });
      },
    },
  });

  textDisplay = document.getElementsByClassName('note-editable')[0];
  statusBar = document.getElementsByClassName('note-status-output')[0];

  let autosaveTimeout;

  function scheduleSave(evt) {
    if (autosaveTimeout) {
      window.clearTimeout(autosaveTimeout);
    }

    saveStatus.innerText = UNSAVED;

    if (uploading > 0) {
      return;
    }

    autosaveTimeout = window.setTimeout(save, 2500);
  }

  function save() {
    saveStatus.innerText = SAVING;

    fetch(`/api/posts/${postKey}`, {
      method: 'PATCH',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        title: postTitle.value,
        content: postContent.value,
      }),
    })
      .then(resp => {
        if (resp.status >= 400) {
          saveStatus.innerText = UNSAVED;
          throw new Error(JSON.stringify(resp.statusText));
        }

        saveStatus.innerText = SAVED;
      });
  }

  postTitle.addEventListener('input', scheduleSave);
})();
