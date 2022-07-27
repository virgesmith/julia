FROM nginx

COPY ./*.html /usr/share/nginx/html/
COPY ./*.js /usr/share/nginx/html/
COPY ./pkg /usr/share/nginx/html/pkg

