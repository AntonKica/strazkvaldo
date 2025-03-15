# strazkvaldo-ui

## setup lokálne prostredie

| softvér | verzia |
| ------- | ------ |
| nginx | 1.26 |

### NGINX

#### create __`/etc/nginx/sites-available/strazkvaldo.local.conf`__
```nginx
server {
  listen 127.0.0.1;
  listen [::1];

  server_name strazkvaldo.local;
  root /var/www/strazkvaldo.local;

  location /ui {
    index index.html;
    try_files $uri $uri/ =404;
  }

  location /ui {
    return 301 $scheme://$server_name/ui/;
  }
}
```
#### symlink __`/etc/nginx/sites-enabled/strazkvaldo.local.conf`__
```bash
ln -s /etc/nginx/sites-available/strazkvaldo.local.conf /etc/nginx/sites-enabled/stazkvaldo.local.conf
```

#### create root folder for webserver
```bash
mkdir -p /var/www/strazkvaldo.local/ui
```

#### reload NGINX configuration
```bash
nginx -s reload
```

#### add to __`/etc/hosts`__
```
127.0.0.1 strazkvaldo.local
::1 strazkvaldo.local
```
