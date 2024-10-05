#!/bin/sh
# kratos-entrypoint.sh

# Run migrations
kratos migrate sql -e --yes -c /etc/config/kratos/config.yaml

# If migration was successful, start the Kratos server
if [ $? -eq 0 ]; then
  echo "Migration successful, starting Kratos server..."
  kratos serve -c /etc/config/kratos/config.yaml --dev --watch-courier
else
  echo "Migration failed, exiting..."
  exit 1
fi
