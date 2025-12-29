Credential Search Operations  
Examples of search usage  
Retrieve all credentials detected with specific criteria, such as their status and user information, and using the Arizona (USA) timezone:

GET https://api.axur.com/gateway/1.0/api/exposure-api/credentials?status=NEW,DISCARDED\&user=contains:admin\&sortBy=created\&page=1\&pageSize=100\&order=asc\&timezone=-07:00  
Retrieve credentials filtered by detection date range, using the Brazil timezone, and retrieving only status, user, password, created, assets, password.hasLetter, and password.length fields:

GET https://api.axur.com/gateway/1.0/api/exposure-api/credentials?created=ge:2024-07-01\&updated=le:2024-07-31\&sortBy=created\&page=1\&pageSize=1\&order=desc\&timezone=-03:00\&fields=status,user,password,created,assets,password.hasLetter,password.length  
The response for the request above will be HTTP STATUS 200 OK with the following body:

{  
  "detections": \[  
    {  
      "password": "DtsKcJvlpM",  
      "assets": \[  
        "ANY\_ASSET"  
      \],  
      "created": "2024-07-27T17:18:36",  
      "password.hasLetter": true,  
      "user": "test@example.com",  
      "password.length": 10,  
      "status": "DISCARDED"  
    }  
  \],  
  "pageable": {  
    "pageNumber": 1,  
    "pageSize": 1,  
    "total": 200  
  }  
}  
Credentials exposure operators  
For date or numeric fields, you can use the following operators:

gt: \- Greater than  
lt: \- Less than  
ge: \- Greater than or equal  
le: \- Less than or equal  
Operators should be used as prefixes for values in query parameters, for example:

created=ge:2024-07-01  
updated=le:2024-07-31  
For text fields, you can use the following operators:

contains: \- Contains Text  
Operators should be used as prefixes for values in query parameters, for example:

user=contains:admin  
To choose which fields to return, use the fields query parameter, separating the desired fields by commas, for example:

fields=status,user,password,created,assets,password.hasLetter,password.length  
Fields supported by exposure API  
Key	Type	Notes  
access.appId	STRING	Access app ID  
access.domain	STRING	Access domain  
access.host	STRING	Access host  
access.tld	STRING	Access TLD  
access.url	STRING	Credential access URL  
assets	LIST	A list containing the asset keys, needs to be separated by comma, e.g: "ASSET1,ASSET,ASSET 3"  
created	DATE	Creation date, supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33"  
credential.types	LIST	A list containing the Credential types, credential type is an enum, it can be "user" or "employee". The list needs to be separated by comma, e.g: "user,employee"  
customer	STRING	Customer Key identifier  
detectionDate	DATE	Detection date, supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33"  
document.id	STRING	Document ID  
document.links.originalFile	STRING	Source file  
document.links.originalFilePackage	STRING	Source file package  
document.links.parsedFrom	STRING	File chunk  
document.links.raw	STRING	Document of origin  
document.timestamp	DATE	Document creation date, supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33"  
downloadLink.detectionDate	DATE	Download link detection date  
downloadLink.source	STRING	Download link source  
downloadLink.value	STRING	Download link  
file.mimeType	STRING	File MIME type  
file.name	STRING	File name  
file.originalPath	STRING	File original path  
file.path	STRING	File path  
file.relativePath	STRING	File relative path  
file.sizeInBytes	INTEGER	File size (Bytes)  
file.timestamp	DATE	File processing date, supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33"  
id	STRING	Detection ID  
intelx.fileName	STRING	IntelX file Name  
intelx.fileType	STRING	IntelX file Type  
intelx.publishDate	DATE	IntelX publish date, supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33"  
leak.descriptions.en	STRING	Leak description  
leak.descriptions.es	STRING	Leak description (Spanish)  
leak.descriptions.pt	STRING	Leak description (Portuguese)  
leak.displayName	STRING	Leak display name  
leak.exposureDate	DATE	Leak exposure date, supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33"  
leak.forum	STRING	Leak forum  
leak.name	STRING	Leak name  
leak.source	STRING	Leak source  
leak.url	STRING	Leak URL  
leak.format	STRING	Leak format, it can be "COMBOLIST", "TABLE" or "STEALER LOG"  
message.author.identifier	INTEGER	Message Author Identifier  
message.author.type	STRING	Message author type  
message.autoDeleteIn	INTEGER	Message auto delete in  
message.chat.identifier	INTEGER	Message Chat Identifier  
message.chat.name	STRING	Message chat name  
message.chat.type	STRING	Message chat type  
message.identifier	INTEGER	Message identifier  
message.repliedTo.chatIdentifier	INTEGER	Replied Message Chat Identifier  
message.repliedTo.messageIdentifier	INTEGER	Replied Message Identifier  
message.selfDestructIn	INTEGER	Self-destruct in  
message.threadId	INTEGER	Conversation identifier  
message.timestamp	DATE	Message publication date, supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33"  
message.type	STRING	Message type  
password	STRING	Password  
password.hasLetter	BOOLEAN	Contains letter  
password.hasLowerCase	BOOLEAN	Contains lowercase letter  
password.hasNumber	BOOLEAN	Contains number  
password.hasSpecialChar	BOOLEAN	Contains special character  
password.hasUpperCase	BOOLEAN	Contains uppercase letter  
password.length	INTEGER	Password length  
password.type	ENUM	Password type, it can be "PLAIN", "MYSQL323", "MD5", "SHA512", "SHA256", "SHA1", "BCRYPT", "SHA384" or "PBKDF2"  
paste.date	DATE	Paste publication date, supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33"  
paste.expire	DATE	Paste expiration date, supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33"  
paste.originLocation	STRING	Paste URL  
paste.source	STRING	Paste source  
paste.title	STRING	Paste title  
paste.user	STRING	Paste author  
source.name	STRING	Source name  
source.timestamp	DATE	Exposure date at the source, supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33"  
source.url	STRING	Source URL  
status	ENUM	Detection Status, it can be "NEW", "IN\_TREATMENT", "SOLVED" or "DISCARDED"  
tags	LIST	Custom labels for organizing detections.  
updated	DATE	Detection update date, supported formats "yyyy-MM-dd" and "yyyy-MM-ddTHH:mm:ss". Sample "2024-01-31T04:55:33"  
user	STRING	Username  
user.emailDomain	STRING	Email domain  
user.emailHost	STRING	Email host  
user.emailTld	STRING	Email TLD  
user.length	INTEGER	Username length  
user.type	ENUM	User type, it can be "EMAIL", "CPF", "USERNAME", "PHONE" or "CNPJ"  
Notes:  
Date fields support formats yyyy-MM-dd and yyyy-MM-ddTHH:mm:ss;  
On date fields using yyyy-MM-ddT00:00:00 is equivalent to yyyy-MM-dd.  
Boolean fields are represented as true, false, 1 (for true value) or 0 (for false value).  
Fields are case-insensitive unless explicitly stated otherwise.  
pageSizes are limited to 1000\.  
The timezone parameter is optional and defaults to UTC.  
The fields parameter is optional and defaults to all fields.  
The order parameter is optional and defaults to desc.  
The sortBy parameter is optional and defaults to created.  
The page parameter is optional and defaults to 1\.  
The pageSize parameter is optional and defaults to 50\.  
The result of pageSize \* page should not exceed 1000000\.  
Only date and integer fields are allowed in the sortBy parameter.  
For user with access to multiple tenants, the customer parameter is required when searching for a child tenant's credentials.  
CSV Credential Extraction Example  
Extract credentials into a CSV file, filtering by the desired criteria.

Python

This example uses Python 3.12 and standard authentication methods.

\#\!/bin/bash  
import csv  
import json  
import logging  
import time  
from logging import Logger

import requests

QUERY: str \= "status=NEW,DISCARDED\&user=contains:admin\&sortBy=created\&page=1\&pageSize=50\&order=asc\&timezone=-07:00"  
BASE\_URL: str \= f"https://api.axur.com/gateway/1.0/api/exposure-api/credentials?{QUERY}"  
API\_KEY: str \= "\<API\_KEY\>"  
MAX\_RETRIES: int \= 3  
INITIAL\_WAIT: float \= 1.0  
WAIT\_INCREMENT: float \= 0.5

logging.basicConfig(format='%(asctime)s %(levelname)s %(name)s.%(funcName)s \- %(message)s', level=logging.INFO)  
LOGGER: Logger \= logging.getLogger(\_\_name\_\_)

def make\_request(retries: int \= 0, wait\_time: float \= INITIAL\_WAIT) \-\> list\[dict\[str, any\]\] | None:  
    headers \= get\_headers(API\_KEY)

    response \= requests.get(BASE\_URL, headers=headers)  
    return process\_response(response, retries, wait\_time)

def get\_headers(api\_key: str) \-\> dict\[str, str\]:  
    out\_header \= {"Content-Type": "application/json"}

    if api\_key:  
        out\_header\["Authorization"\] \= str.format("Bearer {}", api\_key)

    return out\_header

def process\_response(response: requests.Response, retries: int, wait\_time: float) \-\> list\[dict\[str, any\]\] | None:  
    if 200 \<= response.status\_code \< 300:  
        if response.text:  
            return json.loads(response.text)\["detections"\]  
    elif response.status\_code \== 429:  
        if retries \>= MAX\_RETRIES:  
            response.raise\_for\_status()  
        retries \+= 1  
        time.sleep(wait\_time)  
        wait\_time \+= WAIT\_INCREMENT  
        return make\_request(retries, wait\_time)  
    else:  
        response.raise\_for\_status()

def write\_to\_csv(data: list\[dict\[str, any\]\], filename: str) \-\> None:  
    if not data:  
        LOGGER.warning("No data to write.")  
        return

    with open(filename, mode='w', newline='', encoding='utf-8') as file:  
        fieldnames: set\[str\] \= set(\[key for item in data for key in item.keys()\])  
        writer \= csv.DictWriter(file, fieldnames=fieldnames)  
        writer.writeheader()  
        for item in data:  
            writer.writerow(item)

def main() \-\> None:  
    response \= make\_request()  
    write\_to\_csv(response, "data.csv")

if \_\_name\_\_ \== "\_\_main\_\_":  
    main()  
Bash cURL

Use a bash script to extract credential details:

\#\!/bin/bash

API\_KEY='\<API\_KEY\>'  
BASE\_URL='https://api.axur.com/gateway/1.0/api/exposure-api/credentials'

ONE\_HOUR\_AGO=$(TZ='America/Sao\_Paulo' date \--date='-24 hour' \+"%Y-%m-%dT%H:%M:%S")  
CURRENT\_TIME=$(TZ='America/Sao\_Paulo' date \+"%Y-%m-%dT%H:%M:%S")  
SORT\_FIELD="created"  
PAGES=1  
DETECTIONS\_PER\_PAGE=50  
TIMEZONE='-03:00'  
DESIRED\_FIELDS='id,status,user,password,created,assets,password.hasLetter,password.length'

mkdir \-p ./detections

sleep 3

GETQ=$(curl \--noproxy "\*" \\  
\-sLX GET "${BASE\_URL}?created=ge:${ONE\_HOUR\_AGO}\&created=le:${CURRENT\_TIME}\&sortBy=${SORT\_FIELD}\&page=${PAGES}\&pageSize=${DETECTIONS\_PER\_PAGE}\&timezone=${TIMEZONE}\&fields=${DESIRED\_FIELDS}" \\  
\-H "Authorization: Bearer ${API\_KEY}")

\# Check if the response is a valid JSON  
if echo "$GETQ" | jq . \> /dev/null 2\>&1; then  
    echo "$GETQ" | tr \-d '\\000-\\037' | jq \-c '.detections\[\]' | while read \-r detection; do  
        id=$(echo "$detection" | jq \-r '.id')  
        echo "$detection" | jq '.' \> "detections/${id}.json"  
    done  
else  
    echo "Error: The server response is not a valid JSON."  
fi  
Retrieve Credential Detections  
Permission Level Needed: All permission levels

Returns credentials detections matching the specified filter.

Authorizations:  
bearerAuth  
query Parameters  
created	  
string  
Example: created=ge:2024-07-01  
A filter to retrieve credential detections created on or after the specified date in ISO 8601 format (e.g., 'yyyy-MM-dd').

sortBy	  
string  
Default: "created"  
Example: sortBy=created  
A field to specify the attribute by which the results will be sorted.

page	  
integer  
Default: 1  
Example: page=1  
The page number to retrieve in the paginated response. Defaults to 1 if not specified.

pageSize	  
integer  
Default: 50  
Example: pageSize=2  
The number of results displayed per page in the paginated response. Defaults to 50 if not specified.

order	  
string  
Default: "desc"  
Enum: "desc" "asc"  
Example: order=desc  
The order in which the results are sorted. Accepts 'asc' for ascending order and 'desc' for descending order. Defaults to 'desc'.

timezone	  
string  
Default: "00:00"  
Example: timezone=-03:00  
Use one of the records identified in the UTF Offset column on https://en.wikipedia.org/wiki/Time\_zone\#List\_of\_UTC\_offsets. The UTC offset will be used to convert the date input via query parameters and the date output via json.

Responses  
200 OK  
400 Incorrect Query Parameters  
403 FORBIDDEN  
429 Rate Limit exceeded

get  
/exposure-api/credentials  
Request samples  
Python 3.12Bash cURL

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

result \= requests.get(PATH, headers=header)

print(result.json())  
Response samples  
200400403429  
Content type  
application/json

Copy  
Expand allCollapse all  
{  
"detections": \[  
{},  
{}  
\],  
"pageable": {  
"pageNumber": 1,  
"pageSize": 2,  
"total": 1200  
}  
}  
Credential Update Operations  
This section describes the available operations for updating credential detections, including status changes and tag management.

Status Update Operations  
Update Single Detection Status  
Update the status of a specific detection by providing its ID:

PATCH https://api.axur.com/gateway/1.0/api/exposure-api/credentials/:id  
Request Body:

{  
  "field": "status",  
  "value": "DISCARDED"  
}  
Bulk Update Detection Status  
Update the status of multiple detections simultaneously:

PATCH https://api.axur.com/gateway/1.0/api/exposure-api/credentials  
Request Body:

{  
  "field": "status",  
  "value": "DISCARDED",  
  "ids": \[  
    "202411271319565002EF35C72F8C5C298",  
    "202411271319565002EF35C72F8C5C299"  
  \]  
}  
Response: All status update operations return HTTP STATUS 204 NO CONTENT.

Tag Management Operations  
Single Detection Tag Management  
Add Tags to a Single Detection  
Add new tags to a specific detection:

POST https://api.axur.com/gateway/1.0/api/exposure-api/credentials/:id/tags  
Request Body:

{  
  "values": \[  
    "my tag",  
    "tag2"  
  \]  
}  
Remove Tags from a Single Detection  
Remove specific tags from a detection:

POST https://api.axur.com/gateway/1.0/api/exposure-api/credentials/delete/:id/tags  
Request Body:

{  
  "values": \[  
    "tag3",  
    "tag4"  
  \]  
}  
Bulk Tag Management  
Add Tags to Multiple Detections  
Add tags to several detections at once:

POST https://api.axur.com/gateway/1.0/api/exposure-api/credentials/tags  
Request Body:

{  
  "values": \[  
    "my tag",  
    "tag2"  
  \],  
  "ids": \[  
    "202411271319565002EF35C72F8C5C298",  
    "202411271319565002EF35C72F8C5C299"  
  \]  
}  
Remove Tags from Multiple Detections  
Remove tags from several detections simultaneously:

POST https://api.axur.com/gateway/1.0/api/exposure-api/credentials/delete/tags  
Request Body:

{  
  "values": \[  
    "tag3",  
    "tag4"  
  \],  
  "ids": \[  
    "202411271319565002EF35C72F8C5C298",  
    "202411271319565002EF35C72F8C5C299"  
  \]  
}  
Response: All tag operations return HTTP STATUS 204 NO CONTENT.

Multi-Tenant Support (MSSP)  
For users with access to multiple tenants, include the customer parameter when managing a child tenant's credentials:

Status Update Example (MSSP)  
{  
  "field": "status",  
  "value": "DISCARDED",  
  "ids": \[  
    "202411271319565002EF35C72F8C5C298",  
    "202411271319565002EF35C72F8C5C299"  
  \],  
  "customer": "CHILD\_KEY"  
}  
Tag Management Example (MSSP)  
{  
  "values": \[  
    "my tag",  
    "tag2"  
  \],  
  "ids": \[  
    "202411271319565002EF35C72F8C5C298",  
    "202411271319565002EF35C72F8C5C299"  
  \],  
  "customer": "CHILD\_KEY"  
}  
Supported Fields  
Field	Type	Operations	Notes  
status	ENUM	PATCH only	Detection status: "NEW", "IN\_TREATMENT", "SOLVED", or "DISCARDED"  
tags	LIST	POST only	Custom labels for organizing detections. Tags are case-sensitive.  
Important Notes  
General Guidelines  
Maximum bulk operations: 1000 IDs per request  
Field case sensitivity: Fields are case-insensitive unless explicitly stated otherwise  
Detection ID retrieval: Use the Credential Search Operations to obtain detection IDs  
Method-Specific Rules  
PATCH operations: Only the status field is supported  
POST/DELETE operations: Only the tags field is currently supported  
Field parameter: The :field URL parameter currently supports only tags, but may be extended in the future  
Multi-Tenant Requirements  
MSSP users: Must include the customer parameter when managing child tenant credentials  
Applies to: All update operations (PATCH, POST)  
Customer value: Use the child tenant's customer key (e.g., "CHILD\_KEY")  
Response Codes  
Success: 204 NO CONTENT for all successful operations  
Error codes: 400 (Bad Request), 429 (Rate Limit Exceeded)  
Bulk Update Credential Detection  
Permission Level Needed: All permission levels

Update the detection field with the specified ids.

To obtain id, the 'Credential Search Operations' section must be consulted (see Get Credential Search Operations).  
Authorizations:  
bearerAuth  
header Parameters  
Content-Type	  
string  
Example: application/json  
Request Body schema: application/json  
field  
required  
string  
value  
required  
string  
ids  
required  
Array of strings  
customer	  
string  
Customer key (required for MSSP users when managing child tenant's credentials)

Responses  
204 NO CONTENT  
400 Incorrect Query Parameters or Body  
429 Rate Limit exceeded

patch  
/exposure-api/credentials  
Request samples  
PayloadPython 3.12Bash cURL  
Content type  
application/json

Copy  
Expand allCollapse all  
{  
"field": "status",  
"value": "DISCARDED",  
"ids": \[  
"202411271319565002EF35C72F8C5C298",  
"202411271319565002EF35C72F8C5C299"  
\],  
"customer": "CHILD\_KEY"  
}  
Response samples  
204400429  
Content type  
text/plain

Copy  
Update Credential Detection  
Permission Level Needed: All permission levels

Update the detection field with the specified id.

To obtain id, the 'Credential Search Operations' section must be consulted (see Get Credential Search Operations).  
Authorizations:  
bearerAuth  
path Parameters  
id  
required  
string  
Example: 202411271319565002EF35C72F8C5C298  
detection id

header Parameters  
Content-Type	  
string  
Example: application/json  
Request Body schema: application/json  
field  
required  
string  
value  
required  
string  
customer	  
string  
Customer key (required for MSSP users when managing child tenant's credentials)

Responses  
204 NO CONTENT  
400 Incorrect Query Parameters or Body  
429 Rate Limit exceeded

patch  
/exposure-api/credentials/{id}  
Request samples  
PayloadPython 3.12Bash cURL  
Content type  
application/json

Copy  
Expand allCollapse all  
{  
"field": "status",  
"value": "DISCARDED",  
"customer": "CHILD\_KEY"  
}  
Response samples  
204400429  
Content type  
text/plain

Copy  
Add Field Data to Credential Detection  
Permission Level Needed: All permission levels

Add field data to the detection with the specified id and field.

To obtain id, the 'Credential Search Operations' section must be consulted (see Get Credential Search Operations).  
Currently only 'tags' field is supported.  
Authorizations:  
bearerAuth  
path Parameters  
id  
required  
string  
Example: 202411271319565002EF35C72F8C5C298  
detection id

field  
required  
string  
Example: tags  
field name (currently only 'tags' is supported)

header Parameters  
Content-Type	  
string  
Example: application/json  
Request Body schema: application/json  
values  
required  
Array of strings  
customer	  
string  
Customer key (required for MSSP users when managing child tenant's credentials)

Responses  
204 NO CONTENT  
400 Incorrect Query Parameters or Body  
429 Rate Limit exceeded

post  
/exposure-api/credentials/{id}/{field}  
Request samples  
PayloadPython 3.12Bash cURL  
Content type  
application/json

Copy  
Expand allCollapse all  
{  
"values": \[  
"my tag",  
"tag2"  
\],  
"customer": "CHILD\_KEY"  
}  
Response samples  
204400429  
Content type  
text/plain

Copy  
Remove Field Data from Credential Detection  
Permission Level Needed: All permission levels

Remove field data from the detection with the specified id and field.

To obtain id, the 'Credential Search Operations' section must be consulted (see Get Credential Search Operations).  
Currently only 'tags' field is supported.  
Authorizations:  
bearerAuth  
path Parameters  
id  
required  
string  
Example: 202411271319565002EF35C72F8C5C298  
detection id

field  
required  
string  
Example: tags  
field name (currently only 'tags' is supported)

header Parameters  
Content-Type	  
string  
Example: application/json  
Request Body schema: application/json  
values  
required  
Array of strings  
customer	  
string  
Customer key (required for MSSP users when managing child tenant's credentials)

Responses  
204 NO CONTENT  
400 Incorrect Query Parameters or Body  
429 Rate Limit exceeded

post  
/exposure-api/credentials/delete/{id}/{field}  
Request samples  
PayloadPython 3.12Bash cURL  
Content type  
application/json

Copy  
Expand allCollapse all  
{  
"values": \[  
"my tag",  
"tag2"  
\],  
"customer": "CHILD\_KEY"  
}  
Response samples  
204400429  
Content type  
text/plain

Copy  
Bulk Add Field Data to Credential Detections  
Permission Level Needed: All permission levels

Add field data to multiple detections with the specified field.

To obtain ids, the 'Credential Search Operations' section must be consulted (see Get Credential Search Operations).  
Currently only 'tags' field is supported.  
Authorizations:  
bearerAuth  
path Parameters  
field  
required  
string  
Example: tags  
field name (currently only 'tags' is supported)

header Parameters  
Content-Type	  
string  
Example: application/json  
Request Body schema: application/json  
values  
required  
Array of strings  
ids  
required  
Array of strings  
customer	  
string  
Customer key (required for MSSP users when managing child tenant's credentials)

Responses  
204 NO CONTENT  
400 Incorrect Query Parameters or Body  
429 Rate Limit exceeded

post  
/exposure-api/credentials/{field}  
Request samples  
PayloadPython 3.12Bash cURL  
Content type  
application/json

Copy  
Expand allCollapse all  
{  
"values": \[  
"my tag",  
"tag2"  
\],  
"ids": \[  
"202411271319565002EF35C72F8C5C298",  
"202411271319565002EF35C72F8C5C299"  
\],  
"customer": "CHILD\_KEY"  
}  
Response samples  
204400429  
Content type  
text/plain

Copy  
Bulk Remove Field Data from Credential Detections  
Permission Level Needed: All permission levels

Remove field data from multiple detections with the specified field.

To obtain ids, the 'Credential Search Operations' section must be consulted (see Get Credential Search Operations).  
Currently only 'tags' field is supported.  
Authorizations:  
bearerAuth  
path Parameters  
field  
required  
string  
Example: tags  
field name (currently only 'tags' is supported)

header Parameters  
Content-Type	  
string  
Example: application/json  
Request Body schema: application/json  
values  
required  
Array of strings  
ids  
required  
Array of strings  
customer	  
string  
Customer key (required for MSSP users when managing child tenant's credentials)

Responses  
204 NO CONTENT  
400 Incorrect Query Parameters or Body  
429 Rate Limit exceeded

post  
/exposure-api/credentials/delete/{field}  
Request samples  
PayloadPython 3.12Bash cURL  
Content type  
application/json

Copy  
Expand allCollapse all  
{  
"values": \[  
"my tag",  
"tag2"  
\],  
"ids": \[  
"202411271319565002EF35C72F8C5C298",  
"202411271319565002EF35C72F8C5C299"  
\],  
"customer": "CHILD\_KEY"  
}  
Response samples  
204400429  
Content type  
text/plain

Copy  
Credential Count Operations  
Examples of count usage  
Count credentials detected with specific criteria, such as their status, date offset, and using the Brazil timezone:

GET https://api.axur.com/gateway/1.0/api/exposure-api/credentials/total?status=NEW\&timezone=-03:00\&created=ge:2024-01-01\&created=le:2024-01-25  
The response for the request above will be HTTP STATUS 200 OK with the following body:

{  
  "total": 1234  
}  
Fields supported by exposure API (count)  
All the fields and operations used for search can also be used in the count API.

Notes:  
Date fields support formats yyyy-MM-dd and yyyy-MM-ddTHH:mm:ss;  
On date fields using yyyy-MM-ddT00:00:00 is equivalent to yyyy-MM-dd.  
Boolean fields are represented as true, false, 1 (for true value) or 0 (for false value).  
Fields are case-insensitive unless explicitly stated otherwise.  
The timezone parameter is optional and defaults to UTC.  
For user with access to multiple tenants, the customer parameter is required when searching for a child tenant's credentials.  
Count Credentials  
Permission Level Needed: All permission levels

Returns total of credentials detections that match the specified filters.

Authorizations:  
bearerAuth  
query Parameters  
created	  
string  
Example: created=ge:2024-07-01T20:18:36  
The date and time the credentials were created, in ISO 8601 format (e.g., yyyy-MM-ddTHH:mm:ss). This parameter is optional and may be used to filter results based on the creation date.

timezone	  
string  
Default: "00:00"  
Example: timezone=-03:00  
Use one of the records identified in the UTF Offset column on https://en.wikipedia.org/wiki/Time\_zone\#List\_of\_UTC\_offsets. The UTC offset will be used to convert the date input via query parameters and the date output via json.

Responses  
200 OK  
400 Incorrect Query Parameters  
403 FORBIDDEN  
429 Rate Limit exceeded

get  
/exposure-api/credentials/total  
Request samples  
Python 3.12Bash cURL

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

result \= requests.get(PATH, headers=header)

print(result.json())  
Response samples  
200400403429  
Content type  
application/json

Copy  
Expand allCollapse all  
{  
"total": 42  
}