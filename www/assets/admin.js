(function() {
  const postStatuses = document.getElementsByTagName('post-status');

  for (const postStatus of postStatuses) {
    const key = postStatus.getAttribute('data-post-key');

    postStatus.addEventListener('click', function() {
      const published = postStatus.getAttribute('type') === 'published';
      let status;

      fetch(`/api/posts/${key}/published`, {
        method: 'PATCH',
        credentials: 'same-origin',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ published: !published }),
      })
        .then(resp => {
          status = resp.status;
          return resp.text();
        })
        .then(text => {
          if (status !== 204) {
            throw new Error(text);
          }

          postStatus.setAttribute('type', published ? 'unlisted' : 'published');
        })
        .catch(e => {
          debugger
        });
    });
  }
})();
