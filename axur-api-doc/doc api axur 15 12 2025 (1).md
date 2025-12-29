* Introduction  
* Axur Platform API  
  * Axur Platform API  
  * Authentication  
  * Permission levels  
  * Tickets Api Operations  
  * Integration Feed  
  * Filter Operations  
  * Ticket Operations  
  * Axur Platform webhooks  
  * Safelist operations  
  * Users  
  * Customer/Asset  
  * API Fields Description  
  * CSV Ticket Extraction Sample  
  * Credential Search Operations  
  * Credential Update Operations  
  * Credential Count Operations  
  * Web complaints API  
* Credit Card Exposure for Application  
  * Credit Card Exposure for Application  
  * Getting started with Credit Card Exposure for Application  
  * Supported operations on Credit Card Exposure for Application  
* Credit Card Exposure for Issuers  
  * Credit Card Exposure for Issuers  
  * Getting started with Credit Card Exposure for Issuers  
  * Webhooks for Credit Card Exposure for Issuers  
  * Supported operations on Credit Card Exposure for Issuers  
* Threat & Exposure Intelligence TAXII Server  
  * Threat & Exposure Intelligence TAXII Server  
  * Supported operations on Threat & Exposure Intelligence TAXII Server  
  * IoC Collection  
* Threat Hunting  
  * Threat Hunting  
  * Supported operations on Threat Hunting  
* Investigations  
  * Investigations  
  * Supported operations on Investigations  
* Open Data  
  * Open Data  
  * Supported operations on Open Data  
* Http Feed API  
  * Http Feed API  
  * Feed  
* IoC Feed  
  * IoC Feed  
  * Supported operations on IoC Feed  
  * Changelog

[Documentation Powered by ReDoc](https://github.com/Redocly/redoc)

# Axur Platform API (1.0.50)

Download OpenAPI specification:[Download](https://docs.axur.com/en/axur/api/openapi-axur.yaml)

Last edited on August 22nd, 2025\.

# Introduction

**Axur** provides HTTP APIs that allow external scripts and applications to access and manipulate our services programmatically. This document describes the currently supported API operations within Axur Platform and Credit Card Exposure services.

If you have any questions or need help implementing specific use-cases, please contact support through help@axur.com or [https://help.axur.com/en/](https://help.axur.com/en/).

# Axur Platform API

Axur Platform provides HTTP APIs that allow external scripts and applications to access and manipulate tickets programmatically. Supported operations are described below.

# Authentication

Each API request must be authenticated using the Bearer scheme with a valid token. Authentication tokens may be obtained by API KEY:

* An API KEY can be created via the AXUR platform, in my preferences under the [API KEY tab](https://one.axur.com/preferences?tab=api-keys).

The following HTTP header is required for all authenticated requests:

```
Authorization: Bearer <token>
```

**Important:** The placeholder \<token\> in descriptions and examples that follow must be replaced by a valid authentication token.

## bearerAuth

| Security Scheme Type | HTTP Authorization Scheme | Bearer format |
| :---- | :---- | :---- |
| HTTP | bearer | "JWT" |

# Permission levels

Depending on the authenticated user's permission level, some operations may be forbidden.

**Viewers** are not allowed to perform any state-changing operations, such as adding comments or executing lifecycle actions. Read-only users may be restricted to certain assets and ticket types.

**Practitioners** have all permissions of read-only users and additionally are allowed to perform basic state-changing operations, such as creating and discarding tickets. Practitioners may be restricted to certain assets and ticket types.

**Experts** have all permissions of practitioners and additionally are allowed to perform advanced state-changing operations, such as requesting takedowns. Experts may be restricted to certain assets and ticket types.

**Customs** have their permissions assigned by Managers and can only perform the actions related to the features they are assigned to. The features Custom users are allowed to access can be seen by Managers at [My Team](https://one.axur.com/userManagement).

**Managers** have all permissions of experts and additionally are allowed to manage team users and view their activity history. Managers always have access to all assets and ticket types.

# Tickets Api Operations

## Examples of usage

Get all takedowns performed in the month of July using the Arizona (USA) time zone, with complete information of the tickets present in the response:

```
https://api.axur.com/gateway/1.0/api/tickets-api/tickets?type=phishing&takedown.request.date=ge:2024-07-01&takedown.request.date=le:2024-07-31&sortBy=takedown.request.date&page=1&pageSize=200&order=asc&timezone=-07:00
```

Get all phishing tickets opened in the month of July using the Arizona (USA) time zone with only ticket fields and attachments

```
https://api.axur.com/gateway/1.0/api/tickets-api/tickets?type=phishing&open.date=ge:2024-07-01&open.date=le:2024-07-31&sortBy=open.date&page=1&pageSize=200&order=asc&timezone=-07:00&include=fields,attachments
```

## Operators

For date or numeric fields, you can use the following operators:

* gt: \- Greater than  
* lt: \- Less than  
* ge: \- Greater than or equal  
* le: \- Less than or equal

Operators should be used as prefixes for values ​​used in query parameters, for example:

* takedown.request.date=ge:2024-07-01  
* takedown.request.date=le:2024-07-31

For text fields, you can use the following operators:

* contains: \- Contains Text

Operators should be used as prefixes for values ​​used in query parameters, for example:

* ticket.reference=contains:instagram.com

## Fields supported by filters

| Key | Type | Comments |
| :---- | :---- | :---- |
| ticket.key | string |  |
| ticket.customer | string | For user with access to multiple tenants, this parameter is required when searching for a child tenant's tickets |
| ticket.reference | string |  |
| ticket.creation.collection | integer |  |
| ticket.creation.collector | string |  |
| ticket.creation.date | date | Supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33". |
| ticket.creation.originator | string |  |
| ticket.creation.user | integer |  |
| ticket.creation.customer | string |  |
| ticket.investigation | boolean |  |
| ticket.investigation.closed.date | date | Supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33". |
| ticket.investigation.interrupted.date | date | Supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33". |
| ticket.investigation.open.date | date | Supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33". |
| ticket.investigation.reopen.date | date | Supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33". |
| ticket.last-update.date | date | Supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33". |
| ticket.reup.expires | integer |  |
| ticket.reup.state | string |  |
| ticket.tags | string |  |
| ad.product.available-stock | integer |  |
| ad.product.currency | string |  |
| ad.product.location | string |  |
| ad.product.price | decimal |  |
| ad.seller.username | string |  |
| alleged.exposed.birthdate | integer |  |
| alleged.exposed.documents | string |  |
| assets | asset |  |
| category | string |  |
| close.date | date | Supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33". |
| creation.collection | integer |  |
| creation.collector | string |  |
| creation.customer | string |  |
| creation.originator | string |  |
| creation.user | integer |  |
| credential.first-seen | integer |  |
| credential.login-source | string |  |
| credential.password.type | string |  |
| credential.password.value | string |  |
| credential.password.value.masked | string |  |
| credential.username | string |  |
| criticality | enum | \["high", "low", "medium"\] |
| document.type | string |  |
| document.value.masked | string |  |
| document.value.plain | string |  |
| documents.masked | string |  |
| documents.plain | string |  |
| documents.type | string |  |
| domain | string |  |
| domain.registrar | string |  |
| external-author-id | string |  |
| external-channel-id | string |  |
| group | string |  |
| host | string |  |
| incident.date | date | Supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33". |
| ip | string |  |
| isp | string |  |
| leak.sources | string |  |
| matched.resource | string |  |
| media.type | string |  |
| mention.uuid | string |  |
| message.group.name | string |  |
| open.date | date | Supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33". |
| platform | string |  |
| prediction.brand-logo | decimal |  |
| prediction.brand-name | decimal |  |
| prediction.has-authentication-form | boolean |  |
| prediction.languages | string |  |
| prediction.main-language | string |  |
| prediction.risk | decimal |  |
| prediction.threat-content | enum | \["EXPOSED\_CREDENTIAL", "CPF\_CONSULT", "CVE", "BOT\_MESSAGE", "BIN\_CONSULT", "UNDEFINED", "SUSPICIOUS\_MESSAGE", "FRAUD\_TOOLS", "EXPOSED\_CREDIT\_CARD", "DATA\_SALE"\] |
| profile | string |  |
| quarantine.date | date | Supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33". |
| quarantine.last.updated.date | date | Supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33". |
| quarantine.unread | boolean |  |
| resolution | string |  |
| resolution.reason | string |  |
| source.date | date | Supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33". |
| status | string | \["open","quarantine","incident","treatment","closed"\] |
| takedown | boolean |  |
| takedown.cancel.date | date | Supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33". |
| takedown.close.date | date | Supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33". |
| takedown.close.type | enum | \["auto", "manual"\] |
| takedown.first-notification.date | date | Supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33". |
| takedown.notification.date | date | Supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33". |
| takedown.notification.last.type | enum | \["auto", "manual"\] |
| takedown.request.date | date | Supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33". |
| takedown.resolution | string |  |
| takedown.resolution.reason | string |  |
| takedown.reup | boolean |  |
| takedown.submission.date | date | Supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33". |
| takedown.uptime | integer |  |
| takedown.verification.date | date | Supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33". |
| takedown.verification.last.type | enum | \["auto", "manual"\] |
| threat-content | enum | \["EXPOSED\_CREDENTIAL", "CPF\_CONSULT", "CVE", "BOT\_MESSAGE", "BIN\_CONSULT", "UNDEFINED", "SUSPICIOUS\_MESSAGE", "FRAUD\_TOOLS", "EXPOSED\_CREDIT\_CARD", "DATA\_SALE"\] |
| token.creation.date | date | Supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33". |
| treatment.date | date | Supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33". |
| treatment.internal | boolean |  |
| treatment.type | enum | \["internal", "axur"\] |
| treatment.uptime | integer |  |
| type | detection-type | \["corporate-credential-leak", "code-secret-leak", "other-sensitive-data", "database-exposure", "infostealer-credential", "fraudulent-brand-use", "phishing", "paid-search", "malware", "fake-social-media-profile", "similar-domain-name", "fake-mobile-app", "unauthorized-sale", "unauthorized-distribution", "executive-fake-social-media-profile", "executive-personalinfo-leak", "executive-credential-leak", "executive-mobile-phone", "executive-card-leak", "dw-activity", "data-exposure-message", "data-sale-message", "fraud-tool-scheme-message", "suspicious-activity-message", "infrastructure-exposure", "data-exposure-website", "data-sale-website", "fraud-tool-scheme-website", "suspicious-activity-website", "ransomware-attack"\] |

## CSV Ticket Extraction Example

Simple example of extracting tickets to a csv file. Standard authentication is used and with date filters initially set to two hours ago.

* Remember that the elements in the tickets\_fields array are just examples. Feel free to change them to whatever fields suit your needs.

**Python**

This example uses Python 3.7 as programming language.

```
import math
import requests
import json
import datetime
import csv

API_KEY = "<API_KEY>"

ticket_fields = ['ticketKey', 'reference', 'type', 'assets', 'open.date',
                'status', 'resolution', 'resolution.reason', 'close.date']

_TICKET_FILTERS_QUERY = "open.date=ge:{}&open.date=le:{}&sortBy={}&page={}&pageSize={}&order=asc&utc=-03:00"
_TICKETS_API = "tickets-api/tickets?"

CURRENT_TIME = datetime.datetime.now()
ONE_HOUR_AGO = CURRENT_TIME - datetime.timedelta(seconds=3600)
SORT_FIELD = "open.date"
PAGES = 1
TICKET_PER_PAGE = 50
FILE_NAME = 'ticket_sheet'

def endpoint_get(endpoint: str, headers: dict = None) -> dict:
    url = _get_url_for_endpoint(endpoint)
    out_headers = _get_headers(headers)

    response = requests.get(url, headers=out_headers)

    return _process_response(response)

def _get_headers(input_headers: dict = None) -> dict:
    out_header = {}

    if input_headers:
        out_header = input_headers.copy()

    out_header["Content-Type"] = "application/json"

    if API_KEY:
        out_header["Authorization"] = str.format("Bearer {}", API_KEY)

    return out_header

def _process_response(response) -> dict:
    if 200 <= response.status_code < 300:
        if response.text:
            return json.loads(response.text)
        else:
            return None
    response.raise_for_status()

def _get_url_for_endpoint(endpoint: str):
    return str.format("https://api.axur.com/gateway/1.0/api/{}", endpoint)

def get_ticket_fields(tickets_key: list):
    for ticket_key in tickets_key:
        print(f'DEBUG: AXUR: Fetch ticket {ticket_key} details')

def get_ticket_filter(page: int, ticket_per_page: int):
    formated_initial_time = ONE_HOUR_AGO.strftime('%Y-%m-%dT%H:%M:%S')
    formated_final_time = CURRENT_TIME.strftime('%Y-%m-%dT%H:%M:%S')
    endpoint = str.format(_TICKETS_API + _TICKET_FILTERS_QUERY, formated_initial_time, formated_final_time,
                        SORT_FIELD, page, ticket_per_page)
    return endpoint_get(endpoint)

def tickets_to_csv(ticket: dict):
    with open(f'{FILE_NAME}.csv', 'a', encoding='UTF8', newline='') as f:
        writer = csv.writer(f)
        fields = []
        for field in ticket_fields:
            fields.append(get_ticket_field_value(ticket, field))
        writer.writerow(fields)

def get_ticket_field_value(ticket: dict, field: str):
    for key, nested_dict in ticket.items():
        if isinstance(nested_dict, dict) and field in nested_dict:
            return nested_dict[field]
    return None

def main():
    result = get_ticket_filter(PAGES, TICKET_PER_PAGE)
    total = result['pageable']['total']
    total_pages = math.ceil(total/TICKET_PER_PAGE)
    print(f'DEBUG: AXUR: Total tickets {total} and total pages {total_pages}')
    get_all_tickets(total_pages)

def get_all_tickets(total_pages: int):
    for page in range(1, total_pages+1):
        print(f'DEBUG: AXUR: Fetch page {page} from {total_pages}')
        result = get_ticket_filter(page, TICKET_PER_PAGE)
        process_tickets_key(result)
        tickets = result['tickets']
        for ticket in tickets:
            tickets_to_csv(ticket)

def process_tickets_key(result):
    tickets_key_list = []
    for ticket_key in result['tickets']:
        tickets_key_list.append(ticket_key['ticket']['ticketKey'])
    get_ticket_fields(tickets_key_list)

main()
```

**Bash cURL**

Example using bash script with cURL to extract ticket details

```
#!/bin/bash

API_KEY='<API_KEY>'

ONE_HOUR_AGO=$(TZ='America/Sao_Paulo' date --date='-1 hour' +"%Y-%m-%dT%H:%M:%S")
CURRENT_TIME=$(TZ='America/Sao_Paulo' date +"%Y-%m-%dT%H:%M:%S")
SORT_FIELD="open.date"
PAGES=1
TICKET_PER_PAGE=50

mkdir -p ./tickets

sleep 3

GETQ=`curl --noproxy "*" \
-sLX GET "https://api.axur.com/gateway/1.0/api/tickets-api/tickets?open.date=ge${ONE_HOUR_AGO}&open.date=le${CURRENT_TIME}&sortBy=${SORT_FIELD}&page=${PAGES}&pageSize=${TICKET_PER_PAGE}&order=asc&utc=-03:00" \
-H "Authorization: Bearer ${API_KEY}"`

echo "$GETQ" | tr -d '\000-\037' | jq -c '.tickets[]' | while read -r ticket; do
    key=$(echo $ticket | jq -r '.ticket.ticketKey')
    echo $ticket | jq '.' > "tickets/${key}.json"
done
```

## Retrieve Tickets

**Permission Level Needed:** All permission levels

Returns ticket matching the specified filter.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### query Parameters

| current.type | string Example: current.type=phishing,fraudulent-brand-use The filters must be entered via query parameters (see [Examples of usage](https://docs.axur.com/en/axur/api/#section/Examples-of-usage)). All ticket fields are supported (see [Fields supported by filters](https://docs.axur.com/en/axur/api/#section/Fields-supported-by-filters)). For example, you could use “current.type=phishing,fraudulent-brand-use\&current.open.date=ge2024-01-01” to get tickets of type “phishing” or “fraudulent-brand-use” and with a date greater than or equal to “2024-01-01”. |
| :---- | :---- |
| page | integer Default: 1 Example: page=1 Page number |
| pageSize | integer Default: 50 Example: pageSize=10 Page size |
| sortBy | string Example: sortBy=ticket.creation.date Sort by field |
| order | string Enum: "desc" "asc" Example: order=desc Sort order |
| timezone | string Example: timezone=Z Use one of the records identified in the UTF Offset column on [https://en.wikipedia.org/wiki/Time\_zone\#List\_of\_UTC\_offsets](https://en.wikipedia.org/wiki/Time_zone#List_of_UTC_offsets). The UTC offset will be used to convert the date input via query parameters and the date output via json. |
| include | string Example: include=fields,snapshots,texts,attachments Information about the tickets to be retrieved. Available include parameters are ”fields”, ”snapshots”, ”texts”, ”attachments”. Default is all fields. |

### Responses

**200** OK

**400** Incorret Query Parameters

**403** FORBIDDEN

**429** Rate Limit exceded

get/tickets-api/tickets

### **Request samples**

* **Python 3.7**  
* **Bash cURL**

Copy  
\#Remember to replace the 'ticket\_query' params and 'token' fields.  
import requests

ticket\_query \= 'open.date=ge:2024-06-01T13:00:00\&open.date=le:2024-06-01T14:00:00\&sortBy=open.date\&page=1\&pageSize=50\&order=asc\&utc=-03:00'  
token \= '\<API\_KEY\>'

PATH \= f'https://api.axur.com/gateway/1.0/api/tickets-api/tickets?{ticket\_query}'

header \= {  
    'Content-Type': 'application/json',  
    'Authorization': str.format("Bearer {}", token)  
}

result \= requests.get(PATH, headers\=header)

print(result.json())

### **Response samples**

* **200**  
* **400**  
* **403**  
* **429**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"tickets": [`  
  * `[]`  
* `],`  
* `"pageable": {`  
  * `"pageNumber": 1,`  
  * `"pageSize": 10,`  
  * `"total": 1200`  
* `}`

`}`

## Create Ticket

**Permission Level Needed:** Practitioners/Experts/Customs/Managers

Creates a new ticket with the specified reference, type and asset. Reference schema and asset category must be valid for the selected ticket type. If the operation is successful, the key of the newly created ticket will be returned. However, if the reference already exists, the ticket will not be created. Tickets can be manually created as ticket-types examples bellow.

* *To obtain the list of assets, the 'customer' section must be consulted (see [Get Customer](https://docs.axur.com/en/axur/api/#operation/getCustomers)).*  
* *To obtain the list of ticket types, the 'ticket' section must be consulted (see [Get Ticket Types](https://docs.axur.com/en/axur/api/#operation/getTicketTypes)).*

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### header Parameters

| Content-Type | string Example: application/json |
| :---- | :---- |

##### Request Body schema: application/json

| reference required | string |
| :---- | :---- |
| customer | string |
| type required | string Enum: "code-secret-leak" "executive-fake-social-media-profile" "executive-personalinfo-leak" "fake-mobile-app" "fake-social-media-profile" "fraudulent-brand-use" "malware" "other-sensitive-data" "paid-search" "phishing" "similar-domain-name" "unauthorized-distribution" "unauthorized-sale" Ticket Types can be obtained from the section 'tickets' (see [Get Ticket Types](https://docs.axur.com/en/axur/api/#operation/getTicketTypes)). |
| assets required | Array of objects |

### Responses

**200** OK

**400** Asset/Customer not found

**409** Reference already exists

post/tickets-api/tickets

### **Request samples**

* **Payload**  
* **Python 3.7**  
* **Bash cURL**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"reference": "https://www.a-phishing-url.com/",`  
* `"customer": "TEST",`  
* `"type": "phishing",`  
* `"assets": [`  
  * `[]`  
* `]`

`}`

### **Response samples**

* **200**  
* **400**  
* **409**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"ticket": "2vdyqh",`  
* `"warnings": [`  
  * `{}`  
* `]`

`}`

## Retrieve Bulk by Key

**Permission Level Needed:** All permission levels

Returns ticket matching the specified ticket key.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### query Parameters

| keys required | string Example: keys=tsug6g,asdg8g |
| :---- | :---- |
| timezone | string Example: timezone=Z Use one of the records identified in the UTF Offset column on [https://en.wikipedia.org/wiki/Time\_zone\#List\_of\_UTC\_offsets](https://en.wikipedia.org/wiki/Time_zone#List_of_UTC_offsets). The UTC offset will be used to convert the date input via query parameters and the date output via json. |
| include | string Example: include=fields,snapshots,attachments Information about the tickets to be retrieved. Available include parameters are ”fields”, ”snapshots”, ”texts”, ”attachments”. Default is all fields. |

### Responses

**200** OK

**400** Ticket does not exist

get/tickets-api/ticket

### **Response samples**

* **200**  
* **400**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"tickets": [`  
  * `[]`  
* `],`  
* `"unavailableTickets": [`  
  * `[]`  
* `]`

`}`

## Retrieve by Key

**Permission Level Needed:** All permission levels

Returns ticket matching the specified ticket key.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| ticketKey required | string Example: tsug6g |
| :---- | :---- |

##### query Parameters

| timezone | string Example: timezone=Z Use one of the records identified in the UTF Offset column on [https://en.wikipedia.org/wiki/Time\_zone\#List\_of\_UTC\_offsets](https://en.wikipedia.org/wiki/Time_zone#List_of_UTC_offsets). The UTC offset will be used to convert the date input via query parameters and the date output via json. |
| :---- | :---- |
| include | string Example: include=fields,snapshots,attachments Information about the tickets to be retrieved. Available include parameters are ”fields”, ”snapshots”, ”texts”, ”attachments”. Default is all fields. |

### Responses

**200** OK

**400** Ticket does not exist

get/tickets-api/ticket/{ticketKey}

### **Response samples**

* **200**  
* **400**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"ticket": {`  
  * `"reference": "https://www.facebook.com/61559521153572",`  
  * `"ticketKey": "ui40ay",`  
  * `"customerKey": "ACME",`  
  * `"creation.collector": "facebook",`  
  * `"last-update.date": "2024-08-06T17:16:39-03:03",`  
  * `"creation.collection": "92056",`  
  * `"creation.originator": "collector",`  
  * `"creation.date": "2024-07-17T09:41:01-03:03",`  
  * `"tags": []`  
* `},`  
* `"detection": {`  
  * `"creation.collector": "facebook",`  
  * `"open.date": "2024-07-17T09:41:01-03:03",`  
  * `"incident.date": "2024-08-06T17:16:27-03:03",`  
  * `"isp": "Facebook",`  
  * `"ip": "157.240.229.35",`  
  * `"domain.registrar": "RegistrarSafe, LLC",`  
  * `"type": "fake-social-media-profile",`  
  * `"resolution": "",`  
  * `"resolution.reason": "",`  
  * `"assets": [],`  
  * `"creation.collection": "92056",`  
  * `"domain": "facebook.com",`  
  * `"creation.originator": "collector",`  
  * `"host": "www.facebook.com",`  
  * `"prediction.risk": "0.96",`  
  * `"prediction.brand-logo": "1.0",`  
  * `"status": "incident",`  
  * `"prediction.brand-name": "0.9955",`  
  * `"group": []`  
* `},`  
* `"texts": [`  
  * `{}`  
* `],`  
* `"snapshots": {`  
  * `"content": {},`  
  * `"domainInfo": {},`  
  * `"isp": {},`  
  * `"digitalLocation": {},`  
  * `"correlatedLinks": { }`  
* `},`  
* `"attachments": [`  
  * `{},`  
  * `{},`  
  * `{}`  
* `]`

`}`

## Ticket Count by Status

This refers to the **Monitoring** section of stats (see [Stats](https://one.axur.com/stats))

**Note:** from and to parameters are required and must not exceed 90 days.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### query Parameters

| customer | string Example: customer=ACME |
| :---- | :---- |
| from required | string \<date-time\> Example: from=2024-01-01 Date in the format "YYYY-MM-DD" or "YYYY-MM-DDTHH:mm:ss" |
| to required | string \<date-time\> Example: to=2024-01-31 Date in the format "YYYY-MM-DD" or "YYYY-MM-DDTHH:mm:ss" |
| ticketTypes | string Example: ticketTypes=phishing,paid-search Supported types: executive-card-leak,similar-domain-name,executive-personalinfo-leak,data-sale-website,phishing,executive-credential-leak,unauthorized-distribution,database-exposure,code-secret-leak,executive-mobile-phone,suspicious-activity-message,fraud-tool-scheme-message,fraud-tool-scheme-website,data-sale-message,fake-mobile-app,fraudulent-brand-use,malware,fake-social-media-profile,corporate-credential-leak,other-sensitive-data,infostealer-credential,data-exposure-website,executive-fake-social-media-profile,ransomware-attack,paid-search,suspicious-activity-website,data-exposure-message,dw-activity,unauthorized-sale,infrastructure-exposure |
| status required | string Example: status=open Supported status: open, quarantine, incident, treatment, closed |
| assets | string Example: assets=BRANDACME |
| timezone | string Example: timezone=-03:00 Timezone in the format "HH:mm" More examples can be found on: [https://en.wikipedia.org/wiki/List\_of\_UTC\_offsets](https://en.wikipedia.org/wiki/List_of_UTC_offsets). Notice timezone is optional, if not informed the default timezone is UTC |

### Responses

**200** OK

**403** FORBIDDEN

get/tickets-api/stats/customer

### **Response samples**

* **200**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"total": 99999`

`}`

## Ticket Incident Count by Type

This refers to the **Incidents by threat type** section of stats (see [Stats](https://one.axur.com/stats))

**Note:** from and to parameters are required and must not exceed 90 days.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### query Parameters

| customer | string Example: customer=ACME |
| :---- | :---- |
| from required | string \<date-time\> Example: from=2024-01-01 Date in the format "YYYY-MM-DD" or "YYYY-MM-DDTHH:mm:ss" |
| to required | string \<date-time\> Example: to=2024-01-31 Date in the format "YYYY-MM-DD" or "YYYY-MM-DDTHH:mm:ss" |
| ticketTypes | string Example: ticketTypes=phishing,paid-search Supported types: executive-card-leak,similar-domain-name,executive-personalinfo-leak,data-sale-website,phishing,executive-credential-leak,unauthorized-distribution,database-exposure,code-secret-leak,executive-mobile-phone,suspicious-activity-message,fraud-tool-scheme-message,fraud-tool-scheme-website,data-sale-message,fake-mobile-app,fraudulent-brand-use,malware,fake-social-media-profile,corporate-credential-leak,other-sensitive-data,infostealer-credential,data-exposure-website,executive-fake-social-media-profile,ransomware-attack,paid-search,suspicious-activity-website,data-exposure-message,dw-activity,unauthorized-sale,infrastructure-exposure |
| assets | string Example: assets=BRANDACME |
| timezone | string Example: timezone=12:00 Timezone in the format "HH:mm" More examples can be found on: [https://en.wikipedia.org/wiki/List\_of\_UTC\_offsets](https://en.wikipedia.org/wiki/List_of_UTC_offsets). Notice timezone is optional, if not informed the default timezone is UTC |

### Responses

**200** OK

**400** BAD REQUEST

**403** FORBIDDEN

**429** Too many requests. Rate limit of 60 requests per minute exceeded.

get/tickets-api/stats/incident/count/ticket-types

### **Response samples**

* **200**  
* **400**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"totalByTicketType": [`  
  * `[]`  
* `]`

`}`

## Takedown Metrics

This refers to the **Treatment \> Takedown** section of stats (see [Stats](https://one.axur.com/stats))

**Note:** from and to parameters are required and must not exceed 90 days.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### query Parameters

| customer | string Example: customer=ACME |
| :---- | :---- |
| from required | string \<date-time\> Example: from=2024-01-01 Date in the format "YYYY-MM-DD" or "YYYY-MM-DDTHH:mm:ss" |
| to required | string \<date-time\> Example: to=2024-01-31 Date in the format "YYYY-MM-DD" or "YYYY-MM-DDTHH:mm:ss" |
| ticketTypes | string Example: ticketTypes=phishing,paid-search Supported types: executive-card-leak,similar-domain-name,executive-personalinfo-leak,data-sale-website,phishing,executive-credential-leak,unauthorized-distribution,database-exposure,code-secret-leak,executive-mobile-phone,suspicious-activity-message,fraud-tool-scheme-message,fraud-tool-scheme-website,data-sale-message,fake-mobile-app,fraudulent-brand-use,malware,fake-social-media-profile,corporate-credential-leak,other-sensitive-data,infostealer-credential,data-exposure-website,executive-fake-social-media-profile,ransomware-attack,paid-search,suspicious-activity-website,data-exposure-message,dw-activity,unauthorized-sale,infrastructure-exposure |
| assets | string Example: assets=BRANDACME |
| timezone | string Example: timezone=12:00 Timezone in the format "HH:mm" More examples can be found on: [https://en.wikipedia.org/wiki/List\_of\_UTC\_offsets](https://en.wikipedia.org/wiki/List_of_UTC_offsets). Notice timezone is optional, if not informed the default timezone is UTC |

### Responses

**200** OK

**400** BAD REQUEST

**403** FORBIDDEN

**429** Too many requests. Rate limit of 60 requests per minute exceeded.

get/tickets-api/stats/takedown

### **Response samples**

* **200**  
* **400**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"total": {`  
  * `"requested": 3111,`  
  * `"successRate": 97.74070212,`  
  * `"medianTimeToFirstNotification": "P4DT13H",`  
  * `"medianUptime": "P23DT23H",`  
  * `"rawMedianUptime": 120771776.29297659,`  
  * `"discarded": 0,`  
  * `"aborted": 93,`  
  * `"pending": 141,`  
  * `"resolved": 2812,`  
  * `"unresolved": 65`  
* `}`

`}`

## Internal Treatment Metrics

This refers to the **Treatment \> Internal treatment** section of stats (see [Stats](https://one.axur.com/stats))

**Note:** from and to parameters are required and must not exceed 90 days.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### query Parameters

| customer | string Example: customer=ACME |
| :---- | :---- |
| from required | string \<date-time\> Example: from=2024-01-01 Date in the format "YYYY-MM-DD" or "YYYY-MM-DDTHH:mm:ss" |
| to required | string \<date-time\> Example: to=2024-01-31 Date in the format "YYYY-MM-DD" or "YYYY-MM-DDTHH:mm:ss" |
| ticketTypes | string Example: ticketTypes=phishing,paid-search Supported types: executive-card-leak,similar-domain-name,executive-personalinfo-leak,data-sale-website,phishing,executive-credential-leak,unauthorized-distribution,database-exposure,code-secret-leak,executive-mobile-phone,suspicious-activity-message,fraud-tool-scheme-message,fraud-tool-scheme-website,data-sale-message,fake-mobile-app,fraudulent-brand-use,malware,fake-social-media-profile,corporate-credential-leak,other-sensitive-data,infostealer-credential,data-exposure-website,executive-fake-social-media-profile,ransomware-attack,paid-search,suspicious-activity-website,data-exposure-message,dw-activity,unauthorized-sale,infrastructure-exposure |
| assets | string Example: assets=BRANDACME |
| timezone | string Example: timezone=-11:00 Timezone in the format "HH:mm" More examples can be found on: [https://en.wikipedia.org/wiki/List\_of\_UTC\_offsets](https://en.wikipedia.org/wiki/List_of_UTC_offsets). Notice timezone is optional, if not informed the default timezone is UTC |

### Responses

**200** OK

**400** BAD REQUEST

**403** FORBIDDEN

**429** Too many requests. Rate limit of 60 requests per minute exceeded.

get/tickets-api/stats/internal-treatment

### **Response samples**

* **200**  
* **400**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"total": {`  
  * `"requested": 0,`  
  * `"unresolved": 0,`  
  * `"medianUptime": "PT0S",`  
  * `"successRate": 100,`  
  * `"takedownRequested": 0,`  
  * `"pending": 0,`  
  * `"resolved": 0,`  
  * `"discarded": 0`  
* `}`

`}`

## Takedown Uptime

This refers to the **Resolution uptime in days** section of stats (see [Stats](https://one.axur.com/stats))

**Note:** from and to parameters are required and must not exceed 90 days.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### query Parameters

| customer | string Example: customer=ACME |
| :---- | :---- |
| from required | string \<date-time\> Example: from=2024-01-01 Date in the format "YYYY-MM-DD" or "YYYY-MM-DDTHH:mm:ss" |
| to required | string \<date-time\> Example: to=2024-01-31 Date in the format "YYYY-MM-DD" or "YYYY-MM-DDTHH:mm:ss" |
| ticketTypes | string Example: ticketTypes=phishing,paid-search Supported types: executive-card-leak,similar-domain-name,executive-personalinfo-leak,data-sale-website,phishing,executive-credential-leak,unauthorized-distribution,database-exposure,code-secret-leak,executive-mobile-phone,suspicious-activity-message,fraud-tool-scheme-message,fraud-tool-scheme-website,data-sale-message,fake-mobile-app,fraudulent-brand-use,malware,fake-social-media-profile,corporate-credential-leak,other-sensitive-data,infostealer-credential,data-exposure-website,executive-fake-social-media-profile,ransomware-attack,paid-search,suspicious-activity-website,data-exposure-message,dw-activity,unauthorized-sale,infrastructure-exposure |
| assets | string Example: assets=BRANDACME |

### Responses

**200** OK

**400** BAD REQUEST

**403** FORBIDDEN

**429** Too many requests. Rate limit of 60 requests per minute exceeded.

get/tickets-api/stats/takedown/uptime

### **Response samples**

* **200**  
* **400**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"uptime": {`  
  * `"lessThan1Day": 1280,`  
  * `"upTo2Days": 296,`  
  * `"upTo5Days": 535,`  
  * `"upTo10Days": 145,`  
  * `"upTo15Days": 365,`  
  * `"upTo30Days": 78,`  
  * `"upTo60Days": 113,`  
  * `"over60Days": 0`  
* `}`

`}`

## Internal Treatment Uptime

This refers to the **Resolution uptime in days** section of stats (see [Stats](https://one.axur.com/stats))

**Note:** from and to parameters are required and must not exceed 90 days.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### query Parameters

| customer | string Example: customer=ACME |
| :---- | :---- |
| from required | string \<date-time\> Example: from=2024-01-01 Date in the format "YYYY-MM-DD" or "YYYY-MM-DDTHH:mm:ss" |
| to required | string \<date-time\> Example: to=2024-01-31 Date in the format "YYYY-MM-DD" or "YYYY-MM-DDTHH:mm:ss" |
| ticketTypes | string Example: ticketTypes=phishing,paid-search Supported types: executive-card-leak,similar-domain-name,executive-personalinfo-leak,data-sale-website,phishing,executive-credential-leak,unauthorized-distribution,database-exposure,code-secret-leak,executive-mobile-phone,suspicious-activity-message,fraud-tool-scheme-message,fraud-tool-scheme-website,data-sale-message,fake-mobile-app,fraudulent-brand-use,malware,fake-social-media-profile,corporate-credential-leak,other-sensitive-data,infostealer-credential,data-exposure-website,executive-fake-social-media-profile,ransomware-attack,paid-search,suspicious-activity-website,data-exposure-message,dw-activity,unauthorized-sale,infrastructure-exposure |
| assets | string Example: assets=BRANDACME |
| timezone | string Example: timezone=-09:30 Timezone in the format "HH:mm" More examples can be found on: [https://en.wikipedia.org/wiki/List\_of\_UTC\_offsets](https://en.wikipedia.org/wiki/List_of_UTC_offsets). Notice timezone is optional, if not informed the default timezone is UTC |

### Responses

**200** OK

**400** BAD REQUEST

**403** FORBIDDEN

**429** Too many requests. Rate limit of 60 requests per minute exceeded.

get/tickets-api/stats/internal-treatment/uptime

### **Response samples**

* **200**  
* **400**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"uptime": {`  
  * `"lessThan1Day": 1280,`  
  * `"upTo2Days": 296,`  
  * `"upTo5Days": 535,`  
  * `"upTo10Days": 145,`  
  * `"upTo15Days": 365,`  
  * `"upTo30Days": 78,`  
  * `"upTo60Days": 113,`  
  * `"over60Days": 0`  
* `}`

`}`

## Median number of internal incidents by Axur customers

This refers to the **Axur customers median** section of stats (see [Stats](https://one.axur.com/stats))

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### query Parameters

| customer | string Example: customer=ACME |
| :---- | :---- |
| to required | string \<date-time\> Example: to=2024-01-01 Date in the format "YYYY-MM-DD" or "YYYY-MM-DDTHH:mm:ss". Get 13 months back from the indicated date |
| ticketTypes | string Example: ticketTypes=phishing,paid-search Supported types: executive-card-leak,similar-domain-name,executive-personalinfo-leak,data-sale-website,phishing,executive-credential-leak,unauthorized-distribution,database-exposure,code-secret-leak,executive-mobile-phone,suspicious-activity-message,fraud-tool-scheme-message,fraud-tool-scheme-website,data-sale-message,fake-mobile-app,fraudulent-brand-use,malware,fake-social-media-profile,corporate-credential-leak,other-sensitive-data,infostealer-credential,data-exposure-website,executive-fake-social-media-profile,ransomware-attack,paid-search,suspicious-activity-website,data-exposure-message,dw-activity,unauthorized-sale,infrastructure-exposure |
| assets | string Example: assets=BRANDACME |
| timezone | string Example: timezone=-09:30 Timezone in the format "HH:mm" More examples can be found on: [https://en.wikipedia.org/wiki/List\_of\_UTC\_offsets](https://en.wikipedia.org/wiki/List_of_UTC_offsets). Notice timezone is optional, if not informed the default timezone is UTC |

### Responses

**200** OK

**400** BAD REQUEST

**403** FORBIDDEN

**429** Too many requests. Rate limit of 60 requests per minute exceeded.

get/tickets-api/stats/incident/customer/global/median

### **Response samples**

* **200**  
* **400**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"averageType": "median",`  
* `"monthlyStats": [`  
  * `[]`  
* `]`

`}`

## Mean number of internal incidents by Axur customers

This refers to the **Axur customers mean** section of stats (see [Stats](https://one.axur.com/stats))

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### query Parameters

| customer | string Example: customer=ACME |
| :---- | :---- |
| to required | string \<date-time\> Example: to=2024-01-01 Date in the format "YYYY-MM-DD" or "YYYY-MM-DDTHH:mm:ss". Get 13 months back from the indicated date |
| ticketTypes | string Example: ticketTypes=phishing,paid-search Supported types: executive-card-leak,similar-domain-name,executive-personalinfo-leak,data-sale-website,phishing,executive-credential-leak,unauthorized-distribution,database-exposure,code-secret-leak,executive-mobile-phone,suspicious-activity-message,fraud-tool-scheme-message,fraud-tool-scheme-website,data-sale-message,fake-mobile-app,fraudulent-brand-use,malware,fake-social-media-profile,corporate-credential-leak,other-sensitive-data,infostealer-credential,data-exposure-website,executive-fake-social-media-profile,ransomware-attack,paid-search,suspicious-activity-website,data-exposure-message,dw-activity,unauthorized-sale,infrastructure-exposure |
| assets | string Example: assets=BRANDACME |
| timezone | string Example: timezone=-03:00 Timezone in the format "HH:mm" More examples can be found on: [https://en.wikipedia.org/wiki/List\_of\_UTC\_offsets](https://en.wikipedia.org/wiki/List_of_UTC_offsets). Notice timezone is optional, if not informed the default timezone is UTC |

### Responses

**200** OK

**400** BAD REQUEST

**403** FORBIDDEN

**429** Too many requests. Rate limit of 60 requests per minute exceeded.

get/tickets-api/stats/incident/customer/global/mean

### **Response samples**

* **200**  
* **400**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"averageType": "mean",`  
* `"monthlyStats": [`  
  * `[]`  
* `]`

`}`

## Median number of Incidents by Customer Market Segment

This refers to the **{{Market Segment}} sector median** section of stats (see [Stats](https://one.axur.com/stats))

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### query Parameters

| customer | string Example: customer=ACME |
| :---- | :---- |
| to required | string \<date-time\> Example: to=2024-01-01 Date in the format "YYYY-MM-DD" or "YYYY-MM-DDTHH:mm:ss". Get 13 months back from the indicated date |
| ticketTypes | string Example: ticketTypes=phishing,paid-search Supported types: executive-card-leak,similar-domain-name,executive-personalinfo-leak,data-sale-website,phishing,executive-credential-leak,unauthorized-distribution,database-exposure,code-secret-leak,executive-mobile-phone,suspicious-activity-message,fraud-tool-scheme-message,fraud-tool-scheme-website,data-sale-message,fake-mobile-app,fraudulent-brand-use,malware,fake-social-media-profile,corporate-credential-leak,other-sensitive-data,infostealer-credential,data-exposure-website,executive-fake-social-media-profile,ransomware-attack,paid-search,suspicious-activity-website,data-exposure-message,dw-activity,unauthorized-sale,infrastructure-exposure |
| assets | string Example: assets=BRANDACME |
| timezone | string Example: timezone=-03:00 Timezone in the format "HH:mm" More examples can be found on: [https://en.wikipedia.org/wiki/List\_of\_UTC\_offsets](https://en.wikipedia.org/wiki/List_of_UTC_offsets). Notice timezone is optional, if not informed the default timezone is UTC |

### Responses

**200** OK

**400** BAD REQUEST

**403** FORBIDDEN

**429** Too many requests. Rate limit of 60 requests per minute exceeded.

get/tickets-api/stats/incident/customer/market-segment/median

### **Response samples**

* **200**  
* **400**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"marketSegment": "TECHNOLOGY",`  
* `"medians": [`  
  * `[]`  
* `]`

`}`

## Mean number of Incidents by Customer Market Segment

This refers to the **{{Market Segment}} sector mean** section of stats (see [Stats](https://one.axur.com/stats))

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### query Parameters

| customer | string Example: customer=ACME |
| :---- | :---- |
| to required | string \<date-time\> Example: to=2024-01-01 Date in the format "YYYY-MM-DD" or "YYYY-MM-DDTHH:mm:ss". Get 13 months back from the indicated date |
| ticketTypes | string Example: ticketTypes=phishing,paid-search Supported types: executive-card-leak,similar-domain-name,executive-personalinfo-leak,data-sale-website,phishing,executive-credential-leak,unauthorized-distribution,database-exposure,code-secret-leak,executive-mobile-phone,suspicious-activity-message,fraud-tool-scheme-message,fraud-tool-scheme-website,data-sale-message,fake-mobile-app,fraudulent-brand-use,malware,fake-social-media-profile,corporate-credential-leak,other-sensitive-data,infostealer-credential,data-exposure-website,executive-fake-social-media-profile,ransomware-attack,paid-search,suspicious-activity-website,data-exposure-message,dw-activity,unauthorized-sale,infrastructure-exposure |
| assets | string Example: assets=BRANDACME |
| timezone | string Example: timezone=-03:00 Timezone in the format "HH:mm" More examples can be found on: [https://en.wikipedia.org/wiki/List\_of\_UTC\_offsets](https://en.wikipedia.org/wiki/List_of_UTC_offsets). Notice timezone is optional, if not informed the default timezone is UTC |

### Responses

**200** OK

**400** BAD REQUEST

**403** FORBIDDEN

**429** Too many requests. Rate limit of 60 requests per minute exceeded.

get/tickets-api/stats/incident/customer/market-segment/mean

### **Response samples**

* **200**  
* **400**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"marketSegment": "TECHNOLOGY",`  
* `"mean": [`  
  * `[]`  
* `]`

`}`

# Integration Feed

## Examples of usage

Get feed by id

```
https://api.axur.com/gateway/1.0/api/integration-feed/feeds/feed/643e17a7-7d21-43d4-b24c-1a88d10a3114
```

Get feed by id with dry-run. The difference in this case is that the dry-run approach does neither mark the page nor the last data retrieved.

```
https://api.axur.com/gateway/1.0/api/integration-feed/feeds/feed/643e17a7-7d21-43d4-b24c-1a88d10a3114?dry-run=true
```

## Important information

* The feed by id returns an object containing the feedData and the collectionData. For now, the collectionData is an object like the GET tickets-api/tickets endpoint;  
* There is a rate limit per feed Id set to 30s.

## Get feed by id

**Permission Level Needed:** Managers

Returns a feed information containing feed details and its associated collection data. Each request by feedId has a rate limit of one request per 30 seconds.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| feedId required | string Example: 9e780987-10df-409e-9d1b-a63dfeecded0 |
| :---- | :---- |

##### query Parameters

| dry-run | boolean Example: dry-run=true use this query parameter if you want to test the request. |
| :---- | :---- |

### Responses

**200** OK

**403** Forbidden

**404** Not Found

**429** Too Many Requests

get/integration-feed/feeds/feed/{feedId}

### **Response samples**

* **200**  
* **404**  
* **429**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"feedData": {`  
  * `"id": "a3c-867-40f-b79-f18d74",`  
  * `"title": "Closed tickets",`  
  * `"url": "https://api.axur.com/gateway/1.0/api/integration-feed/feeds/feed/a3c-867-40f-b79-f18d74",`  
  * `"createdAt": 1727983596891,`  
  * `"lastRequest": 1728063161589,`  
  * `"lastDataRetrieved": "e2ma5b",`  
  * `"isActive": true,`  
  * `"customerKey": "ACME",`  
  * `"updatedAt": 1728062627600,`  
  * `"params": "status=closed",`  
  * `"eventDate": "ticket.creation.date",`  
  * `"nextPage": 2,`  
  * `"feedType": "ticket"`  
* `},`  
* `"collectionData": {`  
  * `"tickets": [],`  
  * `"pageable": {}`  
* `}`

`}`

# Filter Operations

Filters allow users to specify queries and find existing tickets that match desired criteria. Filter results include only ticket keys, which must be used with valid ticket operations if additional information is needed.

A filter is composed of one or multiple *queries* and an *operation* to combine them (such as *AND* or *OR*). Each query is composed of one *field*, one or multiple *values* and an *operation* applied on the specified field or values.

## Supported operations

* AND  
* OR  
* NOT  
* GREATER\_THAN  
* GREATER\_THAN\_OR\_EQUAL  
* LESS\_THAN  
* LESS\_THAN\_OR\_EQUAL  
* BETWEEN  
* EXISTS  
* NOT\_EXISTS  
* CONTAINS\_TEXT

All ticket fields (see [Retrieve ticket fields](https://docs.axur.com/en/axur/api/#operation/getFields) operation below) with preffix ticket. and current. are supported in search filters.

## Supported fields

The following fields may be included in search queries:

* current.assets  
* current.close.date  
* current.creation.user  
* current.open.date  
* current.resolution.reason  
* current.resolution  
* current.status  
* current.takedown  
* current.type  
* ticket.creation.date  
* ticket.creation.user  
* ticket.tags  
* ticket.customer

Please note that some fields in the documentation, including the ones listed below, are subject to change or may become obsolete. There is no assurance that they will remain unchanged since fields are related to products in continuos improvement. Furthermore, this list may not contain all possible fields:

* current.ad.product.location  
* current.ad.product.price  
* current.ad.seller.username  
* current.alleged.exposed.birthdate  
* current.alleged.exposed.documents  
* current.creation.customer  
* current.credential.first-seen  
* current.credential.password.type  
* current.credential.password.value.masked  
* current.credential.password.value  
* current.credential.username  
* current.credential.login-source  
* current.criticality  
* current.document.type  
* current.document.value.masked  
* current.documents.masked  
* current.documents.plain  
* current.documents.type  
* current.domain.registrar  
* current.domain  
* current.host  
* current.incident.date  
* current.ip  
* current.isp  
* current.leak.sources  
* current.message.group.name  
* current.platform  
* current.prediction.brand-logo  
* current.prediction.brand-name  
* current.prediction.languages  
* current.prediction.main-language  
* current.prediction.risk  
* current.profile  
* current.quarantine.date  
* current.quarantine.last.updated.date  
* current.quarantine.unread  
* current.source.date  
* current.takedown.cancel.date  
* current.takedown.close.date  
* current.takedown.close.type  
* current.takedown.first-notification.date  
* current.takedown.notification.date  
* current.takedown.notification.last.type  
* current.takedown.request.date  
* current.takedown.resolution.reason  
* current.takedown.resolution  
* current.takedown.reup  
* current.takedown.submission.date  
* current.takedown.uptime  
* current.takedown.verification.date  
* current.takedown.verification.last.type  
* current.token.creation.date  
* current.treatment.date  
* current.treatment.internal  
* current.treatment.type  
* current.treatment.uptime  
* ticket.creation.customer  
* ticket.last-update.date

## Examples

A filter that matches tickets of type phishing with status incident or treatment:

```json
{
"queries": [
    {
    "fieldName": "current.type",
    "values": [
        "phishing"
    ],
    "operation": "AND"
    },
    {
    "fieldName": "current.status",
    "values": [
        "incident",
        "treatment"
    ],
    "operation": "OR"
    }
],
"operation": "AND"
}
```

A filter that matches closed deep & dark web activity tickets in Telegram or Whatsapp:

```json
{
"queries": [
    {
    "fieldName": "current.status",
    "values": [
        "closed"
    ],
    "operation": "AND"
    },
    {
    "fieldName": "current.type",
    "values": [
        "dw-activity",
        "data-exposure-message",
        "data-sale-message",
        "fraud-tool-scheme-message",
        "suspicious-activity-message"
    ],
    "operation": "AND"
    },
    {
    "fieldName": "current.platform",
    "values": [
        "telegram",
        "whatsapp"
    ],
    "operation": "OR"
    }
],
"operation": "AND"
}
```

A filter that matches tickets updated since July 1st, 2022 UTC (all dates are represented as timestamps in milliseconds):

```json
{
"queries": [
    {
    "fieldName": "ticket.last-update.date",
    "values": [
        1656633600000
    ],
    "operation": "GREATER_THAN_OR_EQUAL"
    }
],
"operation": "AND"
}
```

## Create Filter

*A new API has been implemented to facilitate the process of obtaining tickets (see [Tickets API](https://docs.axur.com/en/axur/api/#section/Examples-of-usage)).*

**Permission Level Needed:** All permission levels

Creates a new search filter from the request body and returns its identifier.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### header Parameters

| Accept | string Example: application/json |
| :---- | :---- |
| Content-Type | string Example: application/json |

##### Request Body schema: application/json

| queries | Array of objects Array of one or more queries |
| :---- | :---- |
| operation | string Enum: "AND" "OR" "NOT" "GREATER\_THAN" "GREATER\_THAN\_OR\_EQUAL" "LESS\_THAN" "LESS\_THAN\_OR\_EQUAL" "BETWEEN" "EXISTS" "NOT\_EXISTS" "CONTAINS\_TEXT" Operation |

### Responses

**200** OK

**403** FORBIDDEN

post/tickets-filters/filters/tickets

### **Request samples**

* **Payload**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"queries": [`  
  * `[]`  
* `],`  
* `"operation": "AND"`

`}`

### **Response samples**

* **200**  
* **403**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"queryId": "fee315d76f684831bd89df885b775b33"`

`}`

## Retrieve filter results

*A new API has been implemented to facilitate the process of obtaining tickets (see [Tickets API](https://docs.axur.com/en/axur/api/#section/Examples-of-usage)).*

**Permission Level Needed:** All permission levels

Returns ticket keys matching the specified filter.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### query Parameters

| q required | string Example: q=20b55a35dc7a4bd89892c8a3db09e2c0 Filter id |
| :---- | :---- |
| page | integer Default: 1 Example: page=1 Page number |
| pageSize | integer Default: 50 Example: pageSize=10 Page size |
| sortBy | string Example: sortBy=ticket.creation.date Sort by field |
| order | string Enum: "desc" "asc" Example: order=desc Sort order |

### Responses

**200** OK

get/tickets-filters/filters/tickets

### **Response samples**

* **200**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"tickets": [`  
  * `[]`  
* `],`  
* `"metadata": {`  
  * `"total": 123,`  
  * `"perPage": 10,`  
  * `"page": 1,`  
  * `"offset": 0`  
* `}`

`}`

# Ticket Operations

## Ticket Types

* code-secret-leak  
* corporate-credential-leak  
* database-exposure  
* other-sensitive-data  
* dw-activity  
* data-exposure-website  
* data-sale-website  
* fraud-tool-scheme-website  
* suspicious-activity-website  
* data-exposure-message  
* data-sale-message  
* fraud-tool-scheme-message  
* suspicious-activity-message  
* infrastructure-exposure  
* fake-mobile-app  
* fake-social-media-profile  
* fraudulent-brand-use  
* malware  
* paid-search  
* phishing  
* similar-domain-name  
* unauthorized-distribution  
* unauthorized-sale  
* executive-card-leak  
* executive-credential-leak  
* executive-fake-social-media-profile  
* executive-personalinfo-leak  
* infostealer-credential  
* ransomware-attack

## Retrieve ticket fields

*A new API has been implemented to facilitate the process of obtaining tickets (see [Tickets API](https://docs.axur.com/en/axur/api/#section/Examples-of-usage)).*

**Permission Level Needed:** All permission levels

Returns field values for the specified ticket.

Returned fields may vary depending on the ticket type and other criteria.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| ticketKey required | string Example: tsug6g |
| :---- | :---- |

### Responses

**200** OK

get/tickets-core/tickets/{ticketKey}

### **Request samples**

* **Python 3.7**  
* **Bash cURL**

Copy  
\#Remember to replace the 'ticket\_key' and 'token' fields.  
import requests

ticket\_key \= 'ticket key'

PATH \= f'https://api.axur.com/gateway/1.0/api/tickets-core/tickets/{ticket\_key}'

token \= 'token'

header \= {   
  'Content-Type': 'application/json',  
  'Authorization': str.format("Bearer {}", token)  
}

result \= requests.get(PATH, headers\=header)

print(result.json())

### **Response samples**

* **200**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"ticket": {`  
  * `"key": "tsug6g",`  
  * `"customer": "TEST",`  
  * `"reference": "https://www.instagram.com/anyfakeprofile",`  
  * `"fields": [],`  
  * `"detection": {}`  
* `}`

`}`

## Get Ticket Types

**Permission Level Needed:** All permission levels

Retrieves the currently supported ticket types.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

### Responses

**200** OK

get/tickets-core/fields/types

### **Response samples**

* **200**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"types": [`  
  * `[]`  
* `]`

`}`

## Retrieve ticket textual data

*A new API has been implemented to facilitate the process of obtaining tickets (see [Tickets API](https://docs.axur.com/en/axur/api/#section/Examples-of-usage)).*

**Permission Level Needed:** All permission levels

Returns textual data items for the specified ticket, such as user comments, description, evidence message etc.

Returned items may vary depending on the ticket type and other criteria.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| ticketKey required | string Example: k2il51 |
| :---- | :---- |

### Responses

**200** OK

get/tickets-texts/texts/tickets/{ticketKey}

### **Response samples**

* **200**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"texts": [`  
  * `{},`  
  * `{}`  
* `]`

`}`

## Add comment to ticket

**Permission Level Needed:** Practitioners/Experts/Customs/Managers

Adds a text comment to the specified ticket.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| ticketKey required | string Example: hvewpn |
| :---- | :---- |

##### Request Body schema: application/json

| content | string |
| :---- | :---- |
| internal | boolean |
| type | string |

### Responses

**200** OK

post/tickets-texts/texts/tickets/{ticketKey}

### **Request samples**

* **Payload**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"content": "Lorem ipsum dolor sit amet.",`  
* `"internal": false,`  
* `"type": "comment"`

`}`

## Retrieve ticket snapshots

*A new API has been implemented to facilitate the process of obtaining tickets (see [Tickets API](https://docs.axur.com/en/axur/api/#section/Examples-of-usage)).*

**Permission Level Needed:** All permission levels

Retrieve snapshot from specified ticket detection.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| ticketKey required | string Example: hvewpn |
| :---- | :---- |
| detectionIndex required | integer Example: 0 |

### Responses

**200** OK

get/tickets-snapshots/snapshots/tickets/{ticketKey}/detections/{detectionIndex}/merge

### **Response samples**

* **200**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"ticket": "n432h7",`  
* `"detection": 0,`  
* `"details": {`  
  * `"content": {},`  
  * `"domainInfo": {},`  
  * `"isp": {},`  
  * `"digitalLocation": {}`  
* `}`

`}`

## Retrieve ticket lifecycle options

**Permission Level Needed:** Experts/Customs with Takedown permission/Managers

Returns currently available lifecycle transitions for the specified ticket.

Returned keys may be used to execute lifecycle transitions (see [Execute lifecycle transition](https://docs.axur.com/en/axur/api/#operation/applyTransition) operation below).

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| ticketKey required | string Example: xi8luf |
| :---- | :---- |

### Responses

**200** OK

get/tickets-lifecycle/lifecycle/tickets/{ticketKey}/transitions

### **Request samples**

* **Python 3.7**  
* **Bash cURL**

Copy  
\#Remember to replace the 'ticket\_key' and 'token' fields.  
import requests

ticket\_key \= 'ticket key'  
PATH \= f'https://api.axur.com/gateway/1.0/api/tickets-lifecycle/lifecycle/tickets/{ticket\_key}/transitions'

token \= 'token'

header \= {  
    'Content-Type': 'application/json',  
    'Authorization': str.format("Bearer {}", token)  
}

result \= requests.get(PATH, headers\=header)

print(result.json())

### **Response samples**

* **200**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"transitions": [`  
  * `{},`  
  * `{}`  
* `],`  
* `"currentStatus": "open"`

`}`

## Execute lifecycle transition

**Permission Level Needed:** Experts/Customs with Takedown permission

Executes a lifecycle transition (given as a path parameter) on the specified ticket.

For a list of valid transitions, see [Retrieve ticket lifecycle options](https://docs.axur.com/en/axur/api/#operation/listTicketAvailableTransitions) operation above.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| ticketKey required | string Example: 11c4hl |
| :---- | :---- |
| transitionKey required | string Example: closed:discarded |

### Responses

**200** OK

post/tickets-lifecycle/lifecycle/tickets/{ticketKey}/transitions/{transitionKey}

### **Request samples**

* **Python 3.7**  
* **Bash cURL**

Copy  
\#Remember to replace the 'ticket\_key', 'transition\_key' and 'token' fields.  
import requests

ticket\_key \= '11c4hl'  
transition\_key \= 'closed:discarded'  
token \= 'token'

PATH \= f'https://api.axur.com/gateway/1.0/api/tickets-lifecycle/lifecycle/tickets/{ticket\_key}/transitions/{transition\_key}'

header \= {  
    'Content-Type': 'application/json',  
    'Authorization': str.format("Bearer {}", token)  
}

result \= requests.post(PATH, headers\=header)

print(result.json())

### **Response samples**

* **200**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"ticket": "11c4hl",`  
* `"detection": 0,`  
* `"actions": [`  
  * `[]`  
* `]`

`}`

## Retrieve ticket takedown options

**Permission Level Needed:** Experts/Customs with Takedown permission/Managers

Returns currently available takedown options for the specified ticket. A request value in the actions array indicates that the ticket is eligible for takedown.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| ticketKey required | string Example: nbz092 |
| :---- | :---- |

### Responses

**200** OK

get/tickets-takedown/takedown/tickets/{ticketKey}

### **Request samples**

* **Python 3.7**  
* **Bash cURL**

Copy  
\#Remember to replace the 'ticket\_key' and 'token' fields.  
import requests

ticket\_key \= 'b8tq53'  
PATH \= f'https://api.axur.com/gateway/1.0/api/tickets-takedown/takedown/tickets/{ticket\_key}'

token \= 'token'

header \= {  
    'Content-Type': 'application/json',  
    'Authorization': str.format("Bearer {}", token)  
}

result \= requests.get(PATH, headers\=header)

print(result.json())

### **Response samples**

* **200**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"ticket": "nbz092",`  
* `"detection": 0,`  
* `"actions": [`  
  * `"request"`  
* `]`

`}`

## Request takedown

**Permission Level Needed:** Experts/Customs with Takedown permission/Managers

Requests takedown for the specified ticket. The ticket's most recent detection index must be specified as a path parameter.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| ticketKey required | string Example: nbz092 |
| :---- | :---- |
| detectionIndex required | integer Example: 0 |

##### header Parameters

| Content-Type | string Example: application/json |
| :---- | :---- |

##### Request Body schema: application/json

| action | string |
| :---- | :---- |

### Responses

**200** OK

post/tickets-takedown/takedown/tickets/{ticketKey}/detections/{detectionIndex}

### **Request samples**

* **Payload**  
* **Python 3.7**  
* **Bash cURL**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"action": "request"`

`}`

## Get ticket history

**Permission Level Needed:** All permission levels

Retrieve the chronological timeline (history) of specific events for the ticket. An effective way to track takedown status.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| ticketKey required | string Example: 6ctz5w |
| :---- | :---- |

### Responses

**200** OK

get/tickets-timeline/timelines/ticket-history/{ticketKey}

### **Response samples**

* **200**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"ticket": "6ctz5w",`  
* `"actions": [`  
  * `{},`  
  * `{},`  
  * `{},`  
  * `{},`  
  * `{},`  
  * `{},`  
  * `{},`  
  * `{},`  
  * `{},`  
  * `{},`  
  * `{},`  
  * `{},`  
  * `{},`  
  * `{},`  
  * `{},`  
  * `{},`  
  * `{},`  
  * `{},`  
  * `{},`  
  * `{},`  
  * `{},`  
  * `{}`  
* `]`

`}`

## Add tag to ticket

**Permission Level Needed:** Practitioners/Experts/Customs/Managers

Adds a tag to the specified ticket.

Current ticket tags can be found in field ticket.tags (see [Retrieve ticket fields](https://docs.axur.com/en/axur/api/#operation/getFields) operation above).

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| ticketKey required | string Example: hvewpn |
| :---- | :---- |
| tagLabel required | string Example: {my-tag} |

### Responses

**200** OK

put/tickets-tags/tags/tickets/{ticketKey}/{tagLabel}

## Remove tag from ticket

**Permission Level Needed:** Practitioners/Experts/Customs/Managers

Removes a tag from the specified ticket.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| ticketKey required | string Example: hvewpn |
| :---- | :---- |
| tagLabel required | string Example: {my-tag} |

### Responses

**200** OK

delete/tickets-tags/tags/tickets/{ticketKey}/{tagLabel}

## Retrieve ticket attachments

**Permission Level Needed:** All permission levels

Retrieve attachments from specified ticket detection.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| ticketKey required | string Example: hvewpn |
| :---- | :---- |
| detectionIndex required | integer Example: 0 |

### Responses

**200** OK

get/tickets-attachments/attachments/tickets/{ticketKey}/detections/{detectionIndex}

### **Response samples**

* **200**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"attachments": [`  
  * `{},`  
  * `{},`  
  * `{}`  
* `]`

`}`

## Add attachment to ticket

**Permission Level Needed:** Practitioners/Experts/Customs/Managers

Add attachment to specified ticket detection.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| ticketKey required | string Example: hvewpn |
| :---- | :---- |
| detectionIndex required | integer Example: 0 |

##### Request Body schema: multipart/form-data

| file | string \<binary\> The attachment file to be uploaded |
| :---- | :---- |

### Responses

**200** OK

post/tickets-attachments/attachments/tickets/{ticketKey}/detections/{detectionIndex}

### **Response samples**

* **200**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"name": "name.png",`  
* `"url": "https://api.axur.com/gateway/1.0/api/files/file/name.png",`  
* `"metadata": {`  
  * `"byteSize": 0,`  
  * `"contentType": "text/html",`  
  * `"illustrative": false,`  
  * `"author": 123`  
* `},`  
* `"date": 1672066466935,`  
* `"attachmentId": "Base64 encoded URL",`  
* `"fileKey": "abcd1234"`

`}`

## Retrieve ticket attachments just by key

**Permission Level Needed:** All permission levels

Retrieve attachments from specified ticket detection.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| ticketKey required | string Example: hvewpn |
| :---- | :---- |

### Responses

**200** OK

get/tickets-attachments/attachments/tickets/{ticketKey}/detections/detection

### **Response samples**

* **200**

Content type  
application/json

Copy

Expand allCollapse all

`[`

* `{`  
  * `"name": "whois_123456.txt",`  
  * `"url": "https://api.axur.com/gateway/1.0/api/files/file/whois_123456.txt"`  
* `},`  
* `{`  
  * `"name": "html_123456.html",`  
  * `"url": "https://api.axur.com/gateway/1.0/api/files/file/html_123456.html"`  
* `},`  
* `{`  
  * `"name": "screenshot_123456.jpg",`  
  * `"url": "https://api.axur.com/gateway/1.0/api/files/file/screenshot_123456.jpg"`  
* `}`

`]`

## Delete attachment from ticket

**Permission Level Needed:** Practitioners/Experts/Customs/Managers

Remove an attachment from specified ticket detection.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| ticketKey required | string Example: hvewpn |
| :---- | :---- |
| detectionIndex required | integer Example: 0 |
| attachmentId required | string Example: aHR0cHM6Ly9hcGkuYXh1ci5jb20vZ2F0ZXdheS8xLjAvZmlsZXMvZmlsZS9odG1sXzEyMzQ1Ni5odG1s |

### Responses

**204** Attachment was successfully removed.

delete/tickets-attachments/attachments/tickets/{ticketKey}/detections/{detectionIndex}/{attachmentId}

## Retrieve Infostealer credentials leaks associated with a ticket Deprecated

**DEPRECATION NOTICE: this endpoint has been replaced by the [Exposures API](https://docs.axur.com/en/axur/api/#tag/Credential-Search-Operations) and will be removed in a future version.**

**Permission Level Needed:** All permission levels

Returns leaks associated with a ticket of type infostealer-credential.

Possible username types:

* EMAIL  
* CPF  
* CNPJ  
* PHONE  
* USERNAME

Possible password types:

* PLAIN  
* BASE64  
* MD5  
* SHA1  
* SHA224  
* SHA256  
* SHA384  
* SHA512  
* BCRYPT  
* PBKDF2  
* MYSQL323  
* UNKNOWN

NOTE: for users without the *prc* role, the password is masked.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| ticketKey required | string Example: tsug6g |
| :---- | :---- |

### Responses

**200** OK

get/tickets-infostealer-credentials/tickets/{ticketKey}

### **Request samples**

* **Python 3.7**  
* **Bash cURL**

Copy  
\#Remember to replace the 'ticket\_key' and 'token' fields.  
import requests

ticket\_key \= 'ticket key'

PATH \= f'https://api.axur.com/gateway/1.0/api/tickets-infostealer-credentials/tickets/{ticket\_key}'

token \= 'token'

header \= {   
  'Content-Type': 'application/json',  
  'Authorization': str.format("Bearer {}", token)  
}

result \= requests.get(PATH, headers\=header)

print(result.json())

### **Response samples**

* **200**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"leaks": [`  
  * `{},`  
  * `{}`  
* `]`

`}`

# Axur Platform webhooks

Webhooks are an advanced API feature that allows external applications to receive Axur events in almost real time, as they happen, through HTTP or HTTPS. To be able to receive webhook events, the external application must provide an unauthenticated HTTP or HTTPS endpoint accessible from the Internet.

**Important**: In order to protect sensitive information it is recommended to use an HTTPS endpoint with a valid SSL/TLS certificate.

These are the published event types:

* ticket.created  
* ticket.reference.changed  
* ticket.detection.opened  
* ticket.detection.reclassified  
* ticket.detection.closed  
* ticket.quarantine.updated  
* ticket.lifecycle.transition-applied  
* ticket.takedown.requested  
* ticket.takedown.notified  
* ticket.takedown.verified  
* ticket.takedown.done  
* ticket.snapshot.added  
* identity.session.refreshed  
* identity.session.created  
* identity.user.created  
* identity.user.deactivated  
* identity.user.reactivated  
* exposure.created  
* exposure.updated

## Simple Integration Guide for Webhooks

To integrate with the Webhooks feature, follow these basic steps:

1. **Prerequisites**  
   * **HTTP Server**: You need an HTTP server with an exposed URL that accepts POST requests.  
   * **Secure Endpoint**: Ensure your URL is HTTPS-enabled with a valid SSL/TLS certificate to protect sensitive information.  
2. **Validating Requests**  
   * **Signature Verification**: Every webhook request is signed using HMAC-SHA256 with a shared secret.  
     * The signature is included in the **X-Axur-Signature** HTTP header.  
     * Use the shared secret from your subscription to validate the authenticity of the request body.  
3. **Filtering Events Programmatically**  
   * To optimize processing, you can programmatically filter the types of events you want to handle. For example, you might focus on events like ticket.created or ticket.takedown.requested depending on your integration requirements.

## Signature

Since the provided endpoint must be freely accessible from the Internet, the external application needs to verify the authenticity of incoming Axur requests. The body of every published webhook event is signed using HMAC-SHA256 with a shared secret. Each webhook subscription has a different randomly generated secret (see [Create subscription](https://docs.axur.com/en/axur/api/#operation/createSubscription) operation, below). The signature's hexadecimal value is included in the custom HTTP header X-Axur-Signature.

## Requests

All Webhook requests follow the same contract. Some information available is always present but others are event type specific.

You can check out below some request examples according to some published event types, followed by the description of some particular details of each event:

### **Example Request for** ticket.detection.opened

```
{
"event": {
    "type": "ticket.created",
    "date": 1660316140900,
    "sequence": 6424942
},
"ticket": {
    "key": "mc2c0p",
    "customer": "ACME",
    "reference": "https://www.test.com"
}
}
```

### **Example Request for** ticket.detection.opened

```
{
    "event": {
        "type": "ticket.detection.opened",
        "date": 1733947747323,
        "sequence": 9744269
    },
    "ticket": {
        "key": "05vbhy",
        "customer": "ACME",
        "detection": 0,
        "reference": "https://www.test.com",
        "type": "phishing",
        "assets": [
            "ACME"
        ]
    }
}
```

### **Example Request for** ticket.lifecycle.transition-applied

```
{
    "event": {
        "type": "ticket.lifecycle.transition-applied",
        "date": 1733947749776,
        "sequence": 1482475
    },
    "ticket": {
        "key": "05vbhy",
        "customer": "ACME",
        "detection": 0,
        "status": "incident"
    }
}
```

### **Example Request for** ticket.takedown.requested

```
{
    "event": {
        "type": "ticket.takedown.requested",
        "date": 1733948094085,
        "sequence": 2289797
    },
    "ticket": {
        "key": "3xpcn7",
        "customer": "ACME",
        "detection": 0,
        "resolution": "none"
    }
}
```

All events with the **ticket** prefix follow the same pattern, ensuring a consistent approach to handling them. These events only indicate the ticket.key they belong to, which is why it is necessary to retrieve the full ticket details using the specified endpoint.

### **Steps to Retrieve Ticket Information**

1. **Extract the** ticket.key**:** Extract the ticket.key from the returned body. In the example above, the ticket.key is mc2c0p.  
2. **Use the** ticket.key **to retrieve details:** Use the extracted ticket.key to make a request to the following endpoint:

```
https://api.axur.com/gateway/1.0/api/tickets-api/ticket/{ticketKey}
```

4.   
   Replace {ticketKey} with the actual value of the ticket.key. For instance:

```
https://api.axur.com/gateway/1.0/api/tickets-api/ticket/mc2c0p
```

6.   
   **Retrieve ticket details:** This request will provide all the details related to the specific ticket associated with the ticket.key.

### **Other events**

#### **Example Request for** identity.user.created

```
{
    "event": {
        "type": "identity.user.created",
        "date": 1733948640366,
        "author": 12575,
        "byAxur": true
    },
    "user": {
        "email": "test@test.com",
        "customer": "ACME",
        "firstName": "John",
        "lastName": "ACme",
        "roles": [
            "mng"
        ],
        "groups": [
            "manager"
        ]
    }
}
```

#### **Example Request for** exposure.updated

```
{
  "event": {
    "type": "exposure.updated",
    "date": 1733948640366,
    "sequence": 62365
  },
  "detections": [
    {
      "id": "2025052911574317566C29A490F30E33B",
      "status": "IN_TREATMENT",
      "category": "credential",
      "created": 1733948640366,
      "updated": 1752167332650,
      "customer": "ACME",
      "assets": [
        "ACM"
      ],
      "tags": [
        "Urgent"
      ],
      "credential": {
        "types": [
          "employee",
          "user"
        ],
        "user": "john@acme.com",
        "user.type": "EMAIL",
        "password": "p@ssword",
        "password.type": "PLAIN",
        "access.url": "https://vazou.com/login",
        "source.name": "Deep/Dark Web - Telegram"
      }
    }
  ]
}
```

Exposure events can have up to 1000 detections per request.

## Retrieve subscriptions

**Permission Level Needed:** Managers

Returns all webhook subscriptions and related info (identifiers, endpoints and secrets).

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

### Responses

**200** Retrieve webhook subscriptions

get/webhooks/subscriptions

### **Response samples**

* **200**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"subscriptions": [`  
  * `{}`  
* `]`

`}`

## Create subscription

**Permission Level Needed:** Managers

Creates a new webhook subscription with the given endpoint to receive ticket events in real-time.

Important: the endpoint value must be a valid URL that accepts the POST verb and returns an HTTP success code (2xx), otherwise an error will be thrown.

An automatically generated secret is returned along with the created subscription information, which should be used to validate the signature of published events.

After this request is completed, new ticket events will be published to the webhook endpoint.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### Request Body schema: application/json

| endpoint | string |
| :---- | :---- |

### Responses

**200** Create webhook subscription

post/webhooks/subscriptions

### **Request samples**

* **Payload**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"endpoint": "<webhook endpoint URL>"`

`}`

### **Response samples**

* **200**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"endpoint": "https://www.acme.net/webhooks",`  
* `"id": 321,`  
* `"secret": "ddPxe5Vdh9XH1kemf7NWyhRVKWYalTj3L0yrZDuvMSg1CousHokWF38RoxsaYs35MS7jwkuBqVPSHRvpybYgrlEEJxX8GQhhxVUi1ViaGD5SAEwH68PUsEkHC1KQa5z9"`

`}`

## Remove subscription

**Permission Level Needed:** Managers

Removes the existing webhook subscription with the given identifier. After this request is completed, new ticket events will not be published to the webhook endpoint.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| subscriptionKey required | number Example: 321 |
| :---- | :---- |

### Responses

**204** Remove webhook subscription

delete/webhooks/subscriptions/{subscriptionKey}

## Ping subscription

**Permission Level Needed:** Managers

Sends a ping event to the endpoint of the existing webhook subscription with the given identifier, for testing or validation purposes.\\nIf this operation succeeds, it returns an HTTP 200 status code and a JSON body containing the pinged endpoint's HTTP response data. Otherwise, a JSON response body like {\\"error\\": \\"message\\"} is returned.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| subscriptionKey required | number Example: 321 |
| :---- | :---- |

### Responses

**200** Ping subscription

post/webhooks/subscriptions/{subscriptionKey}/ping

### **Response samples**

* **200**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"body": "body from webhook endpoint response",`  
* `"status": 200`

`}`

# Safelist operations

A safelist item is a URL or host that will be ignored when the Axur platform analyzes potential threats. If Axur detects a URL that matches a safelist item, the URL is considered safe and a corresponding ticket will not be created.

## Create safelist item

**Permission Level Needed:** Experts/Customs with Safelist permission/Managers

Creates one or multiple safelist items.

**Field** group accepts values self, partner, email and other, indicating the desired group for the created safelist items.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### header Parameters

| Authorization required | string Example: Bearer \<token\> |
| :---- | :---- |

##### Request Body schema: application/json

| customerKey required | string |
| :---- | :---- |
| group required | string |
| items required | Array of objects |

### Responses

**204** Safelist item was added successfully.

**400** Safelist item wasn't added because has a body request error

**403** Forbidden

post/touchpoints/items

### **Request samples**

* **Payload**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"customerKey": "ACM",`  
* `"group": "SELF",`  
* `"items": [`  
  * `{}`  
* `]`

`}`

### **Response samples**

* **400**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"error": "items are required"`

`}`

## Get customer safelist items

**Permission Level Needed:** Experts/Customs with Safelist permission/Managers

Get customer safelist items by parameters.

**Parameter** group accepts values self, partner, email and other, and filters the response to only include safelist items within the indicated group.

**Parameter** categories accepts values app, marketplace, social and other, and filters the response to only include safelist items within the indicated categories.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### query Parameters

| customerKey required | string Example: customerKey=ACM |
| :---- | :---- |
| group | string Example: group=self |
| categories | string Example: categories=social,app |
| page | integer Example: page=2 |
| perPage | integer Example: perPage=3 |

##### header Parameters

| Authorization required | string Example: Bearer \<token\> |
| :---- | :---- |

### Responses

**200** success.

**400** Safelist items weren't listed because has a request error

**403** Forbidden

get/touchpoints/items

### **Response samples**

* **200**  
* **400**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"metadata": {`  
  * `"page": 2,`  
  * `"perPage": 3,`  
  * `"offset": 2,`  
  * `"total": 5`  
* `},`  
* `"safelistItems": [`  
  * `{}`  
* `]`

`}`

## Update safelist item

**Permission Level Needed:** Experts/Customs with Safelist permission/Managers

Updates a safelist item.

**Field** group accepts values self, partner, email and other, indicating the desired group for the created safelist item.

**Field** category accepts values app, marketplace, social and other, indicating the desired categories for filter safelist item.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| id required | integer Example: 1 |
| :---- | :---- |

##### header Parameters

| Authorization required | string Example: Bearer \<token\> |
| :---- | :---- |

##### Request Body schema: application/json

| content required | string |
| :---- | :---- |
| group required | string |
| category required | string |

### Responses

**204** Safelist item was updated successfully.

**400** Safelist item wasn't updated because has a body request error

**403** Forbidden

**404** Safelist item not found

put/touchpoints/items/{id}

### **Request samples**

* **Payload**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"content": "example-updated.com/acme",`  
* `"group": "SELF",`  
* `"category": "social"`

`}`

### **Response samples**

* **400**  
* **404**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"error": "content is required"`

`}`

## Deactivate safelist item

**Permission Level Needed:** Experts/Customs with Safelist permission/Managers

Deactivates a safelist item.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| id required | integer Example: 1 |
| :---- | :---- |

##### header Parameters

| Authorization required | string Example: Bearer \<token\> |
| :---- | :---- |

### Responses

**204** Safelist item was deactivated successfully.

**403** Forbidden

**404** Safelist item not found

delete/touchpoints/items/{id}

### **Response samples**

* **404**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"error": "safelist item not found"`

`}`

# Users

Users API requests allows customers to translate user\_id information into name and last name. In the light of that, customers will be able to exactly identify the author of safelist submission or ticket creation, for instance.

## Get Users

**Permission Level Needed:** Managers

Get user by parameters.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### query Parameters

| customers | string Example: customers=ACM1,ACM2 |
| :---- | :---- |
| accessToAreas | string Example: accessToAreas=area1,area1 |
| freeText | string Example: freeText=freetext |
| offset | integer Example: offset=1 |
| pageSize | integer Example: pageSize=10 |

##### header Parameters

| Authorization required | string Example: Bearer \<token\> |
| :---- | :---- |

### Responses

**200** success.

**403** Forbidden

get/identity/users

### **Response samples**

* **200**

Content type  
application/json

Copy

Expand allCollapse all

`[`

* `{`  
  * `"id": {},`  
  * `"credentials": {},`  
  * `"customer": {},`  
  * `"policies": [],`  
  * `"profile": {},`  
  * `"groups": [],`  
  * `"roles": [],`  
  * `"internal": false,`  
  * `"active": true,`  
  * `"twoFactorEnabled": false,`  
  * `"lgpd": {}`  
* `}`

`]`

## Get Users Stream

**Permission Level Needed:** Managers

Get user stream by parameters.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### query Parameters

| customers | string Example: customers=ACM1,ACM2 |
| :---- | :---- |
| accessToAreas | string Example: accessToAreas=area1,area1 |
| freeText | string Example: freeText=freetext |
| offset | integer Example: offset=1 |
| pageSize | integer Example: pageSize=10 |

##### header Parameters

| Authorization required | string Example: Bearer \<token\> |
| :---- | :---- |

### Responses

**200** success.

**403** Forbidden

get/identity/users/stream

### **Response samples**

* **200**

Content type  
application/json

Copy

Expand allCollapse all

`[`

* `{`  
  * `"id": {},`  
  * `"credentials": {},`  
  * `"customer": {},`  
  * `"policies": [],`  
  * `"profile": {},`  
  * `"groups": [],`  
  * `"roles": [],`  
  * `"internal": false,`  
  * `"active": true,`  
  * `"twoFactorEnabled": false,`  
  * `"lgpd": {}`  
* `}`

`]`

# Customer/Asset

Customer API requests allows customers to translate ´assetKey´ information into asset name. In the light of that, customers will be able to identify exactly which asset the ticket belongs to, for example.

## Get Customer

**Permission Level Needed:** All permission levels

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### header Parameters

| Authorization required | string Example: Bearer \<token\> |
| :---- | :---- |

### Responses

**200** success.

**403** Forbidden

get/customers/customers

### **Response samples**

* **200**

Content type  
application/json

Copy

Expand allCollapse all

`[`

* `{`  
  * `"name": "Acme Corporation",`  
  * `"key": "ACME",`  
  * `"assets": [],`  
  * `"properties": [ ],`  
  * `"active": true`  
* `}`

`]`

# API Fields Description

Description of ticket API fields

| Field | Description |
| :---- | :---- |
| key | The unique identification code of a ticket. |
| ticket.reference | The reference of what was detected. It can be a URL, IP, or domain depending on the ticket type. |
| tickets-snapshots: referenceInfo.originalReference | If a detected website has a tracking URL, it will appear in this field. |
| current.ip | The detected website's IP. |
| current.isp | It is the Internet service provider where the URL is hosted. |
| ticket.creation.date | Date when the ticket was created on the platform. |
| ticket.creation.user | The user who created the ticket; it can be someone from Axur or someone from the customer's team. |
| ticket.creation.originator | The means by which a ticket was created. It may be via the Axur Platform, Onepixel, or manual insertion via API. |
| detection.open.date | The date when a URL was detected by the platform, typically the same as the creation date. |
| current.incident.date | The date when a ticket was last moved to the Incidents tab. |
| current.status | The tab (status) in which the ticket is, *i.e.* Potential threats, Incidents, Closed, Quarantine, Treatment. |
| current.type | What the current ticket type is, like Fake social media profile or Counterfeit or irregular sale. |
| current.treatment.internal | If internal treatment: detection.treatment.internal \= true; if takedown detection.treatment.internal \= false |
| current.takedown.request.date | The date a Takedown was last requested in a ticket. |
| current.resolution | What the ticket resolution was, whether it was Resolved, Unresolved, Interrupted or Discarded. If the ticket does not have a resolution yet, the value is empty. |
| current.resolution.date | Date on which the ticket was resolved (either by takedown, internal treatment or discarded) |
| current.resolution.reason | Resolution reason |
| current.close.date | The date the ticket was closed, regardless of the solution. |
| current.assets | Which assets are binded to a ticket. |
| ticket.creation.collector | Collector/source that identified the threat. |
| current.domain | The domain for the detected URL. |
| current.host | The URL's full name without the directories. It is composed by domain \+ subdomains. Ex. sub.domain.com. |
| tickets-snapshots: details.domainInfo.registrantEmail | The domain registrant's email, obtained from Whois. |
| tickets-snapshots: details.domainInfo.registrantName | The domain registrant's name, also obtained from Whois. |
| current.treatment.uptime | The time it took to close the ticket. On the CSV file, this number is in days, but the API shows it in milliseconds. |
| tickets-snapshots: details.content.title | The page title that appears in the browser tab. |
| tickets-snapshots: details.content.hash | When a web page becomes a ticket, a hash is generated from its content and it is used to validate integrity. |
| tickets-snapshots: details.content.httpStatusCode.code | When a web page is accessed, it returns an HTTP code. This code represents information about the connection, for example, if the value is 200, it means the connection was successful, but if it is 404, it means there was a failure. |
| current.tags | The tags currently present in a ticket. |
| current.ad.seller.username | In irregular sale scenarios, some plataforms make available some information about what is being advertised. In this case, the information is the seller name. |
| current.ad.product.price | In irregular sale scenarios, some plataforms make available some information about what is being advertised. In this case, the information is the product price. |
| current.ad.product.location | In irregular sale scenarios, some plataforms make available some information about what is being advertised. In this case, the information is the product location. |
| tickets-snapshots: details.content.ad.product.soldAmount | In irregular sale scenarios, some plataforms make available some information about what is being advertised. In this case, the information is the product amount available for sale. |
| current.takedown.first-notification.date | Time in milliseconds between detection.takedown.request.date and detection.treatment.date |
| current.takedown.notification.date | List of all the dates on which notifications were made. |
| current.credential.username | Leaked email |
| current.credential.password.value | Leaked password |
| current.credential.password.type | Password type (hash, plain) |
| current.platform | Activity platform |
| current.profile | Profile of threat Actor |
| detection.criticality | Degree of criticality of DW tickets |
| current.message.group.name | Threat messaging group |
| current.threat-content | Content of the threat |

# CSV Ticket Extraction Sample

Simple example of extracting tickets to a csv file. Standard authentication is used and with date filters initially set to two hours ago.

* Remember to override the pre-set values in the code such as email and password.

## Python

This example uses Python 3.7 as programming language.

```
import math
import requests
import json
import datetime
import csv


API_KEY = '<API KEY>'

ticket_fields = ['key', 'reference', 'current.type', 'current.assets', 'current.open.date',
                'current.status', 'current.resolution', 'current.resolution.reason', 'current.close.date']

_TICKET_FILTERS = "tickets-filters/filters/tickets"
_TICKET_FILTERS_QUERY = "tickets-filters/filters/tickets?q={}&page={}&pageSize={}&sortBy={}&order={}"
_TICKETS_FIELDS = "tickets-core/tickets/{}"

FILTER_JSON = '''{
    "queries": [
        {
            "fieldName": "ticket.last-update_date",
            "values": [
                "$start_date"
            ],
            "operation": "GREATER_THAN"
        },
        {
            "fieldName": "ticket.last-update_date",
            "values": [
                "$end_date"
            ],
            "operation": "LESS_THAN"
        }
    ],
    "operation": "AND"
}'''

TICKET_PER_PAGE = 50


def endpoint_get(endpoint: str, headers: dict = None) -> dict:
    url = _get_url_for_endpoint(endpoint)
    out_headers = _get_headers(headers)

    response = requests.get(url, headers=out_headers)

    return _process_response(response)


def endpoint_post(endpoint: str, data: dict, headers: dict = None) -> dict:
    url = _get_url_for_endpoint(endpoint)
    out_headers = _get_headers(headers)

    response = requests.post(url, json=data, headers=out_headers)

    return _process_response(response)


def _get_headers(input_headers: dict = None) -> dict:
    out_header = {}

    if input_headers:
        out_header = input_headers.copy()

    out_header["Content-Type"] = "application/json"

    if API_KEY:
        out_header["Authorization"] = str.format("Bearer {}", API_KEY)

    return out_header


def _process_response(response) -> dict:
    if response.status_code >= 200 and response.status_code < 300:
        if response.text:
            return json.loads(response.text)
        else:
            return None
    response.raise_for_status()


def _get_url_for_endpoint(endpoint: str):
    return str.format("https://api.axur.com/gateway/1.0/api/{}", endpoint)


def get_ticket_fields(tickets_key: list):
    tickets = []
    for ticket_key in tickets_key:
        print(f'DEBUG: AXUR: Fetch ticket {ticket_key} details')
        tickets.append(endpoint_get(
            str.format(_TICKETS_FIELDS, ticket_key))['ticket'])
    return tickets


def post_ticket_filter(filter: str, start: datetime, end: datetime):
    start_date = int(start.timestamp() * 1000)
    end_date = int(end.timestamp() * 1000)
    endpoint = _TICKET_FILTERS
    data = extract_query(filter, start_date, end_date)
    return endpoint_post(endpoint, data)


def extract_query(filter, start_date, end_date):
    filter_query = filter.replace('\n', '')
    filter_query = filter_query.replace("$start_date", str(
        start_date)).replace("$end_date", str(end_date))
    data = json.loads(filter_query)
    return data


def get_ticket_filter(query_id: str, page: int, ticket_per_page: int):
    endpoint = str.format(_TICKET_FILTERS_QUERY, query_id,
                        page, ticket_per_page, "ticket.last-update_date", "desc")
    return endpoint_get(endpoint)


def tickets_to_csv(ticket: dict):
    with open('ticket_fields.csv', 'a', encoding='UTF8', newline='') as f:
        writer = csv.writer(f)
        fields = []
        for field in ticket_fields:
            fields.append(get_ticket_field_value(ticket, field))
        writer.writerow(fields)


def get_ticket_field_value(ticket: dict, field: str):
    if (field.__eq__("key")) or (field.__eq__("customer")) or field.__eq__("reference"):
        return ticket[field]
    else:
        for ticket_field in ticket['fields']:
            if ticket_field['key'].__eq__(field):
                if (ticket_field['dimension'].__eq__('multi')):
                    return ';'.join(ticket_field['values'])
                else:
                    return ticket_field['value']


def main():
    current_time = datetime.datetime.now()
    one_hour_ago = current_time - datetime.timedelta(seconds=3600)

    json_query = post_ticket_filter(
        FILTER_JSON, one_hour_ago, current_time)
    result = get_ticket_filter(
        json_query['queryId'], 1, TICKET_PER_PAGE)
    total = result['metadata']['total']
    total_pages = math.ceil(total/TICKET_PER_PAGE)
    print(f'DEBUG: AXUR: Total tickets {total} and total pages {total_pages}')
    get_all_tickets(json_query['queryId'], total_pages)


def get_all_tickets(query_id: str, total_pages: int):
    for page in range(1, total_pages+1):
        print(f'DEBUG: AXUR: Fetch page {page} from {total_pages}')
        result = get_ticket_filter(query_id, page, TICKET_PER_PAGE)
        process_tickets(result)


def process_tickets(result):
    tickets_key_list = []
    for ticket_key in result['tickets']:
        tickets_key_list.append(ticket_key['key'])
    tickets = get_ticket_fields(tickets_key_list)
    for ticket in tickets:
        tickets_to_csv(ticket)


main()
```

## Bash cURL

Example using bash script with cURL to extract ticket details

```
#!/bin/bash

API_KEY='<API_KEY>'

START_DATE=1701557092571
END_DATE=1716557092571

if ! [ -e ".base-tickets" ] ; then
    touch ".base-tickets"
fi

# CREATE QUERY

SEARCH=`curl --noproxy "*" -sLX POST 'https://api.axur.com/gateway/1.0/api/tickets-filters/filters/tickets' \
-H 'Content-Type: application/json' \
-H "Authorization: Bearer $API_KEY" \
--data '{
"queries": [
    {
            "fieldName": "ticket.last-update_date",
            "values": [
                "'$START_DATE'"
            ],
            "operation": "GREATER_THAN"
        },
        {
            "fieldName": "ticket.last-update_date",
            "values": [
                "'$END_DATE'"
            ],
            "operation": "LESS_THAN"
        }
],
    "operation": "AND"
}'`

QID=`echo $SEARCH | cut -d'"' -f4`

sleep 3

# QUERY RESULTS

GETQ=`curl --noproxy "*" \
-sLX GET 'https://api.axur.com/gateway/1.0/api/tickets-filters/filters/tickets?q='${QID}'&page=1&pageSize=50&sortBy=current.open.date&order=desc' \
-H "Authorization: Bearer $API_KEY"`

TCKT=`echo $GETQ | grep -oP '(?<={"key":").*?(?=")'`

echo "$TCKT" > tickets

FOPEN=`grep -vxFf .base-tickets tickets`

sleep 3

# RETRIEVE - TICKET DETAILS

echo "$FOPEN" > .temp_tickets

while IFS= read -r line;
do
  echo "Text read from file: $line"

  GETRTV=`curl --noproxy "*" -sLX GET 'https://api.axur.com/gateway/1.0/api/tickets-core/tickets/'${line}'' \
  -H "Authorization: Bearer $API_KEY"`

  echo "$GETRTV" > $line.json
  echo $line >> .base-tickets
done < ".temp_tickets"
```

# Credential Search Operations

## Examples of search usage

Retrieve all credentials detected with specific criteria, such as their status and user information, and using the Arizona (USA) timezone:

```
GET https://api.axur.com/gateway/1.0/api/exposure-api/credentials?status=NEW,DISCARDED&user=contains:admin&sortBy=created&page=1&pageSize=100&order=asc&timezone=-07:00
```

Retrieve credentials filtered by detection date range, using the Brazil timezone, and retrieving only status, user, password, created, assets, password.hasLetter, and password.length fields:

```
GET https://api.axur.com/gateway/1.0/api/exposure-api/credentials?created=ge:2024-07-01&updated=le:2024-07-31&sortBy=created&page=1&pageSize=1&order=desc&timezone=-03:00&fields=status,user,password,created,assets,password.hasLetter,password.length
```

The response for the request above will be HTTP STATUS 200 OK with the following body:

```json
{
  "detections": [
    {
      "password": "DtsKcJvlpM",
      "assets": [
        "ANY_ASSET"
      ],
      "created": "2024-07-27T17:18:36",
      "password.hasLetter": true,
      "user": "test@example.com",
      "password.length": 10,
      "status": "DISCARDED"
    }
  ],
  "pageable": {
    "pageNumber": 1,
    "pageSize": 1,
    "total": 200
  }
}
```

## Credentials exposure operators

For date or numeric fields, you can use the following operators:

* gt: \- Greater than  
* lt: \- Less than  
* ge: \- Greater than or equal  
* le: \- Less than or equal

Operators should be used as prefixes for values in query parameters, for example:

* created=ge:2024-07-01  
* updated=le:2024-07-31

For text fields, you can use the following operators:

* contains: \- Contains Text

Operators should be used as prefixes for values in query parameters, for example:

* user=contains:admin

To choose which fields to return, use the fields query parameter, separating the desired fields by commas, for example:

* fields=status,user,password,created,assets,password.hasLetter,password.length

## Fields supported by exposure API

| Key | Type | Notes |
| :---- | :---- | :---- |
| access.appId | STRING | Access app ID |
| access.domain | STRING | Access domain |
| access.host | STRING | Access host |
| access.tld | STRING | Access TLD |
| access.url | STRING | Credential access URL |
| assets | LIST | A list containing the asset keys, needs to be separated by comma, e.g: "ASSET1,ASSET,ASSET 3" |
| created | DATE | Creation date, supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33" |
| credential.types | LIST | A list containing the Credential types, credential type is an enum, it can be "user" or "employee". The list needs to be separated by comma, e.g: "user,employee" |
| customer | STRING | Customer Key identifier |
| detectionDate | DATE | Detection date, supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33" |
| document.id | STRING | Document ID |
| document.links.originalFile | STRING | Source file |
| document.links.originalFilePackage | STRING | Source file package |
| document.links.parsedFrom | STRING | File chunk |
| document.links.raw | STRING | Document of origin |
| document.timestamp | DATE | Document creation date, supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33" |
| downloadLink.detectionDate | DATE | Download link detection date |
| downloadLink.source | STRING | Download link source |
| downloadLink.value | STRING | Download link |
| file.mimeType | STRING | File MIME type |
| file.name | STRING | File name |
| file.originalPath | STRING | File original path |
| file.path | STRING | File path |
| file.relativePath | STRING | File relative path |
| file.sizeInBytes | INTEGER | File size (Bytes) |
| file.timestamp | DATE | File processing date, supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33" |
| id | STRING | Detection ID |
| intelx.fileName | STRING | IntelX file Name |
| intelx.fileType | STRING | IntelX file Type |
| intelx.publishDate | DATE | IntelX publish date, supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33" |
| leak.descriptions.en | STRING | Leak description |
| leak.descriptions.es | STRING | Leak description (Spanish) |
| leak.descriptions.pt | STRING | Leak description (Portuguese) |
| leak.displayName | STRING | Leak display name |
| leak.exposureDate | DATE | Leak exposure date, supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33" |
| leak.forum | STRING | Leak forum |
| leak.name | STRING | Leak name |
| leak.source | STRING | Leak source |
| leak.url | STRING | Leak URL |
| leak.format | STRING | Leak format, it can be "COMBOLIST", "TABLE" or "STEALER LOG" |
| message.author.identifier | INTEGER | Message Author Identifier |
| message.author.type | STRING | Message author type |
| message.autoDeleteIn | INTEGER | Message auto delete in |
| message.chat.identifier | INTEGER | Message Chat Identifier |
| message.chat.name | STRING | Message chat name |
| message.chat.type | STRING | Message chat type |
| message.identifier | INTEGER | Message identifier |
| message.repliedTo.chatIdentifier | INTEGER | Replied Message Chat Identifier |
| message.repliedTo.messageIdentifier | INTEGER | Replied Message Identifier |
| message.selfDestructIn | INTEGER | Self-destruct in |
| message.threadId | INTEGER | Conversation identifier |
| message.timestamp | DATE | Message publication date, supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33" |
| message.type | STRING | Message type |
| password | STRING | Password |
| password.hasLetter | BOOLEAN | Contains letter |
| password.hasLowerCase | BOOLEAN | Contains lowercase letter |
| password.hasNumber | BOOLEAN | Contains number |
| password.hasSpecialChar | BOOLEAN | Contains special character |
| password.hasUpperCase | BOOLEAN | Contains uppercase letter |
| password.length | INTEGER | Password length |
| password.type | ENUM | Password type, it can be "PLAIN", "MYSQL323", "MD5", "SHA512", "SHA256", "SHA1", "BCRYPT", "SHA384" or "PBKDF2" |
| paste.date | DATE | Paste publication date, supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33" |
| paste.expire | DATE | Paste expiration date, supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33" |
| paste.originLocation | STRING | Paste URL |
| paste.source | STRING | Paste source |
| paste.title | STRING | Paste title |
| paste.user | STRING | Paste author |
| source.name | STRING | Source name |
| source.timestamp | DATE | Exposure date at the source, supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33" |
| source.url | STRING | Source URL |
| status | ENUM | Detection Status, it can be "NEW", "IN\_TREATMENT", "SOLVED" or "DISCARDED" |
| tags | LIST | Custom labels for organizing detections. |
| updated | DATE | Detection update date, supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33" |
| user | STRING | Username |
| user.emailDomain | STRING | Email domain |
| user.emailHost | STRING | Email host |
| user.emailTld | STRING | Email TLD |
| user.length | INTEGER | Username length |
| user.type | ENUM | User type, it can be "EMAIL", "CPF", "USERNAME", "PHONE" or "CNPJ" |

### **Notes:**

* Date fields support formats yyyy-MM-dd and yyyy-MM-ddTHH:mm:ss;  
* On date fields using yyyy-MM-ddT00:00:00 is equivalent to yyyy-MM-dd.  
* Boolean fields are represented as true, false, 1 (for true value) or 0 (for false value).  
* Fields are case-insensitive unless explicitly stated otherwise.  
* pageSizes are limited to 1000\.  
* The timezone parameter is optional and defaults to UTC.  
* The fields parameter is optional and defaults to all fields.  
* The order parameter is optional and defaults to desc.  
* The sortBy parameter is optional and defaults to created.  
* The page parameter is optional and defaults to 1\.  
* The pageSize parameter is optional and defaults to 50\.  
* The result of pageSize \* page should not exceed 1000000\.  
* Only date and integer fields are allowed in the sortBy parameter.  
* For user with access to multiple tenants, the customer parameter is required when searching for a child tenant's credentials.

## CSV Credential Extraction Example

Extract credentials into a CSV file, filtering by the desired criteria.

**Python**

This example uses Python 3.12 and standard authentication methods.

```py
#!/bin/bash
import csv
import json
import logging
import time
from logging import Logger

import requests

QUERY: str = "status=NEW,DISCARDED&user=contains:admin&sortBy=created&page=1&pageSize=50&order=asc&timezone=-07:00"
BASE_URL: str = f"https://api.axur.com/gateway/1.0/api/exposure-api/credentials?{QUERY}"
API_KEY: str = "<API_KEY>"
MAX_RETRIES: int = 3
INITIAL_WAIT: float = 1.0
WAIT_INCREMENT: float = 0.5

logging.basicConfig(format='%(asctime)s %(levelname)s %(name)s.%(funcName)s - %(message)s', level=logging.INFO)
LOGGER: Logger = logging.getLogger(__name__)


def make_request(retries: int = 0, wait_time: float = INITIAL_WAIT) -> list[dict[str, any]] | None:
    headers = get_headers(API_KEY)

    response = requests.get(BASE_URL, headers=headers)
    return process_response(response, retries, wait_time)


def get_headers(api_key: str) -> dict[str, str]:
    out_header = {"Content-Type": "application/json"}

    if api_key:
        out_header["Authorization"] = str.format("Bearer {}", api_key)

    return out_header


def process_response(response: requests.Response, retries: int, wait_time: float) -> list[dict[str, any]] | None:
    if 200 <= response.status_code < 300:
        if response.text:
            return json.loads(response.text)["detections"]
    elif response.status_code == 429:
        if retries >= MAX_RETRIES:
            response.raise_for_status()
        retries += 1
        time.sleep(wait_time)
        wait_time += WAIT_INCREMENT
        return make_request(retries, wait_time)
    else:
        response.raise_for_status()


def write_to_csv(data: list[dict[str, any]], filename: str) -> None:
    if not data:
        LOGGER.warning("No data to write.")
        return

    with open(filename, mode='w', newline='', encoding='utf-8') as file:
        fieldnames: set[str] = set([key for item in data for key in item.keys()])
        writer = csv.DictWriter(file, fieldnames=fieldnames)
        writer.writeheader()
        for item in data:
            writer.writerow(item)


def main() -> None:
    response = make_request()
    write_to_csv(response, "data.csv")


if __name__ == "__main__":
    main()
```

**Bash cURL**

Use a bash script to extract credential details:

```shell
#!/bin/bash

API_KEY='<API_KEY>'
BASE_URL='https://api.axur.com/gateway/1.0/api/exposure-api/credentials'

ONE_HOUR_AGO=$(TZ='America/Sao_Paulo' date --date='-24 hour' +"%Y-%m-%dT%H:%M:%S")
CURRENT_TIME=$(TZ='America/Sao_Paulo' date +"%Y-%m-%dT%H:%M:%S")
SORT_FIELD="created"
PAGES=1
DETECTIONS_PER_PAGE=50
TIMEZONE='-03:00'
DESIRED_FIELDS='id,status,user,password,created,assets,password.hasLetter,password.length'

mkdir -p ./detections

sleep 3

GETQ=$(curl --noproxy "*" \
-sLX GET "${BASE_URL}?created=ge:${ONE_HOUR_AGO}&created=le:${CURRENT_TIME}&sortBy=${SORT_FIELD}&page=${PAGES}&pageSize=${DETECTIONS_PER_PAGE}&timezone=${TIMEZONE}&fields=${DESIRED_FIELDS}" \
-H "Authorization: Bearer ${API_KEY}")

# Check if the response is a valid JSON
if echo "$GETQ" | jq . > /dev/null 2>&1; then
    echo "$GETQ" | tr -d '\000-\037' | jq -c '.detections[]' | while read -r detection; do
        id=$(echo "$detection" | jq -r '.id')
        echo "$detection" | jq '.' > "detections/${id}.json"
    done
else
    echo "Error: The server response is not a valid JSON."
fi
```

## Retrieve Credential Detections

**Permission Level Needed:** All permission levels

Returns credentials detections matching the specified filter.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### query Parameters

| created | string Example: created=ge:2024-07-01 A filter to retrieve credential detections created on or after the specified date in ISO 8601 format (e.g., 'yyyy-MM-dd'). |
| :---- | :---- |
| sortBy | string Default: "created" Example: sortBy=created A field to specify the attribute by which the results will be sorted. |
| page | integer Default: 1 Example: page=1 The page number to retrieve in the paginated response. Defaults to 1 if not specified. |
| pageSize | integer Default: 50 Example: pageSize=2 The number of results displayed per page in the paginated response. Defaults to 50 if not specified. |
| order | string Default: "desc" Enum: "desc" "asc" Example: order=desc The order in which the results are sorted. Accepts 'asc' for ascending order and 'desc' for descending order. Defaults to 'desc'. |
| timezone | string Default: "00:00" Example: timezone=-03:00 Use one of the records identified in the UTF Offset column on [https://en.wikipedia.org/wiki/Time\_zone\#List\_of\_UTC\_offsets](https://en.wikipedia.org/wiki/Time_zone#List_of_UTC_offsets). The UTC offset will be used to convert the date input via query parameters and the date output via json. |

### Responses

**200** OK

**400** Incorrect Query Parameters

**403** FORBIDDEN

**429** Rate Limit exceeded

get/exposure-api/credentials

### **Request samples**

* **Python 3.12**  
* **Bash cURL**

Copy  
\#Remember to replace the 'query' params and 'token' fields.  
import requests

query \= 'created=ge:2024-07-01\&sortBy=created\&page=1\&pageSize=2\&order=desc\&timezone=-03:00'  
token \= '\<API\_KEY\>'

PATH \= f'https://api.axur.com/gateway/1.0/api/exposure-api/credentials?{query}'

header \= {  
    'Content-Type': 'application/json',  
    'Authorization': str.format("Bearer {}", token)  
}

result \= requests.get(PATH, headers\=header)

print(result.json())

### **Response samples**

* **200**  
* **400**  
* **403**  
* **429**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"detections": [`  
  * `{},`  
  * `{}`  
* `],`  
* `"pageable": {`  
  * `"pageNumber": 1,`  
  * `"pageSize": 2,`  
  * `"total": 1200`  
* `}`

`}`

# Credential Update Operations

This section describes the available operations for updating credential detections, including status changes and tag management.

## Status Update Operations

### **Update Single Detection Status**

Update the status of a specific detection by providing its ID:

```
PATCH https://api.axur.com/gateway/1.0/api/exposure-api/credentials/:id
```

**Request Body:**

```json
{
  "field": "status",
  "value": "DISCARDED"
}
```

### **Bulk Update Detection Status**

Update the status of multiple detections simultaneously:

```
PATCH https://api.axur.com/gateway/1.0/api/exposure-api/credentials
```

**Request Body:**

```json
{
  "field": "status",
  "value": "DISCARDED",
  "ids": [
    "202411271319565002EF35C72F8C5C298",
    "202411271319565002EF35C72F8C5C299"
  ]
}
```

**Response:** All status update operations return HTTP STATUS 204 NO CONTENT.

---

## Tag Management Operations

### **Single Detection Tag Management**

#### **Add Tags to a Single Detection**

Add new tags to a specific detection:

```
POST https://api.axur.com/gateway/1.0/api/exposure-api/credentials/:id/tags
```

**Request Body:**

```json
{
  "values": [
    "my tag",
    "tag2"
  ]
}
```

#### **Remove Tags from a Single Detection**

Remove specific tags from a detection:

```
POST https://api.axur.com/gateway/1.0/api/exposure-api/credentials/delete/:id/tags
```

**Request Body:**

```json
{
  "values": [
    "tag3",
    "tag4"
  ]
}
```

### **Bulk Tag Management**

#### **Add Tags to Multiple Detections**

Add tags to several detections at once:

```
POST https://api.axur.com/gateway/1.0/api/exposure-api/credentials/tags
```

**Request Body:**

```json
{
  "values": [
    "my tag",
    "tag2"
  ],
  "ids": [
    "202411271319565002EF35C72F8C5C298",
    "202411271319565002EF35C72F8C5C299"
  ]
}
```

#### **Remove Tags from Multiple Detections**

Remove tags from several detections simultaneously:

```
POST https://api.axur.com/gateway/1.0/api/exposure-api/credentials/delete/tags
```

**Request Body:**

```json
{
  "values": [
    "tag3",
    "tag4"
  ],
  "ids": [
    "202411271319565002EF35C72F8C5C298",
    "202411271319565002EF35C72F8C5C299"
  ]
}
```

**Response:** All tag operations return HTTP STATUS 204 NO CONTENT.

---

## Multi-Tenant Support (MSSP)

For users with access to multiple tenants, include the customer parameter when managing a child tenant's credentials:

### **Status Update Example (MSSP)**

```json
{
  "field": "status",
  "value": "DISCARDED",
  "ids": [
    "202411271319565002EF35C72F8C5C298",
    "202411271319565002EF35C72F8C5C299"
  ],
  "customer": "CHILD_KEY"
}
```

### **Tag Management Example (MSSP)**

```json
{
  "values": [
    "my tag",
    "tag2"
  ],
  "ids": [
    "202411271319565002EF35C72F8C5C298",
    "202411271319565002EF35C72F8C5C299"
  ],
  "customer": "CHILD_KEY"
}
```

---

## Supported Fields

| Field | Type | Operations | Notes |
| :---- | :---- | :---- | :---- |
| status | ENUM | PATCH only | Detection status: "NEW", "IN\_TREATMENT", "SOLVED", or "DISCARDED" |
| tags | LIST | POST only | Custom labels for organizing detections. Tags are case-sensitive. |

---

## Important Notes

### **General Guidelines**

* **Maximum bulk operations:** 1000 IDs per request  
* **Field case sensitivity:** Fields are case-insensitive unless explicitly stated otherwise  
* **Detection ID retrieval:** Use the [Credential Search Operations](https://docs.axur.com/en/axur/api/#section/Examples-of-search-usage) to obtain detection IDs

### **Method-Specific Rules**

* **PATCH operations:** Only the status field is supported  
* **POST/DELETE operations:** Only the tags field is currently supported  
* **Field parameter:** The :field URL parameter currently supports only tags, but may be extended in the future

### **Multi-Tenant Requirements**

* **MSSP users:** Must include the customer parameter when managing child tenant credentials  
* **Applies to:** All update operations (PATCH, POST)  
* **Customer value:** Use the child tenant's customer key (e.g., "CHILD\_KEY")

### **Response Codes**

* **Success:** 204 NO CONTENT for all successful operations  
* **Error codes:** 400 (Bad Request), 429 (Rate Limit Exceeded)

## Bulk Update Credential Detection

**Permission Level Needed:** All permission levels

Update the detection field with the specified ids.

* *To obtain id, the 'Credential Search Operations' section must be consulted (see [Get Credential Search Operations](https://docs.axur.com/en/axur/api/#section/Examples-of-search-usage)).*

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### header Parameters

| Content-Type | string Example: application/json |
| :---- | :---- |

##### Request Body schema: application/json

| field required | string |
| :---- | :---- |
| value required | string |
| ids required | Array of strings |
| customer | string Customer key (required for MSSP users when managing child tenant's credentials) |

### Responses

**204** NO CONTENT

**400** Incorrect Query Parameters or Body

**429** Rate Limit exceeded

patch/exposure-api/credentials

### **Request samples**

* **Payload**  
* **Python 3.12**  
* **Bash cURL**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"field": "status",`  
* `"value": "DISCARDED",`  
* `"ids": [`  
  * `"202411271319565002EF35C72F8C5C298",`  
  * `"202411271319565002EF35C72F8C5C299"`  
* `],`  
* `"customer": "CHILD_KEY"`

`}`

### **Response samples**

* **204**  
* **400**  
* **429**

Content type  
text/plain

Copy

## Update Credential Detection

**Permission Level Needed:** All permission levels

Update the detection field with the specified id.

* *To obtain id, the 'Credential Search Operations' section must be consulted (see [Get Credential Search Operations](https://docs.axur.com/en/axur/api/#section/Examples-of-search-usage)).*

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| id required | string Example: 202411271319565002EF35C72F8C5C298 detection id |
| :---- | :---- |

##### header Parameters

| Content-Type | string Example: application/json |
| :---- | :---- |

##### Request Body schema: application/json

| field required | string |
| :---- | :---- |
| value required | string |
| customer | string Customer key (required for MSSP users when managing child tenant's credentials) |

### Responses

**204** NO CONTENT

**400** Incorrect Query Parameters or Body

**429** Rate Limit exceeded

patch/exposure-api/credentials/{id}

### **Request samples**

* **Payload**  
* **Python 3.12**  
* **Bash cURL**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"field": "status",`  
* `"value": "DISCARDED",`  
* `"customer": "CHILD_KEY"`

`}`

### **Response samples**

* **204**  
* **400**  
* **429**

Content type  
text/plain

Copy

## Add Field Data to Credential Detection

**Permission Level Needed:** All permission levels

Add field data to the detection with the specified id and field.

* *To obtain id, the 'Credential Search Operations' section must be consulted (see [Get Credential Search Operations](https://docs.axur.com/en/axur/api/#section/Examples-of-search-usage)).*  
* *Currently only 'tags' field is supported.*

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| id required | string Example: 202411271319565002EF35C72F8C5C298 detection id |
| :---- | :---- |
| field required | string Example: tags field name (currently only 'tags' is supported) |

##### header Parameters

| Content-Type | string Example: application/json |
| :---- | :---- |

##### Request Body schema: application/json

| values required | Array of strings |
| :---- | :---- |
| customer | string Customer key (required for MSSP users when managing child tenant's credentials) |

### Responses

**204** NO CONTENT

**400** Incorrect Query Parameters or Body

**429** Rate Limit exceeded

post/exposure-api/credentials/{id}/{field}

### **Request samples**

* **Payload**  
* **Python 3.12**  
* **Bash cURL**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"values": [`  
  * `"my tag",`  
  * `"tag2"`  
* `],`  
* `"customer": "CHILD_KEY"`

`}`

### **Response samples**

* **204**  
* **400**  
* **429**

Content type  
text/plain

Copy

## Remove Field Data from Credential Detection

**Permission Level Needed:** All permission levels

Remove field data from the detection with the specified id and field.

* *To obtain id, the 'Credential Search Operations' section must be consulted (see [Get Credential Search Operations](https://docs.axur.com/en/axur/api/#section/Examples-of-search-usage)).*  
* *Currently only 'tags' field is supported.*

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| id required | string Example: 202411271319565002EF35C72F8C5C298 detection id |
| :---- | :---- |
| field required | string Example: tags field name (currently only 'tags' is supported) |

##### header Parameters

| Content-Type | string Example: application/json |
| :---- | :---- |

##### Request Body schema: application/json

| values required | Array of strings |
| :---- | :---- |
| customer | string Customer key (required for MSSP users when managing child tenant's credentials) |

### Responses

**204** NO CONTENT

**400** Incorrect Query Parameters or Body

**429** Rate Limit exceeded

post/exposure-api/credentials/delete/{id}/{field}

### **Request samples**

* **Payload**  
* **Python 3.12**  
* **Bash cURL**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"values": [`  
  * `"my tag",`  
  * `"tag2"`  
* `],`  
* `"customer": "CHILD_KEY"`

`}`

### **Response samples**

* **204**  
* **400**  
* **429**

Content type  
text/plain

Copy

## Bulk Add Field Data to Credential Detections

**Permission Level Needed:** All permission levels

Add field data to multiple detections with the specified field.

* *To obtain ids, the 'Credential Search Operations' section must be consulted (see [Get Credential Search Operations](https://docs.axur.com/en/axur/api/#section/Examples-of-search-usage)).*  
* *Currently only 'tags' field is supported.*

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| field required | string Example: tags field name (currently only 'tags' is supported) |
| :---- | :---- |

##### header Parameters

| Content-Type | string Example: application/json |
| :---- | :---- |

##### Request Body schema: application/json

| values required | Array of strings |
| :---- | :---- |
| ids required | Array of strings |
| customer | string Customer key (required for MSSP users when managing child tenant's credentials) |

### Responses

**204** NO CONTENT

**400** Incorrect Query Parameters or Body

**429** Rate Limit exceeded

post/exposure-api/credentials/{field}

### **Request samples**

* **Payload**  
* **Python 3.12**  
* **Bash cURL**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"values": [`  
  * `"my tag",`  
  * `"tag2"`  
* `],`  
* `"ids": [`  
  * `"202411271319565002EF35C72F8C5C298",`  
  * `"202411271319565002EF35C72F8C5C299"`  
* `],`  
* `"customer": "CHILD_KEY"`

`}`

### **Response samples**

* **204**  
* **400**  
* **429**

Content type  
text/plain

Copy

## Bulk Remove Field Data from Credential Detections

**Permission Level Needed:** All permission levels

Remove field data from multiple detections with the specified field.

* *To obtain ids, the 'Credential Search Operations' section must be consulted (see [Get Credential Search Operations](https://docs.axur.com/en/axur/api/#section/Examples-of-search-usage)).*  
* *Currently only 'tags' field is supported.*

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| field required | string Example: tags field name (currently only 'tags' is supported) |
| :---- | :---- |

##### header Parameters

| Content-Type | string Example: application/json |
| :---- | :---- |

##### Request Body schema: application/json

| values required | Array of strings |
| :---- | :---- |
| ids required | Array of strings |
| customer | string Customer key (required for MSSP users when managing child tenant's credentials) |

### Responses

**204** NO CONTENT

**400** Incorrect Query Parameters or Body

**429** Rate Limit exceeded

post/exposure-api/credentials/delete/{field}

### **Request samples**

* **Payload**  
* **Python 3.12**  
* **Bash cURL**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"values": [`  
  * `"my tag",`  
  * `"tag2"`  
* `],`  
* `"ids": [`  
  * `"202411271319565002EF35C72F8C5C298",`  
  * `"202411271319565002EF35C72F8C5C299"`  
* `],`  
* `"customer": "CHILD_KEY"`

`}`

### **Response samples**

* **204**  
* **400**  
* **429**

Content type  
text/plain

Copy

# Credential Count Operations

## Examples of count usage

Count credentials detected with specific criteria, such as their status, date offset, and using the Brazil timezone:

```
GET https://api.axur.com/gateway/1.0/api/exposure-api/credentials/total?status=NEW&timezone=-03:00&created=ge:2024-01-01&created=le:2024-01-25
```

The response for the request above will be HTTP STATUS 200 OK with the following body:

```json
{
  "total": 1234
}
```

## Fields supported by exposure API (count)

All the fields and operations used for [search](https://docs.axur.com/en/axur/api/#section/Fields-supported-by-exposure-API) can also be used in the count API.

### **Notes:**

* Date fields support formats yyyy-MM-dd and yyyy-MM-ddTHH:mm:ss;  
* On date fields using yyyy-MM-ddT00:00:00 is equivalent to yyyy-MM-dd.  
* Boolean fields are represented as true, false, 1 (for true value) or 0 (for false value).  
* Fields are case-insensitive unless explicitly stated otherwise.  
* The timezone parameter is optional and defaults to UTC.  
* For user with access to multiple tenants, the customer parameter is required when searching for a child tenant's credentials.

## Count Credentials

**Permission Level Needed:** All permission levels

Returns total of credentials detections that match the specified filters.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### query Parameters

| created | string Example: created=ge:2024-07-01T20:18:36 The date and time the credentials were created, in ISO 8601 format (e.g., yyyy-MM-ddTHH:mm:ss). This parameter is optional and may be used to filter results based on the creation date. |
| :---- | :---- |
| timezone | string Default: "00:00" Example: timezone=-03:00 Use one of the records identified in the UTF Offset column on [https://en.wikipedia.org/wiki/Time\_zone\#List\_of\_UTC\_offsets](https://en.wikipedia.org/wiki/Time_zone#List_of_UTC_offsets). The UTC offset will be used to convert the date input via query parameters and the date output via json. |

### Responses

**200** OK

**400** Incorrect Query Parameters

**403** FORBIDDEN

**429** Rate Limit exceeded

get/exposure-api/credentials/total

### **Request samples**

* **Python 3.12**  
* **Bash cURL**

Copy  
\#Remember to replace the 'query' params and 'token' fields.  
import requests

query \= 'created=ge:2024-07-01T20:18:36\&timezone=-03:00'  
token \= '\<API\_KEY\>'

PATH \= f'https://api.axur.com/gateway/1.0/api/exposure-api/credentials/total?{query}'

header \= {  
    'Content-Type': 'application/json',  
    'Authorization': str.format("Bearer {}", token)  
}

result \= requests.get(PATH, headers\=header)

print(result.json())

### **Response samples**

* **200**  
* **400**  
* **403**  
* **429**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"total": 42`

`}`

# Web complaints API

Provides operations to retrieve web complaint data reported by customers. Use this API to query complaints based on date ranges and other criteria.

## Get web complaints

API for querying web complaints that retrieves a paginated list based on filter criteria, with authentication performed using api-key sent in the Authorization header.

## Request Examples

### **cURL**

Example of a request using cURL in the terminal.

```shell
curl -X GET "https://api.axur.com/gateway/1.0/api/web-complaints/results?initialDate=2022-01-01&finalDate=2025-09-09&order=desc&timezone=-3&page=1&pageSize=200"      -H "Authorization: Bearer <API-KEY>"
```

### **Python**

Example of a request using the Python requests library.

```py
import requests
import json

base_path = "https://api.axur.com/gateway/1.0/api/web-complaints/results"
api_key = "API-KEY"

headers = {
"Authorization": f"Bearer {api_key}"
}

params = {
    "initialDate": "2022-01-01",
    "finalDate": "2025-09-09",
    "order": "desc",
    "timezone": "-03:00",
    "page": 1,
    "pageSize": 200
    }

try:
  response = requests.get(base_path, headers=headers, params=params)
  response.raise_for_status()

  print(f"Status Code: {response.status_code}")
  print("Response JSON:")
  print(json.dumps(response.json(), indent=2))

except requests.exceptions.HTTPError as http_err:
  print(f"HTTP error occurred: {http_err}")
  print(f"Response Content: {response.text}")
except requests.exceptions.RequestException as req_err:
  print(f"An error occurred: {req_err}")
```

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### query Parameters

| initialDate required | string \<date\> Example: initialDate=2021-01-01 Mandatory filter. Date in format yyyy-MM-dd. |
| :---- | :---- |
| finalDate | string \<date\> Example: finalDate=2025-09-23 Optional filter. To include data up to the current day, leave it empty or provide a date equal to or later than today. |
| order | string Default: "desc" Enum: "asc" "desc" Optional filter. Use 'asc' or 'desc' to sort the response by creation date. |
| timezone | string Example: timezone=-03:00 Optional filter. Time zone used to compose the response. Default is Z (UTC). |
| page | integer Default: 1 Optional filter. The page number of the content. |
| pageSize | integer \[ 1 .. 200 \] Default: 200 Optional filter. The size of the response page, with allowed values between 1 and 200\. |

##### header Parameters

| Authorization required | string Mandatory. The api-key for authentication. |
| :---- | :---- |

### Responses

**200** A paginated list of complaints.

**401** Unauthorized. The authorization token was not provided or is invalid.

**429** Too Many Requests — this endpoint allows up to 3 requests per second.

get/web-complaints/complaints

### **Response samples**

* **200**  
* **401**  
* **429**

Content type  
application/json

Example

A successful response with a list of complaints

Copy

Expand allCollapse all

`{`

* `"content": [`  
  * `{},`  
  * `{}`  
* `],`  
* `"pageNumber": 2,`  
* `"pageSize": 200,`  
* `"totalElements": 435,`  
* `"previousPage": "https://api.axur.com/gateway/1.0/web-complaints/complaints?initialDate=2021-01-01&page=1",`  
* `"nextPage": "https://api.axur.com/gateway/1.0/web-complaints/complaints?initialDate=2021-01-01&page=3"`

`}`

# Credit Card Exposure for Application

Credit Card Exposure for Application is responsible for monitoring internet activities in search of leaked credit cards.

Axur provides an HTTP API for Credit Card Exposure for Application, allowing external applications, like ecommerces and payment gateways, to access information programmatically. Therefore, ecommerces can check if a customer's credit card has been compromised.

This document describes the currently supported API operations.

# Getting started with Credit Card Exposure for Application

For you to have access to the API you must hire a plan with our sales team. We also have free trials and Proof of Concepts.

The API has a rate limit per IP Address of maximal **120 request per minute**

To make an API query, you must generate a sha256 hash with the desired credit card number.

# Supported operations on Credit Card Exposure for Application

## Exists Leak

Check if a credit card has leaks.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| hash required | string Example: 744807d6af484cebf6f3138ee8d504340eed54daa71630aeebe5174f1b775429 sha256 hash with the desired credit card number |
| :---- | :---- |

##### query Parameters

| incomplete | boolean Example: incomplete=true Flag to check incomplete cards, detected without expiration and/or CVV (default is false) |
| :---- | :---- |

### Responses

**200** OK

**204** No results

**403** Forbidden

get/cardstream/{hash}

### **Request samples**

* **Python 3.7**  
* **Bash cURL**

Copy  
import requests

PATH \= 'https://api.axur.com/gateway/1.0/api/cardstream/{}'  
token \= '\<token\>'  
card\_hash \= '\<sha256 hash of card number\>'

headers \= {  
        'Content-Type': 'application/json',  
        'Authorization': 'Bearer {}'.format(token)  
}

url \= PATH.format(card\_hash)  
result \= requests.get(url, headers\=headers)  
print(result.json())

### **Response samples**

* **200**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"leaked": true`

`}`

## Find Leaks

List all leaks for a credit card.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| hash required | string Example: 744807d6af484cebf6f3138ee8d504340eed54daa71630aeebe5174f1b775429 sha256 hash with the desired credit card number |
| :---- | :---- |

##### query Parameters

| incomplete | boolean Example: incomplete=true Flag to return incomplete cards, detected without expiration and/or CVV (default is false) |
| :---- | :---- |

### Responses

**200** OK

**204** No results

**403** Forbidden

get/cardstream/{hash}/leaks

### **Request samples**

* **Python 3.7**  
* **Bash cURL**

Copy  
import requests

PATH \= 'https://api.axur.com/gateway/1.0/api/cardstream/{}/leaks'  
token \= '\<token\>'  
card\_hash \= '\<sha256 hash of card number\>'

headers \= {  
        'Content-Type': 'application/json',  
        'Authorization': 'Bearer {}'.format(token)  
}

url \= PATH.format(card\_hash)  
result \= requests.get(url, headers\=headers)  
print(result.json())

### **Response samples**

* **200**

Content type  
application/json

Copy

Expand allCollapse all

`[`

* `{`  
  * `"cc": 1111110000000000,`  
  * `"detectedAt": 1674840216,`  
  * `"expiryAt": "10/27"`  
* `}`

`]`

## Check Leak

Check if a credit card has leaks with low latency for South American customers.

This API endpoint has a rate limit of **1,150 requests per minute** per IP address.

**Important:** This endpoint is unavailable for European customers.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| hash required | string Example: 744807d6af484cebf6f3138ee8d504340eed54daa71630aeebe5174f1b775429 sha256 hash with the desired credit card number |
| :---- | :---- |

### Responses

**200** Credit card leak status.

**400** Invalid request due to bad input format.

**403** Forbidden

get/cardhash/check/{hash}

### **Request samples**

* **Python 3.7**  
* **Bash cURL**

Copy  
import requests

PATH \= 'https://sa.api.axur.com/gateway/1.0/api/cardhash/check/{}'  
token \= '\<token\>'  
card\_hash \= '\<sha256 hash of card number\>'

headers \= {  
        'Content-Type': 'application/json',  
        'Authorization': 'Bearer {}'.format(token)  
}

url \= PATH.format(card\_hash)  
result \= requests.get(url, headers\=headers)  
print(result.json())

### **Response samples**

* **200**  
* **400**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"hash": "744807d6af484cebf6f3138ee8d504340eed54daa71630aeebe5174f1b775429",`  
* `"exists": true`

`}`

# Credit Card Exposure for Issuers

Credit Card Exposure for Issuers is responsible for monitoring internet activity in search of exposed credit cards. Axur provides an HTTP API and Webhooks for Credit Card Exposure for Issuers, allowing external applications, such as credit card issuers, to access information programmatically. Therefore, issuers can check whether their customer's credit card has been compromised in real time. This document describes the currently supported API operations and webhook notifications.

# Getting started with Credit Card Exposure for Issuers

* For you to have access to the API you must hire a plan with our sales team. We also have Proof of Concepts.  
* For your safety you can provide us a list with the IP addresses that will consume the API.  
* Once you are registered as a customer, you will need to generate API keys for authentication (see [Authentication](https://docs.axur.com/en/axur/api/#tag/Authentication))  
* To query the API, you can use the argument from to specify a start period for the leaks found and the argument cvv to choose whether or not you want to receive the cvv value.  
* Results are limited to the latest 2500 detections per request, if your search has more results an optional **page** argument can be used to get the next detections.  
* You can request our operations team to receive incomplete credit cards that doesn't have the expiration date and/or cvv.

# Webhooks for Credit Card Exposure for Issuers

Webhooks are an advanced API feature that allows external applications to receive Axur events almost in real time, as they happen, through HTTP or HTTPS. To be able to receive webhook events, the external application must provide an unauthenticated HTTP or HTTPS endpoint accessible from the Internet. This endpoint registration process is submited by our Data Leakage team. You might require this action throught our Technical Support team (at help@axur.com or [https://help.axur.com/en/](https://help.axur.com/en/)).

Here is an example payload sent by Axur:

```json
{
  "cardNumber": 1234567891011121,
  "bin": 123456,
  "bin8": 12345678,
  "securityCode": 123,
  "detectedAt": 1689270290000,
  "expirationDate": {
    "month": 9,
    "year": 2032
  },
  "source": {
    "name": "Deep/Dark Web",
    "url": "Telegram",
    "sourceType": "Deep/Dark Web - Telegram"
  }
}
```

## Webhook Signature

Since the provided endpoint must be freely accessible from the Internet, the external application needs to verify the authenticity of incoming Axur requests. The body of every published webhook event is signed using HMAC-SHA256 with a shared secret. Each customer has a unique secret that will be shared when registering a Webhook with out Technical Support team.

The signature's hexadecimal value is included in the custom HTTP header X-Axur-Signature.

**Python**

Signature verification function example for python 3.11+:

```py
import hmac
import hashlib

def is_signature_valid(content: str, signature: str, secret: str) -> bool:
    """
    Validates the HMAC SHA-256 signature.

    :param content: The original message payload.
    :param signature: The HMAC signature from X-Axur-Signature header.
    :param secret: The shared secret used to generate the signature.
    :return: True if the signature is valid, False otherwise.
    """
    computed_signature = hmac.new(secret.encode(), content.encode(), hashlib.sha256).hexdigest()
    return hmac.compare_digest(computed_signature, signature)
```

# Supported operations on Credit Card Exposure for Issuers

## Get exposed cards

Authorizations:

[apiKey](https://docs.axur.com/en/axur/api/#section/Authentication/apiKey)

##### query Parameters

| from required | integer Example: from=1694627090000 (milliseconds) |
| :---- | :---- |
| cvv required | boolean Example: cvv=true |
| page | integer Example: page=1 |

### Responses

**200** OK

**402** Bad Request

**500** Internal Server Error

get/cardcast/credit-card-exposures

### **Response samples**

* **200**  
* **402**  
* **500**

Content type  
application/json

Copy

Expand allCollapse all

`[`

* `{`  
  * `"cardNumber": 1234567891011121,`  
  * `"bin": 123456,`  
  * `"bin8": 12345678,`  
  * `"securityCode": 123,`  
  * `"detectedAt": 1689270290000,`  
  * `"expirationDate": {},`  
  * `"source": {}`  
* `},`  
* `{`  
  * `"cardNumber": 4957030420210454,`  
  * `"bin": 495703,`  
  * `"bin8": 49570304,`  
  * `"securityCode": 321,`  
  * `"detectedAt": 1689270290000,`  
  * `"expirationDate": {},`  
  * `"source": {}`  
* `}`

`]`

# Threat & Exposure Intelligence TAXII Server

This section outlines the supported API operations within the Threat & Exposure Intelligence TAXII Server.

This API was developed following TAXII and STIX 2.1 specifications. All endpoints, headers, requests, responses, and filters are defined by this specification. The official documentation of TAXII and STIX can be found here: [https://oasis-open.github.io/cti-documentation/resources.html\#taxii-21-specification](https://oasis-open.github.io/cti-documentation/resources.html#taxii-21-specification)

# Supported operations on Threat & Exposure Intelligence TAXII Server

## TAXII Server Discovery

**Permission Level Needed:** None

This Endpoint provides general information about a TAXII Server, including the advertised API Roots. It's a common entry point for TAXII Clients into the data and services provided by a TAXII Server.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

### Responses

**200** OK

get/cyber-stix/taxii2/

### **Request samples**

* **Python 3.12**

Copy  
import requests

result \= requests.get('https://api.axur.com/gateway/1.0/api/cyber-stix/taxii2/')

print(result.json())

### **Response samples**

* **200**

Content type  
application/taxii+json;version=2.1

Copy

Expand allCollapse all

`{`

* `"title": "Axur TAXII server",`  
* `"description": "Server description",`  
* `"api_roots": [`  
  * `[]`  
* `]`

`}`

## Retrieve API Root

**Permission Level Needed:** All users

This Endpoint provides general information about an API Root, which can be used to help users and clients decide whether and how they want to interact with it.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| api-root-id required | string Example: polaris |
| :---- | :---- |

### Responses

**200** OK

get/cyber-stix/{api-root-id}/

### **Request samples**

* **Python 3.12**

Copy  
\# Remember to replace with the discovery api root id  
import requests

token \= '\<API\_KEY\>'  
api\_root \= 'polaris'

path \= f'https://api.axur.com/gateway/1.0/api/cyber-stix/{api\_root}/'

header \= {  
    'Authorization': f'Bearer {token}'  
}

result \= requests.get(path, headers\=header)

print(result.json())

### **Response samples**

* **200**

Content type  
application/taxii+json;version=2.1

Copy

Expand allCollapse all

`{`

* `"title": "API Root",`  
* `"description": "API Root description",`  
* `"versions": "application/taxii+json;version=2.1",`  
* `"max_content_length": 1`

`}`

## Retrieve API Root Collections

**Permission Level Needed:** All users

This Endpoint provides information about the Collections hosted under this API Root.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| api-root-id required | string Example: polaris |
| :---- | :---- |

### Responses

**200** OK

get/cyber-stix/{api-root-id}/collections/

### **Request samples**

* **Python 3.12**

Copy  
\# Remember to replace with the discovery api root id  
import requests

token \= '\<API\_KEY\>'  
api\_root \= 'polaris'

path \= f'https://api.axur.com/gateway/1.0/api/cyber-stix/{api\_root}/collections/'

header \= {  
    'Authorization': f'Bearer {token}'  
}

result \= requests.get(path, headers\=header)

print(result.json())

### **Response samples**

* **200**

Content type  
application/taxii+json;version=2.1

Copy

Expand allCollapse all

`{`

* `"collections": [`  
  * `{}`  
* `]`

`}`

## Retrieve API Root Collection by Id

**Permission Level Needed:** All users

This Endpoint provides general information about a Collection, which can be used to help users and clients decide whether and how they want to interact with it.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| api-root-id required | string Example: polaris |
| :---- | :---- |
| collection-id required | string |

### Responses

**200** OK

get/cyber-stix/{api-root-id}/collections/{collection-id}/

### **Request samples**

* **Python 3.12**

Copy  
\# Remember to replace with the discovery api root and collection ids  
import requests

token \= '\<API\_KEY\>'  
api\_root \= 'polaris'  
collection\_id \= 'anId'

path \= f'https://api.axur.com/gateway/1.0/api/cyber-stix/{api\_root}/collections/{collection\_id}/'

header \= {  
    'Authorization': f'Bearer {token}'  
}

result \= requests.get(path, headers\=header)

print(result.json())

### **Response samples**

* **200**

Content type  
application/taxii+json;version=2.1

Copy

Expand allCollapse all

`{`

* `"id": "collectionId",`  
* `"title": "title",`  
* `"description": "description",`  
* `"can_read": true,`  
* `"can_write": false,`  
* `"media_types": [`  
  * `"application/taxii+json;version=2.1"`  
* `]`

`}`

## Retrieve Collection Manifest

**Permission Level Needed:** All users

This Endpoint retrieves a manifest about the objects in a Collection.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| api-root-id required | string Example: polaris |
| :---- | :---- |
| collection-id required | string |

##### query Parameters

| added\_after | string Example: added\_after=2016-03-23T01:01:01.000Z |
| :---- | :---- |
| limit | number Example: limit=10 |
| next | number Example: next=10 |
| match\[\<id\>\] | string Example: match\[\<id\>\]=id-1,id-2,id-3 |
| match\[\<spec\_version\>\] | string Example: match\[\<spec\_version\>\]=2.0,2.1 |
| match\[\<type\>\] | string Example: match\[\<type\>\]=bundle |
| match\[\<version\>\] | string Example: match\[\<version\>\]=2016-03-23T01:01:01.000Z |

### Responses

**200** OK

get/cyber-stix/{api-root-id}/collections/{collection-id}/manifest/

### **Response samples**

* **200**

Content type  
application/taxii+json;version=2.1

Copy

Expand allCollapse all

`{`

* `"more": true,`  
* `"next": 2,`  
* `"objects": [`  
  * `{}`  
* `]`

`}`

## Retrieve Collection STIX Objects

**Permission Level Needed:** All users

This Endpoint retrieves objects from a Collection. Clients can search for objects in the Collection, retrieve all objects in a Collection, or paginate through objects in the Collection.

Refer to the [documentation](https://docs.oasis-open.org/cti/stix/v2.1/csprd01/stix-v2.1-csprd01.html#_Toc16070617) for a detailed schema of STIX objects.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| api-root-id required | string Example: polaris |
| :---- | :---- |
| collection-id required | string |

##### query Parameters

| added\_after | string Example: added\_after=2016-03-23T01:01:01.000Z |
| :---- | :---- |
| limit | number Example: limit=10 |
| next | number Example: next=10 |
| match\[\<id\>\] | string Example: match\[\<id\>\]=id-1,id-2,id-3 |
| match\[\<spec\_version\>\] | string Example: match\[\<spec\_version\>\]=2.0,2.1 |
| match\[\<type\>\] | string Example: match\[\<type\>\]=bundle |
| match\[\<version\>\] | string Example: match\[\<version\>\]=2016-03-23T01:01:01.000Z |

### Responses

**200** OK

get/cyber-stix/{api-root-id}/collections/{collection-id}/objects/

### **Request samples**

* **taxii2client \+ BearerAuth**

Copy  
from taxii2client.v21 import Server, as\_pages  
from requests.auth import AuthBase

\# \=== Bearer authentication class \===  
class BearerAuth(AuthBase):  
    def \_\_init\_\_(self, token):  
        self.token \= token

    def \_\_call\_\_(self, r):  
        r.headers\["Authorization"\] \= f"Bearer {self.token}"  
        return r

auth \= BearerAuth('\<YOUR\_TOKEN\>')

server \= Server('https://api.axur.com/gateway/1.0/api/cyber-stix/taxii2/', auth\=auth)

print(f"Number of API Roots: {len(server.api\_roots)}")

\# Only the first API Root is used to avoid excessive load in production  
api\_root \= server.api\_roots\[0\]  
print(f"API Root URL: {api\_root.url}")  
print(f"Title: {api\_root.title}")  
print(f"Description: {api\_root.description}")  
print(f"Max Content Length: {api\_root.max\_content\_length}")

for collection in api\_root.collections:  
    print(f"\\nCollection: {collection.title}")  
    print(f"ID: {collection.id}")  
    print(f"Can Read: {collection.can\_read}")  
    print(f"Can Write: {collection.can\_write}")

    \# Only the first page is fetched with 1 object per request,  
    \# to keep the example fast and light in production environments.  
    envelope \= next(as\_pages(collection.get\_objects, per\_request\=1))  
    print(f"Number of objects returned: {len(envelope\['objects'\])}")

    for i, obj in enumerate(envelope\['objects'\], start\=1):  
      if obj\['type'\] \== 'bundle':  
        print(f"Number of objects in bundle {i}: {len(obj\['objects'\])}")  
        for j, bundle\_obj in enumerate(obj\['objects'\], start\=1):  
          print(f"Object {j}: \=============================")  
          print(bundle\_obj)  
        print()  
      else:  
        print(f"Object {i}: \=============================")  
        print(obj)

### **Response samples**

* **200**

Content type  
application/taxii+json;version=2.1

Copy

Expand allCollapse all

`{`

* `"more": true,`  
* `"next": 2,`  
* `"objects": [`  
  * `[]`  
* `]`

`}`

## Retrieve STIX Object by Id

**Permission Level Needed:** All users

This Endpoint gets an object from a Collection by its id. It can be thought of as a search where the match\[id\] parameter is set to the {object-id} in the path.

Refer to the [documentation](https://docs.oasis-open.org/cti/stix/v2.1/csprd01/stix-v2.1-csprd01.html#_Toc16070617) for a detailed schema of STIX objects.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| api-root-id required | string Example: polaris |
| :---- | :---- |
| collection-id required | string |
| object-id required | string Example: indicator--252c7c11-daf2-42bd-843b-be65edca9f61 |

##### query Parameters

| added\_after | string Example: added\_after=2016-03-23T01:01:01.000Z |
| :---- | :---- |
| limit | number Example: limit=10 |
| next | number Example: next=10 |
| match\[\<version\>\] | string Example: match\[\<version\>\]=2016-03-23T01:01:01.000Z |

### Responses

**200** OK

get/cyber-stix/{api-root-id}/collections/{collection-id}/objects/{object-id}/

### **Response samples**

* **200**

Content type  
application/taxii+json;version=2.1

Copy

Expand allCollapse all

`{`

* `"more": true,`  
* `"objects": [`  
  * `[]`  
* `]`

`}`

## Retrieve STIX Object Versions

**Permission Level Needed:** All users

This Endpoint retrieves a list of one or more versions of an object in a Collection.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| api-root-id required | string Example: polaris |
| :---- | :---- |
| collection-id required | string |
| object-id required | string Example: indicator--252c7c11-daf2-42bd-843b-be65edca9f61 |

### Responses

**200** OK

get/cyber-stix/{api-root-id}/collections/{collection-id}/objects/{object-id}/versions/

### **Response samples**

* **200**

Content type  
application/taxii+json;version=2.1

Copy

Expand allCollapse all

`{`

* `"versions": [`  
  * `"2024-05-28T17:37:47.799Z"`  
* `]`

`}`

# IoC Collection

This section introduces the **CTI IoC data collection** available on the Threat & Exposure Intelligence TAXII Server.  
The collection stores **Indicators of Compromise (IOCs)** in the form of STIX 2.1 Indicator objects.

All [supported TAXII operations](https://docs.axur.com/en/axur/api/#tag/Supported-operations-on-Threat-and-Exposure-Intelligence-TAXII-Server) (discovery, collection access, object retrieval, and filtering) apply to this IOC Collection.  
This allows clients to query, fetch, and manage indicators such as IP addresses, domain names, file hashes and other observables.

### **Example STIX Indicator Object**

```json
{
  "type": "indicator",
  "spec_version": "2.1",
  "id": "indicator--b7a8d32f-9c9a-4b1f-8e6e-1f82a5c7f4b5",
  "created": "2025-09-08T12:00:00.000Z",
  "modified": "2025-09-08T12:00:00.000Z",
  "name": "Suspicious IP address",
  "pattern": "[ipv4-addr:value = '166.0.184.127']",
  "pattern_type": "stix",
  "valid_from": "2025-09-08T12:00:00.000Z",
  "confidence": 15
}
```

# Threat Hunting

Threat Hunting is responsible for creating asynchronous searches on our data lake. You can query terabytes of data to find detections such as fraudulent websites, credit cards and credential leaks

For you to have access to the API you must hire a plan with our sales team.

# Supported operations on Threat Hunting

## Start Search

This endpoint is used to initiate an asynchronous search. After initiating the search, you will receive a search-id and use it to pool the search results and status. Idle searches will be automatically cancelled.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### Request Body schema: application/json

| query required | string For information on how to create queries, visit: [https://help.axur.com/en/articles/10542787-what-are-the-credential-search-parameters-of-threat-hunting](https://help.axur.com/en/articles/10542787-what-are-the-credential-search-parameters-of-threat-hunting) |
| :---- | :---- |
| source required | string signal-lake \- urls and domains, such as websites and the data it contains signal-lake-social-media \- profiles in social media platforms, such as Facebook signal-lake-ads \- ads in social media, such as Facebook credential \- login credentials from different platforms, such as email and password credit-card \- credit card data, such as bin and cvv chat-message \- messages in chat apps, such as Whatsapp, Telegram and Discord forum-message \- messages in deep web forums and ransomware feeds social-media-posts \- posts in social media, such as Twitter / X tokens \- chunks of text files where we identified tokens, such as urls, names, emails and SSNs |
| customer | string Search credits will be consumed from the customer informed here. If no customer is informed, credits will be consumed from the user's main customer. |

### Responses

**200** OK

**402** PAYMENT\_REQUIRED

post/threat-hunting-api/external-search

### **Request samples**

* **Payload**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"query": "emailDomain=\"acme.com\"",`  
* `"source": "credential",`  
* `"customer": "TEST"`

`}`

### **Response samples**

* **200**  
* **402**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"searchId": "5daeb8ae-dedb-4b87-b55d-9c26b00c612f"`

`}`

## Get Search Results

This endpoint is used to get the status of a search and the paginated results.

Credits are consumed everytime you hit this endpoint with results to be seen. If the page you requested has no results, or if the partial results are insufficiente for pagination, you will not be charged.

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| searchId required | string Example: 355ecaed-f1c7-4206-8ae8-f2c85bab3343 |
| :---- | :---- |

##### query Parameters

| page required | integer Example: page=1 |
| :---- | :---- |
| alias | boolean Example: alias=true used to apply human-readable aliases on the document's properties |

### Responses

**200** OK

**402** PAYMENT\_REQUIRED

get/threat-hunting-api/external-search/{searchId}

### **Response samples**

* **200**  
* **402**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"id": "b9fa46a6-9e3a-4b2f-8fec-0a4e9e937a6b",`  
* `"result": {`  
  * `"status": {},`  
  * `"data": [],`  
  * `"pagination": {}`  
* `}`

`}`

# Investigations

Investigations is responsible for managing CTI/ART Investigations, as well as their status and attachments.

For you to have access to the API you must hire a plan with our sales team.

# Supported operations on Investigations

## Create Investigation

**Permission Level Needed:** Experts/Managers

Create Investigation

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### Request Body schema: application/json

| customer | string customer key |
| :---- | :---- |
| asset | string asset associated with the investigation |
| relation | string Ticket url |
| title | string |
| details | string |
| description | string |
| recipients | Array of strings recipients emails |

### Responses

**200** OK

**400** Bad Request

**403** Forbidden

**405** Method Not Allowed

**500** Internal Server Error

post/investigations/investigation

### **Request samples**

* **Payload**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"customer": "string",`  
* `"asset": "string",`  
* `"relation": "https://one.axur.com/deep-dark-web/tickets/<ticketKey>",`  
* `"title": "string",`  
* `"details": "string",`  
* `"description": "string",`  
* `"recipients": [`  
  * `"string"`  
* `]`

`}`

### **Response samples**

* **200**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"id": 0`

`}`

## Get Investigations paginated

**Permission Level Needed:** Experts/Managers

Get Investigations paginated

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| customer\_key required | string Example: CSTMR\_KEY |
| :---- | :---- |

### Responses

**200** OK

**400** Bad Request

**403** Forbidden

**500** Internal Server Error

get/investigations/investigation/search/{customer\_key}

### **Response samples**

* **200**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"metadata": {`  
  * `"total": 10,`  
  * `"page": 2`  
* `},`  
* `"investigations": [`  
  * `{}`  
* `]`

`}`

## Upload attachment and add it to investigation

**Permission Level Needed:** Experts/Managers

Upload attachment and add it to investigation

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| investigation\_id required | integer Example: 123 |
| :---- | :---- |

##### Request Body schema: multipart/form-data

| file | string \<binary\> The attachment file to be uploaded, the max size for the file name is 223 characters |
| :---- | :---- |

### Responses

**200** OK

**400** Bad Request – various validation errors

**403** Forbidden

**404** Not Found

**405** Method Not Allowed

**500** Internal Server Error

post/investigations/investigation/{investigation\_id}/attachment

### **Response samples**

* **200**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"attachment": {`  
  * `"url": "https://url_to_your_attachment.com/some/path/to/attachment.png",`  
  * `"fileName": "your_file_name.png",`  
  * `"date": 2178781,`  
  * `"metadata": {},`  
  * `"author": {}`  
* `}`

`}`

## Upload attachment and add it to status

**Permission Level Needed:** Experts/Managers

Upload attachment and add it to Status

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| investigation\_id required | string Example: 123 |
| :---- | :---- |
| status\_action required | string Example: INVESTIGATION\_REQUESTED Status type ( INVESTIGATION\_REQUESTED, INVESTIGATION\_STARTED, SEARCHING\_INFORMATION, OBTAINING\_ITEMS, ANALYZING\_ITEMS, VERIFYING\_ITEMS, TESTING\_ITEMS, WAITING\_THREAT\_ACTOR\_RESPONSE, WAITING\_CUSTOMER\_RESPONSE, GENERATING\_REPORT, INVESTIGATION\_COMPLETED, INVESTIGATION\_INTERRUPTED, INVESTIGATION\_REOPENED ) |
| status\_creation\_date required | integer Example: 12676472 the creation date time in millis from the status action |

##### Request Body schema: multipart/form-data

| file | string \<binary\> The attachment file to be uploaded, the max size for the file name is 223 characters |
| :---- | :---- |

### Responses

**200** OK

**400** Bad Request

**403** Forbidden

**404** Not Found

**405** Method Not Allowed

**500** Internal Server Error

post/investigations/investigation/{investigation\_id}/status/{status\_action}/{status\_creation\_date}/attachment

### **Response samples**

* **200**

Content type  
application/json

Copy

Expand allCollapse all

`true`

# Open Data

The product enables Axur's clients to access detailed datasets of their platform interactions through API, adhering to the OpenData data exchange standard. This data can be plugged into their Business Intelligence software to generate dashboards and insights, helping them to better understand the health of their operations and sell the value of Axur's services to their executives. Additionally, the data can be used to fine-tune Axur's platform detection, such as identifying which collector is bringing more false positives, allowing for adjustments and reducing time spent on triage.

Initially (in V1), only managers will have access to the service and will be responsible for creating API keys. These API keys can then be shared with other users at the managers' discretion.

# Supported operations on Open Data

## Get available datasets

Get available datasets

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

### Responses

**200** OK

**403** Forbidden

get/customer-open-data/datasets

### **Request samples**

* **Python 3.7**  
* **Bash cURL**

Copy  
import requests

url \= 'https://api.axur.com/gateway/1.0/api/customer-open-data/datasets'  
token \= '\<token\>'

headers \= {  
        'Content-Type': 'application/json',  
        'Authorization': 'Bearer {}'.format(token)  
}

result \= requests.get(url, headers\=headers)  
print(result.json())

### **Response samples**

* **200**

Content type  
application/json

Copy

Expand allCollapse all

`[`

* `{`  
  * `"dataset": "tickets",`  
  * `"createdAt": 169893289310`  
* `}`

`]`

## Get available tables for a dataset

Get available tables for a dataset

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| dataset required | string |
| :---- | :---- |

### Responses

**200** OK

**403** Forbidden

get/customer-open-data/datasets/{dataset}

### **Request samples**

* **Python 3.7**  
* **Bash cURL**

Copy  
import requests

url \= 'https://api.axur.com/gateway/1.0/api/customer-open-data/datasets/tickets'  
token \= '\<token\>'

headers \= {  
        'Content-Type': 'application/json',  
        'Authorization': 'Bearer {}'.format(token)  
}

result \= requests.get(url, headers\=headers)  
print(result.json())

### **Response samples**

* **200**

Content type  
application/json

Copy

Expand allCollapse all

`[`

* `{`  
  * `"filename": "tickets_22-11-2023.csv",`  
  * `"createdAt": 169893289310`  
* `}`

`]`

## Get available dictionary for a dataset

Get available dictionary for a dataset

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| dataset required | string |
| :---- | :---- |

### Responses

**200** OK

**403** Forbidden

get/customer-open-data/datasets/{dataset}/dictionary

### **Request samples**

* **Python 3.7**  
* **Bash cURL**

Copy  
import requests

url \= 'https://api.axur.com/gateway/1.0/api/customer-open-data/datasets/tickets/dictionary'  
token \= '\<token\>'

headers \= {  
        'Content-Type': 'application/octet-stream',  
        'Authorization': 'Bearer {}'.format(token)  
}

result \= requests.get(url, headers\=headers)  
print(result.json())

## Get a dataset table file

Get a dataset table file

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| dataset required | string |
| :---- | :---- |
| filename required | string |

### Responses

**200** OK

**403** Forbidden

get/customer-open-data/datasets/{dataset}/{filename}

### **Request samples**

* **Python 3.7**  
* **Bash cURL**

Copy  
import requests

url \= 'https://api.axur.com/gateway/1.0/api/customer-open-data/datasets/tickets/tickets-2023.csv'  
token \= '\<token\>'

headers \= {  
        'Content-Type': 'application/octet-stream',  
        'Authorization': 'Bearer {}'.format(token)  
}

result \= requests.get(url, headers\=headers)  
print(result.json())

## Get last file added to a dataset

Get last file added to a dataset

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### path Parameters

| dataset required | string |
| :---- | :---- |

### Responses

**200** OK

**403** Forbidden

get/customer-open-data/datasets/{dataset}/current

### **Request samples**

* **Python 3.7**  
* **Bash cURL**

Copy  
import requests

url \= 'https://api.axur.com/gateway/1.0/api/customer-open-data/datasets/tickets/current'  
token \= '\<token\>'

headers \= {  
        'Content-Type': 'application/octet-stream',  
        'Authorization': 'Bearer {}'.format(token)  
}

result \= requests.get(url, headers\=headers)  
print(result.json())

### **Response samples**

* **200**

Content type  
application/json

Copy

Expand allCollapse all

`[`

* `{`  
  * `"filename": "tickets_22-11-2023.csv",`  
  * `"createdAt": 169893289310`  
* `}`

`]`

# Http Feed API

The HTTP Feed API endpoint allows clients to upload structured feed content containing a source identifier, a list of URLs, and an optional suggested locale. The request body must be a JSON object that includes at least one URL. A successful operation returns a 201 Created status with a basic success message.

The source field is a required alphanumeric string that uniquely identifies the origin of the submission, allowing for proper attribution and tracking. The optional suggestedLocale field is a standard locale code (e.g., "en-US", "pt-BR") that can be used during the inspection or processing of the submitted URLs to guide language or regional context handling. This endpoint is rate-limited and includes audit logging with the source IP address.

# Feed

## Upload Content Feed

Submits a structured feed containing a source identifier, a list of URLs, and an optional suggested locale.

**Permission Level Needed:** All permission levels

The source is a required alphanumeric string that identifies the origin of the feed submission.  
The suggestedLocale is an optional locale code (e.g., "en-US", "pt-BR") that can be used to guide the processing of URLs.

##### Request Body schema: application/json

| source required | string Alphanumeric identifier for the feed origin. This is an arbitrary value that can be freely chosen according to your brand, company name, or any other meaningful identifier you prefer to use for tracking or organizing the submission source. |
| :---- | :---- |
| urls required | Array of strings List of URLs to be processed. |
| suggestedLocale | string Optional locale code used for URL inspection. |

### Responses

**201** Feed accepted and stored successfully.

**400** Bad Request – validation failed (e.g., no URLs provided).

**500** Internal Server Error – unexpected failure during processing.

post/http-feed/1.0/content-feed

### **Request samples**

* **Payload**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"source": "my-source-key",`  
* `"urls": [`  
  * `"https://example.com/page1",`  
  * `"https://example.com/page2"`  
* `],`  
* `"suggestedLocale": "en-US"`

`}`

### **Response samples**

* **201**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"message": "success"`

`}`

# IoC Feed

**DEPRECATION NOTICE: IoC Feed operations are deprecated and will be discontinued. Please refer to the [TAXII Server IOC collection](https://docs.axur.com/en/axur/api/#tag/IoC-Collection).**

The IoC Feed is an on-demand feed of indicators from Axur sources, including our MISP. Its main purpose is to be a database of IoCs that can be retrieved at any time, with support for filtering by type, date, and more.

To access the API, you must purchase a plan from our sales team. We also offer free trials and proofs of concept.

# Supported operations on IoC Feed

⚠️ Deprecated

## Gets a report of available IoCs Deprecated

IoC's report

**DEPRECATION NOTICE: this endpoint has been replaced by the [TAXII Server IOCs collection](https://docs.axur.com/en/axur/api/#operation/getCollectionObjects) and will be removed in a future version.**

Authorizations:

[bearerAuth](https://docs.axur.com/en/axur/api/#section/Authentication/bearerAuth)

##### query Parameters

| format | string Example: format=STIX Response format ( CSV, STIX) |
| :---- | :---- |
| created | string Example: created=1697425200000,1697511599999 Timestamp (ms) frame in which to search for IoCs (start,end) |
| type | string Example: type=sha256,domain,url IoC types to search for (domain, filename, md5, sha1, sha256, url) |
| tags | string Example: tags=containsAny(,string) Searches for IoCs with tags that match any string provided |

### Responses

**200** OK

**403** Forbidden

get/ioc-core/iocs/report/stix

### **Request samples**

* **Python 3.7**

Copy  
import requests

PATH \= 'https://api.axur.com/gateway/1.0/api/ioc-core/iocs/report/stix'  
token \= '\<token\>'  
created \= 'created=1697425200000,1697511599999'

headers \= {  
  'Content-Type': 'application/json',  
  'Authorization': 'Bearer {}'.format(token)  
}

url \= PATH \+ '?' \+ created  
result \= requests.get(url, headers\=headers)  
print(result.json())

### **Response samples**

* **200**

Content type  
application/json

Copy

Expand allCollapse all

`{`

* `"type": "bundle",`  
* `"spec_version": 2,`  
* `"id": "bundle--bf2fbac2-7fd7-4a59-90c6-5a6bdab95016",`  
* `"objects": [`  
  * `{}`  
* `]`

`}`

# Changelog

### **1.0.0 | 2022-12-20**

* Initial version

### **1.0.1 | 2023-01-20**

* Added attachment actions in ticket operations  
* Added translation of platform terms into API language

### **1.0.2 | 2023-02-02**

* Add some request sample in pyhton and cURL

### **1.0.3 | 2023-02-28**

* Better description of supported fields  
* Added documentation for Cardcast API and Leakstream™

### **1.0.4 | 2023-03-09**

* Added documentation for Cardstream™

### **1.0.5 | 2023-03-24**

* Removed "executive-mobile-phone" from Ticket Types in Ticket Operation

### **1.0.6 | 2023-06-15**

* Added “Customs” from Permission levels

### **1.0.7 | 2023-06-29**

* Added GET Retrive subscriptions example  
* Added POST Create subscription example  
* Added DEL Remove subscription example  
* Added POST Ping subscription example

### **1.0.8 | 2023-07-05**

* Added documentation for API Keys

### **1.0.9 | 2023-07-19**

* Added new ticket type "infostealer-credential"

### **1.0.10 | 2023-07-31**

* Added new route to get infostealer credential leaks

### **1.0.11 | 2023-08-09**

* Added new ticket type "ransomware-attack"

### **1.0.12 | 2023-08-16**

* Added documentation for IoC Search

### **1.0.13 | 2023-09-13**

* Added documentation for Credit Card Exposure

### **1.0.14 | 2023-10-11**

* Added documentation for Customer Credential Exposure

### **1.0.15 | 2023-10-13**

* Added documentation for Investigations

### **1.0.16 | 2023-10-17**

* Added documentation for IoC Core

### **1.0.17 | 2023-10-30**

* Removed ticket type "infostealer-credential" from ticket creation

### **1.0.18 | 2023-11-17**

* Removed unsupported ticket types from ticket creation

### **1.0.19 | 2023-11-22**

* Added documentation for Open Data

### **1.0.20 | 2024-02-19**

* Removed deprecated Cardcast and Leakstream™ endpoints

### **1.0.21 | 2024-03-04**

* Added sourceType field to Cardcast endpoints

### **1.0.22 | 2024-03-07**

* Fixed users endpoint

### **1.0.23 | 2024-03-21**

* Updated ticket extraction examples to use API KEY.  
* Removed Create Session by username and password  
* Removed Api Keys session  
* Updated authentication session with API KEY information  
* Updated translation of platform terms into API language

### **1.0.24 | 2024-04-22**

* Added documentation for Customer/Asset

### **1.0.25 | 2024-04-26**

* Updated documentation for Cardstream™ to include the incomplete flag

### **1.0.26 | 2024-06-17**

* Added permissioning for each endpoint

### **1.0.27 | 2024-08-19**

* Added documentation for Ticket API

### **1.0.28 | 2024-08-22**

* Removed LeakStream documentation  
* Added Ticket API creation

### **1.0.29 | 2024-09-16**

* Added Ticket API field text operator

### **1.0.30 | 2024-12-17**

* Removal of the requirement to enter the customer key when creating tickets using [Ticket API](https://docs.axur.com/en/axur/api/#operation/createTicket)

### **1.0.31 | 2024-12-20**

* Possibility of selecting the composition of the ticket response when using the [Ticket API](https://docs.axur.com/en/axur/api/#operation/retrieveTicketApi)

### **1.0.32 | 2025-01-29**

* Added Exposure API documentation  
* Added Credential search operations  
* Added Credential update operations

### **1.0.33 | 2025-02-04**

* Addeed MSSP options to Exposure API documentation

### **1.0.34 | 2025-02-14**

* Addeed MSSP options to Fields supported by filters

### **1.0.35 | 2025-03-07**

* Added Ticket Stats to Ticket Api Operations  
* Added Credential Count Operations

### **1.0.36 | 2025-03-11**

* Added CardHash Check Endpoint

### **1.0.37 | 2025-03-14**

* Improved the descriptions for query parameters in the exposure API's credentials and total endpoints, clarifying their usage and defaults.  
* Added Python and cURL code samples for better developer guidance.  
* Replaced a detailed field list in count API docs with a reference to the search API documentation for consistency and brevity.

### **1.0.38 | 2025-04-07**

* Add 'user.length' field to exposure API search documentation

### **1.0.39 | 2025-04-16**

* Add Credit Card Exposure \- For Issuers webhook signature

### **1.0.40 | 2025-05-08**

* Deprecate infostealer credential tickets details endpoint.

### **1.0.41 | 2025-05-09**

* Add Phishing URL Feed information.

### **1.0.42 | 2025-05-15**

* Add URL Feed API information.

### **1.0.43 | 2025-05-23**

* Add Threat & Exposure Intelligence TAXII Server information.

### **1.0.44 | 2025-06-13**

* Add documentation for credential tag management operations

### **1.0.45 | 2025-06-16**

* Add endpoints for removing field data in credential detections

### **1.0.46 | 2025-06-30**

* Adjusted multiple endpoint response schemas to ensure full compatibility with OpenAPI 3.0 specifications.

### **1.0.47 | 2025-07-10**

* Added webhooks for exposure events

### **1.0.48 | 2025-07-11**

* Adjusted descriptions of Retrieve Tickets and Retrieve Bulk by Key

### **1.0.49 | 2025-07-30**

* Added Python code sample for TAXII2 client with Bearer authentication

### **1.0.50 | 2025-08-22**

* Adjusted Python code sample for TAXII2 client with Bearer authentication

### **1.0.51 | 2025-09-08**

* Deprecated IOC Feed operations  
* Highlighted the Threat & Exposure Intelligence TAXII Server IoC collection

### **1.0.52 | 2025-09-23**

* Add Web Complaints documentation

### **1.0.53 | 2025-09-25**

* Update Web Complaints documentation

### **1.0.54 | 2025-09-30**

* Remove Phishing URL Feed section

### **1.0.55 | 2025-10-04**

* Update Web Complaints documentation

### **1.0.56 | 2025-10-23**

* Add 'leak.format' field to exposure API search documentation

### **1.0.57 | 2025-11-12**

* Add 'warnings' field to ticket create endpoint

