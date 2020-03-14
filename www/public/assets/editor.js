$('#editor').summernote({
  focus: true,

  toolbar: [
    ['style', ['style']],
    ['font', [
      'forecolor',
      'backcolor',
      'bold',
      'italic',
      'underline',
      'strikethrough',
      'superscript',
      'subscript',
    ]],
    ['paragraph', [
      'ol',
      'ul',
      'paragraph',
    ]],
    ['insert', [
      'picture',
      'link',
      'video',
      'table',
      'hr',
    ]],
    ['misc', [
      'fullscreen',
      'codeview',
      'undo',
      'redo',
      'help',
    ]],
  ],

  callbacks: {
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
          json.filepaths.forEach(f => $(this).summernote('insertImage', `/${f}`));
        })
        .catch(e => {
          alert(`there was an err: ${e}... tell Kevin to fix his shit`);
        });
    },
  },
});
