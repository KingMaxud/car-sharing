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

# Logout
POST http://{{host}}:{{port}}/api/logout

HTTP 303
