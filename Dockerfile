FROM nginx

# assumes wasm component already built on host machine

COPY ./*.html /usr/share/nginx/html/
COPY ./*.css /usr/share/nginx/html/
COPY ./*.js /usr/share/nginx/html/
COPY ./pkg /usr/share/nginx/html/pkg

