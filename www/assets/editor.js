/**
 * Configuration for the embedded Summernote editor and listeners
 * to enable and disable the submit button
 */
(function() {
  const RGX = new RegExp(`(${window.location.host}/uploads/)(.+)`);

  /**
   * Singleton utility object to ease the experience with `fetch`,
   * primarily by setting headers and rejecting the response
   * if the status code does not match expectation.
   */
  const API = new (class {
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
    }

    get(url, { expect, headers }) {
      return this.request('GET', url, { expect, headers });
    }

    post(url, { expect, headers, body }) {
      return this.request('POST', url, { expect, headers, body });
    }

    patch(url, { expect, headers, body }) {
      return this.request('PATCH', url, { expect, headers, body });
    }
  })();

  /**
   * Singleton object to encapsulate interacting with API and DOM
   */
  const PAGE = new (class Page {
    constructor() {
      this.autosaveTimeout = null;
      this.form = document.getElementById('editor-form');
      this.postImages = document.getElementById('post-images');
      this.saveStatus = document.getElementById('save-status');
      this.uploading = 0;

      // Need to use jquery for the editor node
      this.editor = $('#summernote-editor');
      this.editor.summernote({
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
          onChange: (/* contents, $editable */) => this.scheduleSave(),

          onImageUpload: files => this.uploadImages(files),
        },
      });

      // These won't exist until after initializing Summernote editor
      this.statusBar = document.getElementsByClassName('note-status-output')[0];
      this.textDisplay = document.getElementsByClassName('note-editable')[0];

      this.postTitle.addEventListener('input', _evt => this.scheduleSave());
    }

    get postTitle() {
      return this.form['post-title'];
    }

    get postKey() {
      return this.form.getAttribute('data-post-key');
    }

    markUnsaved() {
      this.setSaveStatus('Unsaved');
    }

    markSaving() {
      this.setSaveStatus('Saving...');
    }

    markSaved() {
      this.setSaveStatus('Saved');
    }

    setSaveStatus(status) {
      this.saveStatus.innerText = status;
    }

    createImagesList() {
      let ul = document.createElement('ul');

      ul.id = 'post-images-list';
      this.postImages.appendChild(ul);

      return ul
    }

    fetchImages() {
      API.get(`/api/posts/${this.postKey}/images`, { expect: 200 })
        .then(resp => resp.json())
        .then(this.setImages.bind(this))
        .catch(err => {
          this.postImages.innerHTML = null;
          this.showError(this.postImages)(err);
        });
    }

    setImages({ images }) {
      this.postImages.innerHTML = null;

      if (!images || !images.length) {
        this.postImages.innerText =
          'Upload images to select a preview image for the post';

        return;
      }

      let ul = this.createImagesList();
      images.forEach(image => this.appendImage(ul, image));
    }

    appendImage(parentNode, image) {
      let img = document.createElement('img');
      img.className = 'post-image';
      img.src = `/uploads/${image.thumbnailFilename || image.filename}`;
      img.addEventListener('click', _evt => this.scheduleSave(50));

      let label = document.createElement('label');
      label.htmlFor = `image-${image.imageId}`;
      label.appendChild(img);

      let input = document.createElement('input');
      input.type = 'radio';
      input.name = 'previewImageId';
      input.id = `image-${image.imageId}`;
      input.value = image.imageId;
      input.hidden = true;

      let li = document.createElement('li');
      li.appendChild(input);
      li.appendChild(label);

      parentNode.appendChild(li);
    }

    showError(parentNode) {
      //function buildAlert({received: { code, reason, text }}) {
      function buildAlert(err) {
        let {received: { code, reason, text }} = err;
        let errorAlert = document.createElement('div');
        let msg = String(code);

        if (reason) { msg += ` ${reason}`; }
        if (text) { msg += `: ${text}`; }

        errorAlert.classList.add('alert', 'alert-danger');
        errorAlert.innerHTML = msg;

        parentNode.appendChild(alert);
      }

      return buildAlert
    }

    scheduleSave(timeout = null) {
      if (this.autosaveTimeout) {
        window.clearTimeout(this.autosaveTimeout);
      }

      this.markUnsaved();

      if (this.uploading > 0 || !this.postTitle.value) {
        return;
      }

      timeout = typeof timeout == 'number' ? timeout : 500;
      this.autosaveTimeout = window.setTimeout(this.save.bind(this), timeout);
    }

    extractUploaded() {
      const linkedUploads = [];

      document.querySelectorAll('.note-editable img').forEach(img => {
        const matches = RGX.exec(img.src);

        if (matches) {
          const filename = matches[2];

          console.log(`[save::linkedUpload] MATCHED FILENAME: ${filename}`);
          linkedUploads.push(filename);
        }
      });

      return linkedUploads;
    }

    save() {
      this.markSaving();

      const linkedUploads = this.extractUploaded();

      let previewImageRadio = document.querySelector('input[name=previewImageId]:checked');
      let previewImageId = previewImageRadio ?
        Number(previewImageRadio.value) :
        null;

      API.patch(`/api/posts/${this.postKey}`, {
        expect: 204,
        body: {
          post: {
            title: this.postTitle.value,
            content: this.form['summernote-editor'].value,
          },
          previewImageId,
          linkedUploads,
        },
      })
        .then(() => {
          this.postImages.innerHTML = null;
          return this.fetchImages();
        })
        .then(() => this.markSaved())
        .catch(err => {
          this.markUnsaved();
          window.alert(JSON.stringify(err));
        });
    }

    uploadImages(files) {
      this.statusBar.innerHTML = null;
      this.markUnsaved();

      const data = new FormData()

      for (const file of files) {
        data.append('files', file, file.name);
        this.uploading++;
      }

      let filenames = Array.from(files).map(f => f.name).join(', ');
      let uploadingAlert = document.createElement('div');

      uploadingAlert.classList.add('alert', 'alert-primary');
      uploadingAlert.innerHTML = `<progress value=1 max=2></progress> Uploading ${filenames}`;

      this.statusBar.appendChild(uploadingAlert);

      API.post(`/api/posts/${this.postKey}/images/new`, { headers: {}, body: data })
        .then(resp => resp.json())
        .then(json => this.addNewImages(json))
        .catch(this.showError(this.statusBar))
        .finally(() => uploadingAlert.remove());
    }

    addNewImages({ images }) {
      let imagesList = document.getElementById('post-images-list');

      if (!imagesList) {
        // Clear content in case there was an error message present
        this.postImages.innerHTML = null;
        imagesList = this.createImagesList();
      }

      images.forEach(image => {
        this.editor.summernote('insertImage', `/uploads/${image.filename}`)
        this.appendImage(imagesList, image);
        this.uploading--;
      });
    }
  })();

  PAGE.fetchImages();
})();
