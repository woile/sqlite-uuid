# sqlite-uuid

Add support for UUIDv4 and UUIDv7 to SQLite

> [!CAUTION]
> This is an experimental plugin

## Installation

```sh
wget -qO- "https://github.com/woile/sqlite-uuid/releases/download/0.4.0/libsqlite_uuid-$(uname -s)-$(uname -m).tar.gz" | tar xvz
```

## Usage

```sql
.load libsqlite_uuid

--- Let's try uuid_blob which produces a blob (smaller than text)
CREATE TABLE events(
  event_id uuid_blob primary key,
  name TEXT
);

INSERT INTO events(event_id, name) VALUES (uuid_blob(), 'up');
INSERT INTO events(event_id, name) VALUES (uuid_v7_blob(), 'down');
INSERT INTO events(event_id, name) VALUES (uuid_as_blob('018d9887-42cd-7115-b1ca-18227ac211b4'), 'down');

SELECT uuid_from_blob(event_id), name
FROM events
WHERE event_id = uuid_as_blob('018d9887-42cd-7115-b1ca-18227ac211b4');

--- Let's use uuid which produces strings

CREATE TABLE events_as_str(
  event_id uuid primary key,
  name TEXT
);

INSERT INTO events_as_str(event_id, name) VALUES (uuid(), 'up');
INSERT INTO events_as_str(event_id, name) VALUES (uuid(7), 'down');

SELECT event_id name FROM events_as_str;
```

## Functions

```sql
--- create a UUIDv4 as BLOB
uuid_blob()

--- create a UUIDv7 as BLOB (recommended)
uuid_v7_blob()

--- create a UUIDv4 as TEXT
uuid()

--- create a UUIDv7 as TEXT
uuid_v7()

--- convert TEXT UUID to BLOB
uuid_as_blob('018d9887-42cd-7115-b1ca-18227ac211b4')

--- convert UUID BLOB to TEXT
uuid_from_blob(column_name)
```

Thanks to the amazing work by [asg017](https://github.com/asg017) who built an amazing [sqlite-ecosystem](https://github.com/asg017/sqlite-ecosystem)
