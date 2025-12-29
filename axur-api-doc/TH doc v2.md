Threat Hunting  
Threat Hunting is responsible for creating asynchronous searches on our data lake. You can query terabytes of data to find detections such as fraudulent websites, credit cards and credential leaks

For you to have access to the API you must hire a plan with our sales team.

Supported operations on Threat Hunting

Start Search  
This endpoint is used to initiate an asynchronous search. After initiating the search, you will receive a search-id and use it to pool the search results and status. Idle searches will be automatically cancelled.

Authorizations:  
bearerAuth  
Request Body schema: application/json  
query  
required  
string  
For information on how to create queries, visit: https://help.axur.com/en/articles/10542787-what-are-the-credential-search-parameters-of-threat-hunting

source  
required  
string  
signal-lake \- urls and domains, such as websites and the data it contains  
signal-lake-social-media \- profiles in social media platforms, such as Facebook  
signal-lake-ads \- ads in social media, such as Facebook  
credential \- login credentials from different platforms, such as email and password  
credit-card \- credit card data, such as bin and cvv  
chat-message \- messages in chat apps, such as Whatsapp, Telegram and Discord  
forum-message \- messages in deep web forums and ransomware feeds  
social-media-posts \- posts in social media, such as Twitter / X  
tokens \- chunks of text files where we identified tokens, such as urls, names, emails and SSNs  
customer	  
string  
Search credits will be consumed from the customer informed here. If no customer is informed, credits will be consumed from the user's main customer.

Responses  
200 OK  
402 PAYMENT\_REQUIRED

post  
/threat-hunting-api/external-search  
Request samples  
Payload  
Content type  
application/json

Copy  
Expand allCollapse all  
{  
"query": "emailDomain=\\"acme.com\\"",  
"source": "credential",  
"customer": "TEST"  
}  
Response samples  
200402  
Content type  
application/json

Copy  
Expand allCollapse all  
{  
"searchId": "5daeb8ae-dedb-4b87-b55d-9c26b00c612f"  
}  
Get Search Results  
This endpoint is used to get the status of a search and the paginated results.

Credits are consumed everytime you hit this endpoint with results to be seen. If the page you requested has no results, or if the partial results are insufficiente for pagination, you will not be charged.

Authorizations:  
bearerAuth  
path Parameters  
searchId  
required  
string  
Example: 355ecaed-f1c7-4206-8ae8-f2c85bab3343  
query Parameters  
page  
required  
integer  
Example: page=1  
alias	  
boolean  
Example: alias=true  
used to apply human-readable aliases on the document's properties

Responses  
200 OK  
402 PAYMENT\_REQUIRED

get  
/threat-hunting-api/external-search/{searchId}  
Response samples  
200402  
Content type  
application/json

Copy  
Expand allCollapse all  
{  
"id": "b9fa46a6-9e3a-4b2f-8fec-0a4e9e937a6b",  
"result": {  
"status": {  
"running": false,  
"totalParts": 14,  
"searchedParts": 14,  
"resultsLimit": 250000,  
"finishReason": "SUCCESSFUL",  
"totalResults": 102  
},  
"data": \[  
{  
"metadata.detectionDate": 1743682310476,  
"content.cardNumber": "1712891323625490",  
"metadata.bin": "171289",  
"metadata.bin8": "17128913",  
"metadata.source.name": "Deep/Dark Web \- Telegram",  
"metadata.source.timestamp": 1743680249000,  
"metadata.file.name": "ashawn\_unobs\_2.2.txt",  
"metadata.file.path": "\[PH\]49.150.6.168/Important Files/Desktop/AAA/",  
"metadata.message.author.type": "CHAT",  
"metadata.message.timestamp": 1743680249000,  
"metadata.message.type": "DOCUMENT",  
"metadata.message.chat.name": "🌩️ Moon Cloud | Free Logs",  
"metadata.message.chat.type": "SUPERGROUP",  
"id": "2025040312115075065500965443D2EDB11026979",  
"metadata.message.caption": "🔗 Link | 🌙 MoonProject | 👤 Admin\\n———————————————————\\n🪙BUY MOON CLOUD PRIVATE🪙\\n\\n🔎MoonSearcher | Best url-searcher bot"  
},  
{  
"metadata.detectionDate": 1743682264714,  
"content.cardNumber": "1712891323625490",  
"metadata.bin": "171289",  
"metadata.bin8": "17128913",  
"metadata.source.name": "Deep/Dark Web \- Telegram",  
"metadata.source.timestamp": 1743680249000,  
"metadata.file.name": "aFOR NEW INDEX.txt",  
"metadata.file.path": "\[PH\]49.150.6.168/Important Files/Desktop/AAA/A/",  
"metadata.message.author.type": "CHAT",  
"metadata.message.timestamp": 1743680249000,  
"metadata.message.type": "DOCUMENT",  
"metadata.message.chat.name": "🌩️ Moon Cloud | Free Logs",  
"metadata.message.chat.type": "SUPERGROUP",  
"id": "20250403121105075390602653EB5C6EA00023671",  
"metadata.message.caption": "🔗 Link | 🌙 MoonProject | 👤 Admin\\n———————————————————\\n🪙BUY MOON CLOUD PRIVATE🪙\\n\\n🔎MoonSearcher | Best url-searcher bot"  
}  
\],  
"pagination": {  
"page": 2,  
"size": 100,  
"offset": 100,  
"total": 102  
}  
}  
}

What are the credential search parameters of Threat Hunting?  
Updated over a week ago  
In this article, you'll find the credential search parameters, along with usage examples for each. If you have any questions, please contact our support team.

 

Attention: Credentials containing passwords with fewer than 4 digits are automatically disregarded and are not registered as detections on the Axur platform.

How to search for credential information?  
Parameter

Description

Example

leakFormat

Format in which the leak was found

leakFormat="COMBOLIST"  
leakFormat="STEALER LOG"  
leakFormat="TABLE"

detectionDate

Date when the credential was detected

detectionDate=YYYY-MM-DD

detectionDate\>=YYYY-MM-DDThh:mm

detectionDate\<=YYYY-MM-DDThh:mm:ss

detectionDate\>=YYYY-MM-DD AND detectionDate\<=YYYY-MM-DD

user

User associated with the credential

user=user@example.com

user=(user1@example.com OR user2@example.com)

emailDomain

Email domain

emailDomain=example.com

emailDomain=(example1.com OR example2.com)

emailHost

Email host

emailHost=support.example.com

emailHost=(support.example.com OR mkt.example.com OR ti.example.com)

emailTld

Top-level domain (TLD) of the email

emailTld=com

userType

Type of user

userType=EMAIL

 

userType=PHONE

 

userType=USERNAME 

password

Password associated with the credential

password=123456789

passwordHasLetter

Indicates if the password contains letters

passwordHasLetter=true

passwordHasLetter=false

passwordHasLowerCase

Indicates if the password contains lowercase letters

passwordHasLowerCase=true

passwordHasLowerCase=false

passwordHasUpperCase

Indicates if the password contains uppercase letters

passwordHasUpperCase=true

passwordHasUpperCase=false

passwordHasNumber

Indicates if the password contains numbers

passwordHasNumber=true

passwordHasNumber=false

passwordHasSpecialChar

Indicates if the password contains special characters

passwordHasSpecialChar=true

passwordHasSpecialChar=false

passwordLength

Length of the password in characters

passwordLength=8

passwordLength\>=12

passwordType

Type of stored password

passwordType=BCRYPT

 

passwordType=MD5

 

passwordType=MYSQL323

 

passwordType=PBKDF2

 

passwordType=PLAIN

 

passwordType=SHA1

 

passwordType=SHA256

 

passwordType=SHA384

 

passwordType=SHA512 

   
How to search for access information?  
 

Parameter

Description

Example

accessDomain

Domain associated with the credential

accessDomain=example.com

accessHost

Specific access host

accessHost=login.example.com

accessTld

Top-level domain (TLD) of access

accessTld=com

accessUrl

Access URL of the credential

accessUrl="https://login.example.com/app"

 

accessUrl=example.com

accessAppId

App ID for access (Google Play IDs only)

accessAppId="br.com.example.app"

   
How to search for leak source information?  
 

Parameter

Description

Example

sourceName

Name of the source where the credential was exposed

sourceName=Deep/Dark Web

 

sourceName=Deep/Dark Web \- Telegram

 

sourceName=Deep/Dark Web \- WhatsApp

 

sourceName=IntelX

 

sourceName=mega

 

sourceName=paste.ee

 

sourceName=pastebin

sourceUrl

URL of the source where the credential was found

sourceUrl=example.com

 

sourceUrl=breachforums.st

 

sourceUrl=leakbase.io

 

sourceUrl=\*

   
How to search for file information?  
 

Parameter

Description

Usage Examples

fileName

Name of the file

fileName="filename.txt"

filePath

Path of the file

filePath="50 000 000 Link Login Pass 10.rar"

What are Threat Hunting operators, and what are they?  
Updated over a week ago  
Search operators help refine and combine search terms efficiently, allowing for more precise and relevant queries.

 

What search operators are available in Threat Hunting?  
Operator

Definition

Example

" "

Exact match search

"rose"

AND

Requires both terms in results

rose AND red

OR

Includes either term in results

rose OR red

NOT

Excludes results containing a specific term

"bouquet of roses" AND NOT red

exists

Ensures a field contains information

emailDomain=example.com AND exists=accessURL

()

Groups terms within parameters

emailDomain=(example1.com OR example2.com)

\~

Approximate search (supports values 1 and 2\)

ormus\~1

\*

Replaces zero or more characters in search patterns

ormus\* (returns results like "ormuspay")

\*

Identifies that a field contains any information

cvv=\*

?

Replaces a single character in search patterns

l?g?n (returns results like login, logon, l0gin, l0g1n)

   
Make sure to use parentheses when applying operators in your searches to ensure the correct precedence of operations. This will help refine the results and avoid unintended interpretations of the terms.

 

Use case examples (recommended)

1\. (emailDomain=example1.com OR emailDomain=example2.com) AND detectionDate\>=2025-01-01

 

Use case examples (not recommended)

1\. emailDomain=example1.com OR emailDomain=example2.com AND detectionDate\>=2025-01-01

What are the credential search parameters of Threat Hunting?  
Updated over a week ago  
In this article, you'll find the credential search parameters, along with usage examples for each. If you have any questions, please contact our support team.

 

Attention: Credentials containing passwords with fewer than 4 digits are automatically disregarded and are not registered as detections on the Axur platform.

How to search for credential information?  
Parameter

Description

Example

leakFormat

Format in which the leak was found

leakFormat="COMBOLIST"  
leakFormat="STEALER LOG"  
leakFormat="TABLE"

detectionDate

Date when the credential was detected

detectionDate=YYYY-MM-DD

detectionDate\>=YYYY-MM-DDThh:mm

detectionDate\<=YYYY-MM-DDThh:mm:ss

detectionDate\>=YYYY-MM-DD AND detectionDate\<=YYYY-MM-DD

user

User associated with the credential

user=user@example.com

user=(user1@example.com OR user2@example.com)

emailDomain

Email domain

emailDomain=example.com

emailDomain=(example1.com OR example2.com)

emailHost

Email host

emailHost=support.example.com

emailHost=(support.example.com OR mkt.example.com OR ti.example.com)

emailTld

Top-level domain (TLD) of the email

emailTld=com

userType

Type of user

userType=EMAIL

 

userType=PHONE

 

userType=USERNAME 

password

Password associated with the credential

password=123456789

passwordHasLetter

Indicates if the password contains letters

passwordHasLetter=true

passwordHasLetter=false

passwordHasLowerCase

Indicates if the password contains lowercase letters

passwordHasLowerCase=true

passwordHasLowerCase=false

passwordHasUpperCase

Indicates if the password contains uppercase letters

passwordHasUpperCase=true

passwordHasUpperCase=false

passwordHasNumber

Indicates if the password contains numbers

passwordHasNumber=true

passwordHasNumber=false

passwordHasSpecialChar

Indicates if the password contains special characters

passwordHasSpecialChar=true

passwordHasSpecialChar=false

passwordLength

Length of the password in characters

passwordLength=8

passwordLength\>=12

passwordType

Type of stored password

passwordType=BCRYPT

 

passwordType=MD5

 

passwordType=MYSQL323

 

passwordType=PBKDF2

 

passwordType=PLAIN

 

passwordType=SHA1

 

passwordType=SHA256

 

passwordType=SHA384

 

passwordType=SHA512 

   
How to search for access information?  
 

Parameter

Description

Example

accessDomain

Domain associated with the credential

accessDomain=example.com

accessHost

Specific access host

accessHost=login.example.com

accessTld

Top-level domain (TLD) of access

accessTld=com

accessUrl

Access URL of the credential

accessUrl="https://login.example.com/app"

 

accessUrl=example.com

accessAppId

App ID for access (Google Play IDs only)

accessAppId="br.com.example.app"

   
How to search for leak source information?  
 

Parameter

Description

Example

sourceName

Name of the source where the credential was exposed

sourceName=Deep/Dark Web

 

sourceName=Deep/Dark Web \- Telegram

 

sourceName=Deep/Dark Web \- WhatsApp

 

sourceName=IntelX

 

sourceName=mega

 

sourceName=paste.ee

 

sourceName=pastebin

sourceUrl

URL of the source where the credential was found

sourceUrl=example.com

 

sourceUrl=breachforums.st

 

sourceUrl=leakbase.io

 

sourceUrl=\*

   
How to search for file information?  
 

Parameter

Description

Usage Examples

fileName

Name of the file

fileName="filename.txt"

filePath

Path of the file

filePath="50 000 000 Link Login Pass 10.rar"  
What are the credit card search parameters of Threat Hunting?  
Updated over a week ago  
In this article, you'll find the credit card search parameters, along with usage examples for each. If you have any questions, please contact our support team.

 

How to search for credit card information?  
 

Parameter

Description

Example

detectionDate

Date when the card was detected

detectionDate=YYYY-MM-DD

detectionDate\>=YYYY-MM-DDThh:mm

detectionDate\<=YYYY-MM-DDThh:mm:ss

detectionDate\>=YYYY-MM-DD AND detectionDate\<=YYYY-MM-DD

cardNumber

Full card number

cardNumber="1234567891234567"

cardNumber="1234567891234567" AND cvv=192

cardNumber

Partial card number

cardNumber=\*5590

cardNumber=5364\* AND cardNumber=\*5590

cardNumberHash

search by SHA256 hash

cardNumberHash=ff989800f2d1525ed3d7febbef7c0360ec0d8e413ac84ffc20d8c1ca99f9942e

 

ff989800f2d1525ed3d7febbef7c0360ec0d8e413ac84ffc20d8c1ca99f9942e

bin

BIN number (first 6 digits)

bin=123456

bin=(123456 OR 654321 OR 789987\)

bin8

BIN8 number (first 8 digits)

bin8=12345678

cvv

Card security code (CVV)

cvv=123

expirationMonth

Card expiration month

expirationMonth=10

expirationYear

Card expiration year

expirationYear=24

holder

Cardholder's name

holder=carlos

   
How to search for source information?  
 

Parameter

Description

Example

sourceName

Name of the source where the information was published

sourceName=Deep/Dark Web

 

sourceName=Deep/Dark Web \- Telegram

 

sourceName=Deep/Dark Web \- WhatsApp

 

sourceName=IntelX

 

sourceName=mega

 

sourceName=paste.ee

 

sourceName=pastebin 

sourceUrl

URL of the source

sourceUrl=example.com

   
How to search for file information?  
 

Parameter

Description

Example

fileName

Name of the file

fileName="filename.txt"

filePath

File path

filePath="50 000 000 Link Login Pass 10.rar"  
What are the Infected Machine search parameters of Threat Hunting?  
Updated over a week ago  
This segment will be available soon\!

In this article, you'll find the Infected Machine search parameters, along with usage examples for each. If you have any questions, please contact our support team.

 

How to search for infected machine information?  
 

Parameter

Description

Example

user

Infected machine's user

user=person-1

user=(person-1 OR person2)

ip

Infected machine's IP address

ip=111.101.100.32

malwareLocation

Malware location

malwareLocation="C:\\Windows"

operationalSystem

Infected machine's operating system

operationalSystem=windows

hardware

Infected machine's hardware

hardware=intel

country

Infected machine's country

country=US

computerTimezone

Infected machine's time zone

computerTimezone=UTC-03:00

sourceName

Exposure source name

sourceName=Deep/Dark Web

 

sourceName=Deep/Dark Web \- Telegram

 

sourceName=Deep/Dark Web \- WhatsApp

 

sourceName=IntelX

 

sourceName=mega

 

sourceName=paste.ee

 

sourceName=pastebin 

zipcode

Infected machine's postal code

zipcode=88310

computerLanguage

Infected machine's language

computerLanguage="pt-BR"

displayHeight

Infected machine's display height

displayHeight=1080

displayWidth

Infected machine's display width

displayWidth=1920

fileName

File name

fileName="filename.txt"

filePath

File path

filePath="NEW WLFR CLOUD"

What are the URL & Domains search parameters of Threat Hunting?  
Updated over a week ago  
In this article, you'll find the URL & Domains search parameters, along with usage examples for each. If you have any questions, please contact our support team.

Additionally, we provide multiple search parameters to help you refine your results when searching for URLs and domains. All of these parameters are available when you access the result details. Below, we have organized the search parameters into a hierarchy to optimize your analysis.

1\. Primary URL & Domain Identification  
These fields are essential for identifying and classifying URLs and domains:

 

Parameter

Description

Example

reference

Full reference of the URL or domain

reference="https://phishing-site.com/login"

domain

Registered domain of the reference

domain="malicious-example.com"

domainCreationDate

Domain creation date (useful for identifying newly created domains)

domainCreationDate\>2025-01-16

origin

Origin of the reference

origin=phishtank OR origin=urlscan

host

Host related to the reference

host="secure-bank.example.com"

subdomain

Subdomain associated with the reference

subdomain="login"

   
2\. Threat & Phishing Indicators  
Parameter

Description

Example

contentType

 

Type of Content

 

contentType="e-commerce","parked domain", "financial", "news", "social media", "forum", "message app", "error page", "blank page", "login page", "adult", "gambling", "games", "captcha", "under construction", "other"

impersonatedBrand

Targeted brand impersonation

impersonatedBrand="Netflix" OR impersonatedBrand="Facebook"

impersonatedBrandsHigh, impersonatedBrandsMedium, impersonatedBrandsLow

Brand impersonation level

impersonatedBrandsHigh="Apple" OR impersonatedBrandsMedium="Microsoft"

companiesMentioned

Companies mentioned in the HTML or screenshot

companiesMentioned="Amazon" OR companiesMentioned="Tesla"

companyLogo

Detected company logos

companyLogo="paypal" OR companyLogo="visa"

languages

Languages present in the content

languages="english" OR languages="spanish" OR languages="french" OR languages="portuguese"

predominantLanguage

Predominant language in the content

predominantLanguage="english" OR predominantLanguage="spanish"

imageDescription

Image description (Available in English)

imageDescription="yellow background" AND imageDescription="casino"

predominantColor

Predominant color in text format

predominantColorHex="orange"

predominantColorHex

Predominant color in hexadecimal format

predominantColorHex="\#FE3131"

predominantColorRGB

Predominant color in RGB

predominantColorRGB="\[254, 49, 49\]"

contentHTML

Search by textual content of the page

htmlContent="99999-9999"

htmlLinks

Search by links contained on the page

htmlLinks="wa.me"

   
3\. Technical URL Analysis  
Parameter

Description

Example

referenceIp

IP associated with the reference

referenceIp="45.67.89.101"

protocol

URL protocol

protocol="http" OR protocol="ftp"

queryStrings

URL query parameters

queryStrings="?sessionid=abcd1234"

httpStatus

HTTP response status code

httpStatus=404 OR httpStatus=503

redirectedTo

URL where redirection occurred

redirectedTo="https://secure-payment.xyz"

finalUrl

Final URL after redirections

finalUrl="https://final-malicious-site.com"

   
4\. WHOIS Data (Domain Registration)  
Parameter

Description

Example

domainStatus

Current domain status

domainStatus="suspended"

registrant, registrantOrganization, registrantEmail

Registrant information

registrant="Anonymous" OR registrantEmail="fake-registrar@mail.com"

administrator, administratorOrganization, administratorEmail

Domain administrator information

administrator="John Doe" OR administratorEmail="admin@example.net"

technical, technicalOrganization, technicalEmail

Technical contact information

technical="Tech Support" OR technicalEmail="tech@domain.com"

registrar, registrarEmail

Domain registrar company

registrar="Namecheap" OR registrarEmail="registrar@mail.com"

nameServers

List of name servers

nameServers="ns1.fake-dns.com" OR nameServers="ns2.fake-dns.com"

ipAddresses

IP addresses associated with name servers

ipAddresses="185.199.108.153"

   
5\. DNS Records & Infrastructure  
Parameter

Description

Example

dnsRecordType (A)

IPv4 address record for the domain

dnsRecordType="A" AND dnsRecordValue="192.0.2.1"

dnsRecordType (AAAA)

IPv6 address record for the domain

dnsRecordType="AAAA" AND dnsRecordValue="2001:db8::1"

dnsRecordType (CNAME)

Canonical name record (alias for another domain)

dnsRecordType="CNAME" AND dnsRecordValue="example.com"

dnsRecordType (NS)

Name server record, indicating authoritative DNS servers

dnsRecordType="NS" AND dnsRecordValue="ns1.example.com"

dnsRecordType (MX)

Mail exchange record, specifying mail servers for the domain

dnsRecordType="MX" AND dnsRecordValue="mail.example.com"

ipAddresses

IP addresses associated with name servers

ipAddresses="185.199.108.153"

   
6\. How to search for Geolocation information?  
Parameter

Description

Example

geolocationCountryName

Indicates the country where the server is located.

geolocationCountryName="United States"

geolocationCountryCode

Indicates the country code where the server is located.

geolocationCountryCode="CN"

geolocationIp

Geolocation IP

geolocationIp \= "203.0.113.45"

latitude

Represents the latitude coordinate of the estimated IP location.

latitude="54.6876"

longitude

Represents the longitude coordinate of the estimated IP location.

 

longitude="25.2806"

isp

Server related to the reference.

isp=Cloudflare

   
7\. How can I search for signal data from the 'facebook-ads-coll' source in the Meta Ad Library?  
Parameter

Description

Example

metaAdId

 

Ad ID

 

 

metaAdId=24286785267571234

 

metaAdUrl

 

 

Ad URL

 

metaAdUrl=https://www.facebook.com/ads/library?id=24286785267571234

 

metaAdvertiserProfiles

 

Advertiser profile

metaAdvertiserProfiles=https://facebook.com/discount-amazon-store

 

metaProfileId

 

Profile ID

metaProfileId=123489169657887

 

metaProfileName

 

Profile Name

metaProfileName="Discount Store"

Hunting Like a Pro: Searches for Similar Domain Names  
Updated over a week ago  
In today's digital threat landscape, protecting your brand and your customers is essential. Axur’s Intelligent Monitoring of Similar Domains enables the detection and takedown of fraudulent domains that attempt to impersonate your company. But how can you perform even more effective and targeted searches to protect your brand?

This article will guide you through four levels of detection, from the most restrictive to the most comprehensive, so you can search like a true professional in Threat Hunting for fraudulent similar domains.

Level 1: Exact Brand Match Domains (Multiple TLDs)  
This is the most restrictive level, detecting only domains that exactly match your brand name but use different Top-Level Domains (TLDs). It's ideal for companies looking to catch the most obvious impersonations.

Examples of detection:

netflix.cn

netflix.co

netflix.com.mx

netflix.online

Query example:

​domainLabel=netflix AND referenceType=DOMAIN

Level 2: Exact Match in Domain Names and Hosts  
At this level, the search expands to include exact matches in both domain names and hostnames. This helps identify subdomains or services trying to mimic your brand.

 

Examples of detection:

netflix.com

netflix.github.io

netflix.co

netflix.com.br

Query example:

​(domainLabel=netflix OR subdomain=netflix) AND referenceType=DOMAIN

Level 3: Domain and Host Match with Typos and Homoglyphs  
Moving up a level, this search includes detection of domains and hosts that use typosquatting (misspellings) and homoglyphs (visually similar characters that deceive users). This is critical for capturing more sophisticated fraud attempts.

Examples of detection:

netflíx.com

nettflix.github.io

netfllx.co

online.netflīx.com

Query example:

​(domainLabel=netflix\~1 OR sanitizedDomainLabel=netflix OR subdomain=netflix OR sanitizedSubdomain=netflix) AND referenceType=DOMAIN

Level 4: Comprehensive Match with Typos and Homoglyphs in Any Position  
The most comprehensive level. Here, the search combines typosquatting and homoglyph detection with the ability to find your brand at the beginning, middle, or end of any word used in the domain or host name. Ideal for maximum fraud protection.

Examples of detection:

newnetflix.github.io

netflis.co

ѕuрроrt-netflix.com

nētflixlogin.com

Query example:

​(domainLabel=\*netflix\* OR subdomain=\*netflix\* OR domainLabel=netflix\~1 OR subdomain=netflix\~1 OR sanitizedDomainLabel=\*netflix\* OR sanitizedSubdomain=netflix) AND referenceType=DOMAIN

Need to adjust your rules?  
Keep in mind that the ideal detection level depends on your risk profile and available resources. Start with a more conservative level and broaden your scope as needed, monitoring the results and refining your queries to optimize cost-effectiveness and reduce false positives.

What are the search parameters for Social Media Profiles in Threat Hunting?  
Updated this week  
In this section, you will find the complete list of search parameters for profiles on the monitored platforms (Facebook, Instagram, TikTok, and WhatsApp). These filters allow you to identify accounts, analyze technical metadata, and cross-reference information for Threat Hunting investigations.

Important tip: Use the parameters in the “General” section for scans that cover all social networks simultaneously. For in-depth investigations on a specific platform, use the dedicated sections (e.g., TikTok or Facebook) to access granular metadata.

1\. General Identification (Multi-platform)  
These attributes are normalized and work across most platforms, making them ideal for broad searches.

Parameter

Description

Example

socialMedia

Social network where the data was collected

socialMedia=tiktok

profileId

Unique profile ID on the platform (general search)

profileId="123456789"

username

Profile username/handle

username="store\_sales"

name

Profile display name

name="official support"

followers

Total number of followers

followers\>1000

following

Number of accounts the profile follows

following\<5

description

Profile biography or description text

description="Promotion"

verified

Indicates whether the profile has a verification badge

verified=false

verified=true

profileImageDescription

Automatic visual description of the profile picture

profileImageDescription="blue logo"

coverImageDescription

Automatic visual description of the cover image

coverImageDescription="smiling people"

   
2\. AI Enrichment and Analysis  
Parameters generated through artificial intelligence analysis, useful for detecting fraud, brand impersonation, and visual patterns.

Parameter

Description

Example

impersonatedBrandsHigh

Impersonated brand (High confidence)

impersonatedBrandsHigh="Netflix"

impersonatedBrandsMedium

Impersonated brand (Medium confidence)

impersonatedBrandsMedium="Bank X"

impersonatedBrandsLow

Impersonated brand (Low confidence)

impersonatedBrandsLow="Store Y"

companyLogo

Company logo detected in the image

companyLogo="Brand LogoZ"

companiesMentioned

Company names mentioned in the profile text

companiesMentioned="Magazine"

predominantLanguage

Predominant language of the profile

predominantLanguage="english"

contentType

Content category (e.g., Finance, Retail)

contentType="Financial Services"

predominantColor

Predominant color (text)

predominantColor="red"

predominantColorHex

Predominant color (Hexadecimal)

predominantColorHex="\#FE3131"

predominantColorRGB

Predominant color (RGB)

predominantColorRGB="\[254, 49, 49\]"

presenceOfPeopleProfileImage

Person detection in the profile picture

presenceOfPeopleProfileImage=true

presenceOfPeopleCoverImage

Person detection in the cover image

presenceOfPeopleCoverImage=false

   
3\. Platform-Specific Parameters: Facebook  
Technical and location-based attributes exclusive to Facebook profiles and pages.

Parameter

Description

Example

facebookProfileId

Facebook-specific profile ID

facebookProfileId="10000..."

facebookUsername

Facebook username

facebookUsername="official.page"

facebookName

Page/profile name

facebookName="Offers USA"

facebookSearchType

Collection type (People or Pages)

facebookSearchType="Pages"

facebookSearchType="People"

facebookDescription

Page description

facebookDescription="brand store"

facebookAbout

"About” information of the page

facebookAbout="since 2010..."

facebookCategory

Page category

facebookCategory="Shopping"

facebookWebsite

Website linked to the page

facebookWebsite="fake-site.com"

facebookFollowers

Number of followers

facebookFollowers\<100

facebookFollowersCount

Technical follower count

facebookFollowersCount=50

facebookFanCount

Number of fans

facebookFanCount=10

facebookVerified

Page verification status

facebookVerified=true

facebookUnclaimed

Indicates whether the page has no confirmed owner

facebookUnclaimed=true

facebookGlobalBrandName

Associated global brand name

facebookGlobalBrandName="BrandX"

facebookWhatsappNumber

Linked WhatsApp number

facebookWhatsappNumber="+55..."

facebookLocation

Location label

facebookLocation exists

facebookCity

Page city

facebookCity="New York"

facebookCountry

Page country

facebookCountry="Mexico"

facebookStreet

Address (Street)

facebookStreet="Av. Paulista"

facebookZip

Postal code (ZIP code)

facebookZip="01310-100"

facebookLatitude

Location latitude

facebookLatitude="-23.55"

facebookLongitude

Location longitude

facebookLongitude="-46.63"

facebookMetadataOrigin

Metadata source

facebookMetadataOrigin="API"

facebookPictureSilhouette

Indicates whether the photo is a default silhouette

facebookPictureSilhouette=true

   
4\. Platform-Specific Parameters: Instagram  
Fields focused on biography, media metrics, and Instagram links.

Parameter

Description

Example

instagramProfileId

Unique identifier of the Instagram profile

instagramProfileId=987654

instagramUsername

Profile username/handle

instagramUsername="insta\_user"

instagramName

Profile display name

instagramName="User Name"

instagramBio

Text shown in the profile biography

instagramBio="follow the link"

instagramBioLinks

Clickable links available in the bio

instagramBioLinks="bit.ly/promo"

instagramExternalUrl

Main external URL linked to the profile

instagramExternalUrl="site.com"

instagramFollowers

Total number of followers

instagramFollowers\>5000

instagramFollowing

Number of accounts the profile follows

instagramFollowing\<100

instagramNumPost

Total number of posts on the account

instagramNumPost=50

instagramVerified

Indicates whether the account has a blue verification badge

instagramVerified=false

instagramPrivate

Indicates whether the account is private

instagramPrivate=true

   
5\. Platform-Specific Parameters: TikTok  
Detailed engagement metrics, privacy settings, and account status on TikTok.

Parameter

Description

Example

tiktokProfileId

Unique identifier of the TikTok profile

tiktokProfileId="67890..."

tiktokUsername

TikTok page username

tiktokUsername="tiktok\_user"

tiktokName

TikTok page display name

tiktokName="Creator"

tiktokDescription

Description of the TikTok page

tiktokDescription="Videos diários"

tiktokFollowers

Total number of followers

tiktokFollowers\>100

tiktokFollowing

Number of accounts the account follows

tiktokFollowing=10

tiktokHeartCount

Total number of hearts/likes (alternative metric)

tiktokHeartCount="5000"

tiktokVideoCount

Total number of videos posted

tiktokVideoCount=15

tiktokVerified

Indicates whether the account is verified

tiktokVerified=true

tiktokPrivate

Indicates whether the account is private

tiktokPrivate=false

   
6\. Platform-Specific Parameters: WhatsApp  
Identification data for WhatsApp Business or Personal accounts.

Parameter

Description

Example

whatsappProfileId

Unique identifier of the WhatsApp profile

whatsappProfileId="55119..."

whatsappName

Username of the account owner

whatsappName="Loja Atendimento"

whatsappNumber

Phone number of the account owner

whatsappNumber="+5511988887777"

whatsappDescription

Description of the WhatsApp profile

whatsappDescription="Horário 9h-18h"

whatsappStatus

Status or message set by the account owner

whatsappStatus="Disponível"

whatsappWebsite

Website linked to the account owner

whatsappWebsite="loja.com.br"

   
7\. Ready-to-use Query Examples  
You can combine the parameters above to create advanced filters on the platform:

Identify underage TikTok profiles selling products:  
​tiktokUnderAge18=TRUE AND tiktokSeller=TRUE

Search for Facebook pages without a confirmed owner in São Paulo:  
​facebookUnclaimed=TRUE AND facebookCity="São Paulo"

Find Instagram profiles with suspicious links in the bio:  
​instagramBioLinks="bit.ly/oferta-falsa" AND verified=FALSE

Brand monitoring (Brand Protection) across all social networks:  
​impersonatedBrandsHigh="MinhaMarca" AND verified=FALSE