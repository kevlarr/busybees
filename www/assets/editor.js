/**
 * Configuration for the Summernote editor, autosave, etc.
 */
(function() {
  const uploaded_img_rgx = new RegExp(`(${window.location.host}/uploads/)(.+)`);

  const page = new (class Page {
    constructor() {
      this.autosaveTimeout = null;
      this.uploading = 0;

      this.form = document.getElementById('editorForm');
      this.postImages = document.getElementById('postImages');

      this.saveStatusSpinner = document.querySelector('#save-status .spinner');
      this.saveStatusText = document.getElementById('save-status-text');

      // Need to use jquery for the editor node
      this.editor = $('#summernoteEditor');
      this.editor.summernote({
        toolbar: [
          ['style', ['style']],
          ['font', ['forecolor', 'backcolor', 'bold', 'italic', 'underline', 'strikethrough', 'superscript', 'subscript']],
          ['paragraph', ['ol', 'ul', 'paragraph']],
          ['insert', ['picture', 'link', 'video', 'table', 'hr']],
          ['misc', ['fullscreen', 'codeview', 'undo', 'redo', 'help']],
        ],

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
      this.postPublishedDate.addEventListener('input', _evt => this.scheduleSave());
    }

    get postTitle() {
      return this.form.postTitle;
    }

    get postPublishedDate() {
      return this.form.postPublishedDate;
    }

    get postKey() {
      return this.form.getAttribute('data-post-key');
    }

    markUnsaved() {
      this.showSpinner(false);
      this.setSaveStatus('Unsaved');
    }

    markSaving() {
      this.showSpinner(true);
      this.setSaveStatus('Saving');
    }

    markSaved() {
      this.showSpinner(false);
      this.setSaveStatus('Saved');
    }

    showSpinner(bool) {
      this.saveStatusSpinner.style.display = bool ? "initial" : "none";
    }

    setSaveStatus(status) {
      this.saveStatusText.innerText = status;
    }

    createImagesList() {
      let ul = HTML.Ul({ id: 'postImagesList' });
      this.postImages.append(ul);
      return ul;
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
        this.postImages.innerText = 'No images have been uploaded.';

        return;
      }

      let ul = this.createImagesList();
      images.forEach(image => this.appendImage(ul, image));
    }

    appendImage(parentNode, image) {
      parentNode.append(
        HTML.Li(null, [
          HTML.Input({
            hidden: true,
            id: `image-${image.imageId}`,
            name: 'previewImageId',
            type: 'radio',
            value: image.imageId,
          }),
          HTML.Label({
            htmlFor: `image-${image.imageId}`,
          }, [
            HTML.Img({
              className: image.isPreview ? 'post-image is-preview' : 'post-image',
              src: `/uploads/${image.thumbnailFilename || image.filename}`,
              events: {
                click: (/* event */) => this.scheduleSave(50),
              },
            }),
          ]),
        ])
      );
    }

    showError(parentNode) {
      function buildAlert(err) {
        let {received: { code, reason, text }} = err;
        let msg = String(code);

        if (reason) { msg += ` ${reason}`; }
        if (text) { msg += `: ${text}`; }

        parentNode.append(HTML.Div({
          className: 'alert alert-danger',
        }, [
          msg,
        ]));
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
        const matches = uploaded_img_rgx.exec(img.src);

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

      // As weird as this is, the endpoint is so fast that there's no
      // real feedback (ie. the loading spinner doesn't really shows up)
      // so add a delay after marking it as saving to make it obvious on
      // the page that something is happening.
      window.setTimeout(() => {
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
              content: this.form.summernoteEditor.value,
              published_at_date: this.postPublishedDate.value || null,
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
      }, 1000);
    }

    uploadImages(files) {
      this.statusBar.innerHTML = null;
      this.markUnsaved();

      const data = new FormData()

      for (const file of files) {
        data.append('files', file, file.name);
        this.uploading++;
      }

      let alert = HTML.Div({ className: 'alert alert-primary' }, [
        HTML.Progress({max: 2, value: 1}),
        ` Uploading ${files.length} file(s)`,
      ]);

      this.statusBar.append(alert);

      API.post(`/api/posts/${this.postKey}/images/new`, { headers: {}, body: data })
        .then(resp => resp.json())
        .then(json => this.addNewImages(json))
        .catch(this.showError(this.statusBar))
        .finally(() => alert.remove());
    }

    addNewImages({ images }) {
      let imagesList = document.getElementById('postImagesList');

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

  page.fetchImages();
})();
