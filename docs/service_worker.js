var cacheName = "ow-pwa-v1";

var filesToCache = [
  "./",
  "./index.html",
  "./index.css",
  "./owapp.js",
  "./owapp_bg.wasm",
  "./icon-48.png",
  "./icon-72.png",
  "./icon-96.png",
  "./icon-144.png",
  "./icon-168.png",
  "./icon-192.png",
  "./icon-256.png",
];

/* Start the service worker and cache all of the app's content */
self.addEventListener("install", function (e) {
  e.waitUntil(
    caches.open(cacheName).then(function (cache) {
      return cache.addAll(filesToCache);
    })
  );
});

/* Serve cached content when offline */
self.addEventListener("fetch", function (e) {
  e.respondWith(
    caches.match(e.request).then(function (response) {
      return response || fetch(e.request);
    })
  );
});

/* Clear old cache */
self.addEventListener("activate", function (e) {
  e.waitUntil(
    caches.keys().then(function (keyList) {
      return Promise.all(
        keyList.map(function (key) {
          if (key === cacheName) {
            return;
          }
          return caches.delete(key);
        })
      );
    })
  );
});
