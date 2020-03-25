(function controlSubmitState() {
  let email = document.getElementById('SignInEmail');
  let password = document.getElementById('SignInPassword');
  let submit = document.getElementById('SignInSubmit');

  function setSubmitState() {
    email.value && password.value ?
      submit.removeAttribute('disabled') :
      submit.setAttribute('disabled', true);
  }

  email.addEventListener('input', setSubmitState);
  password.addEventListener('input', setSubmitState);
})();

(function animateWelcome() {
  let welcome = document.getElementById('SignInWelcome');
  let cursor = " <box-cursor></box-cursor>";
  let position = 2;

  window.setTimeout(function updateWelcome() {
    welcome.innerHTML = 'Welcome'.slice(0, position) + cursor;
    position++;

    if (position < 8) {
      window.setTimeout(updateWelcome, 100);
    }
  }, 100);
})();
