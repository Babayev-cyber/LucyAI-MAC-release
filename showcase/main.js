'use strict';
// Standalone showcase: this file makes no network requests.
const button = document.querySelector('#hello');
const message = document.querySelector('#message');

function greeting(name = 'Lucy') {
  return 'Hello from ' + name + '!';
}

button.addEventListener('click', () => {
  message.textContent = greeting();
});
