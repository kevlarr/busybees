$('#editor').summernote({
  focus: true,

  callbacks: {
    onImageUpload(files) {
      const data = new FormData()

      for (const file of files) {
        data.append('files', file, file.name);
      }

      resp = fetch('/images', {
        method: 'POST',
        body: data
      })
        .then(resp => {
          if (resp.status >= 400) {
            throw new Error(JSON.stringify(resp));
          }
          return resp.json()
        })
        .then(json => {
          //debugger;

          json.filepaths.forEach(f => $(this).summernote('insertImage', `/${f}`));
        })
        .catch(e => {
          debugger;
        });
    },
  },
});
