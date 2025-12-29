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

##### Response Schema: application/json

| status | integer |
| :---- | :---- |
| code | string |
| message | string |

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
