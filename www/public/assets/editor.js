$('#editor').summernote({
  focus: true,

  callbacks: {
    onImageUpload(files) {
      const data = new FormData()

      for (const file of files) {
        data.append('files', file, file.name);
      }

      resp = fetch('/api/upload/image', {
        method: 'POST',
        body: data
      })
        .then(resp => {
          // FIXME resp status
          return resp.json()
        })
        .then(json => {
          debugger;

          $(this).summernote(
            'insertImage',
            'https://media1.tenor.com/images/a1dcc06e23e05ee5b47b992bd0fbd62e/tenor.gif'
          );
        })
        .catch(e => {
          debugger;
        });
    },
  },
});
