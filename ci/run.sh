cd site
bundle install
bundle exec scripts/get-data.rb
bundle exec jekyll build
cd ..
curl -O http://geolite.maxmind.com/download/geoip/database/GeoLite2-City.tar.gz
tar xvf GeoLite2-City.tar.gz --strip-components=1 GeoLite2-City_*/GeoLite2-City.mmdb
