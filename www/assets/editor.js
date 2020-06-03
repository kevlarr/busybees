/**
 * Configuration for the embedded Summernote editor and listeners
 * to enable and disable the submit button
 */
const API = (function() {
  function request(method, url, { expect = 200, headers, body }) {
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
  }

  return {
    get(url, { expect, headers }) {
      return request('GET', url, { expect, headers });
    },
    post(url, { expect, headers, body }) {
      return request('POST', url, { expect, headers, body });
    },
    patch(url, { expect, headers, body }) {
      return request('PATCH', url, { expect, headers, body });
    },
  };
})();

(function() {
  const SAVED = 'Saved';
  const UNSAVED = 'Unsaved';
  const SAVING = 'Saving...';
 
  const RGX = new RegExp(`(${window.location.host}/uploads/)(.+)`);

  const postKey = document.getElementById('EditorForm').getAttribute('data-post-key');
  const postTitle = document.getElementById('PostTitle');
  const postContent = document.getElementById('SummernoteEditor');
  const saveStatus = document.getElementById('saveStatus');

  let textDisplay;
  let statusBar;

  let uploading = 0;

  function showError({received: { code, reason, text }}) {
    let errorAlert = document.createElement('div');
    let msg = String(code);

    if (reason) { msg += ` ${reason}`; }
    if (text) { msg += `: ${text}`; }

    errorAlert.classList.add('alert', 'alert-danger');
    errorAlert.innerHTML = msg;

    statusBar.appendChild(errorAlert);
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
        scheduleSave();
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

        API.post(`/api/posts/${postKey}/images/new`, { headers: {}, body: data })
          .then(resp => resp.json())
          .then(json => {
            json.srcpaths.forEach(path => {
              console.log(`[onImageUpload::insertImage] PATH: ${path}`);
              $(this).summernote('insertImage', `/${path}`)
            });
          })
          .catch(showError)
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

    const linkedUploads = [];

    document.querySelectorAll('.note-editable img').forEach(img => {
      const matches = RGX.exec(img.src);

      if (matches) {
        const filename = matches[2];

        console.log(`[save::linkedUpload] MATCHED FILENAME: ${filename}`);
        linkedUploads.push(filename);
      }
    });

    API.patch(`/api/posts/${postKey}`, {
      expect: 204,
      body: {
        post: {
          title: postTitle.value,
          content: postContent.value,
        },
        linkedUploads,
      },
    })
      .then(() => {
        saveStatus.innerText = SAVED;
      })
      .catch(err => {
        saveStatus.innerText = UNSAVED;
        window.alert(JSON.stringify(err));
      });
  }

  postTitle.addEventListener('input', scheduleSave);
})();
