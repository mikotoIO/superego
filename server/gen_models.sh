sea-orm-cli migrate -u "sqlite://temp.sqlite?mode=rwc"
sea-orm-cli generate entity -u "sqlite://temp.sqlite?mode=rwc" -o ./src/entities --with-serde both
rm ./temp.sqlite