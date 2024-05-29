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

# Create a car
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

# Get car
GET http://{{host}}:{{port}}/api/cars/{{car_id}}
[Cookies]
session-token: {{token}}

HTTP 200
[Asserts]
jsonpath "$.id" exists
jsonpath "$.name" exists
jsonpath "$.hourly_rate" exists
jsonpath "$.daily_rate" exists
jsonpath "$.weekly_rate" exists
jsonpath "$.photos" exists
jsonpath "$.status" exists
jsonpath "$.created_at" exists

# Get car list
GET http://{{host}}:{{port}}/api/cars?status=available
[Cookies]
session-token: {{token}}

HTTP 200

# Update car
PATCH http://{{host}}:{{port}}/api/cars/{{car_id}}
[Cookies]
session-token: {{token}}
{
   "name": "Updated Awesome Car"
}

HTTP 200
[Asserts]
jsonpath "$.name" == "Updated Awesome Car"

# Delete car
DELETE http://{{host}}:{{port}}/api/cars/{{car_id}}

HTTP 200

# Logout
POST http://{{host}}:{{port}}/api/logout
[Cookies]
session-token: {{token}}

HTTP 303
