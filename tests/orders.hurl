# Login and capture session-token
POST http://{{host}}:{{port}}/api/login
Content-Type: application/json

{
  "auth_date": 1711117804,
  "first_name": "Maxud",
  "hash": "964b995230e8e2ef33b949380ef703ffee133eaefe58ebb726b12180dc21498a",
  "id": 443621429,
  "last_name": "Abdulmalikov",
  "photo_url": "https://t.me/i/userpic/320/_PO3SLTElcThIH_w3felgsqSo3Dn4br5mcxugCLvjCM.jpg",
  "username": "KingMaxud"
}

HTTP 303

[Asserts]
header "Set-Cookie" contains "session-token="

[Captures]
token: cookie "session-token"

# Create a car to capture it's id
POST http://{{host}}:{{port}}/api/cars
[Cookies]
session-token: {{token}}
{
  "name": "My Awesome Car",
  "hourly_rate": 20,
  "daily_rate": 150,
  "weekly_rate": 800,
  "photos": null,
  "status": "available"
}

HTTP 200
[Captures]
car_id: jsonpath "$.id"

# Make order
POST http://{{host}}:{{port}}/api/orders
Content-Type: application/json
[Cookies]
session-token: {{token}}
{
  "car_id": "{{car_id}}"
}

HTTP 200
[Captures]
order_id: jsonpath "$.id"

# Orders history
GET http://{{host}}:{{port}}/api/orders/history
[Cookies]
session-token: {{token}}

HTTP 200

# Cancel order
PATCH http://{{host}}:{{port}}/api/orders/cancel/{{order_id}}
[Cookies]
session-token: {{token}}

HTTP 200

# Accept order
PATCH http://{{host}}:{{port}}/api/orders/accept/{{order_id}}
[Cookies]
session-token: {{token}}

HTTP 200
[Asserts]
jsonpath "$.status" == "accepted"

# Start rent
PATCH http://{{host}}:{{port}}/api/orders/start/{{order_id}}
[Cookies]
session-token: {{token}}

HTTP 200
[Asserts]
jsonpath "$.status" == "processing"

# Finish rent
PATCH http://{{host}}:{{port}}/api/orders/finish/{{order_id}}
[Cookies]
session-token: {{token}}

HTTP 200
[Asserts]
jsonpath "$.status" == "finished"

# Set paid
PATCH http://{{host}}:{{port}}/api/orders/set_paid/{{order_id}}
[Cookies]
session-token: {{token}}

HTTP 200
[Asserts]
jsonpath "$.paid" == true

# Get order
GET http://{{host}}:{{port}}/api/orders/{{order_id}}
[Cookies]
session-token: {{token}}

HTTP 200
[Asserts]
jsonpath "$.id" exists
jsonpath "$.user_id" exists
jsonpath "$.car_id" exists
jsonpath "$.start_rent_time" exists
jsonpath "$.end_rent_time" exists
jsonpath "$.paid" exists
jsonpath "$.created_at" exists
jsonpath "$.updated_at" exists

# List orders
GET http://{{host}}:{{port}}/api/orders
[Cookies]
session-token: {{token}}

HTTP 200

# Delete order
DELETE http://{{host}}:{{port}}/api/orders/{{order_id}}
[Cookies]
session-token: {{token}}

HTTP 200

# Delete car
DELETE http://{{host}}:{{port}}/api/cars/{{car_id}}

HTTP 200

# Logout
POST http://{{host}}:{{port}}/api/logout
[Cookies]
session-token: {{token}}

HTTP 303
