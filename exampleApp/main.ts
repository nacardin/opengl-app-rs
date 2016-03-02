import {Blaze} from 'blazegl'

Blaze.createWindow({
  title: 'Example App'
}).then(function (appWindow) {
  appWindow.load('components/app')
});
